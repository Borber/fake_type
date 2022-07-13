use std::{io::{self, Seek, SeekFrom, BufReader, Read}, fs::{File, OpenOptions}, path::Path, fs};
use std::os::windows::fs::FileExt;
use std::path::PathBuf;

pub struct Type {
    pub suffix: String,
    pub bytes: Vec<u8>
}

/// 字符串“Borber”的 blake3 哈希值，用于标记伪造文件
/// The blake3 hash value of string "Borber", use to flag faked file
pub const FLAG: &[u8; 64] = b"67ea716879a2881181afb79f9737553ae96ed1d65119551ab416957a01ff0f58";


/// 改变文件类型
/// Change file to fake type
///
/// # 用例
/// # Examples
/// ```
/// use std::path::Path;
/// use fake_type::Type;
///
/// let path = Path::new(r"file/test.txt");
/// let flag = fake_type::check_fake(path, fake_type::FLAG)?;
/// println!("Is it a fake file:{:?}",flag);
/// if !flag {
///     fake_type::fake(path, fake_type::FLAG, Type {bytes: vec![],suffix: "".to_string()})?;
/// }
/// ```
///
pub fn fake(path: &Path, flag: &[u8], f_type: Type) -> Result<(), io::Error> {
    let mut suffix = path.extension().unwrap().to_str().unwrap().as_bytes().to_vec();
    // 填充占位
    // Fill the placeholder
    while suffix.len() < 16 {
        suffix.push(u8::try_from('#').unwrap());
    }
    // 防止后缀名过长
    // Prevent suffix name from being too long
    while suffix.len() > 16 {
        suffix.pop();
    }
    let mut head = [0u8; 48]; // 原文件头及冗余信息
    let mut f_reader = BufReader::new(File::open(path).expect("打开文件失败"));
    let f_writer = OpenOptions::new().write(true).open(path).expect("以可写方式打开文件失败");
    f_reader.read_exact(&mut head)?;
    f_writer.seek_write(f_type.bytes.as_slice(), 0).expect("写入失败");
    let offset = f_reader.seek(SeekFrom::End(0))?;
    let data = [flag, suffix.as_slice(), &head].concat();
    f_writer.seek_write(data.as_slice(), offset)?;
    let mut new_path = PathBuf::from(path);
    new_path.set_extension(f_type.suffix);
    fs::rename(path.as_os_str(), new_path.as_os_str())?;
    Ok(())
}

/// 将fake文件还原为原始文件
/// Restore the fake file to the original file
///
/// # 用例
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let path = Path::new(r"file/test.txt");
///     let flag = fake_type::check_fake(path,  fake_type::FLAG)?;
///     println!("Is it a fake file:{:?}",flag);
///     if flag {
///         fake_type::restore(path)?;
///     }
/// ```
///
pub fn restore(path: &Path) -> Result<(), io::Error> {
    let mut f_reader = File::open(path)?;
    let offset = f_reader.seek(SeekFrom::End(-64))?;
    let mut data = [0u8; 64];
    f_reader.seek_read(data.as_mut_slice(), offset)?;
    let (suffix, head) = data.split_at(16);
    let suffix = std::str::from_utf8(suffix).unwrap().replace('#', "");
    let f_writer = OpenOptions::new().write(true).open(path)?;
    f_writer.seek_write(head, 0)?;
    f_writer.set_len(offset - 64)?;
    let mut new_path = PathBuf::from(path);
    new_path.set_extension(suffix);
    fs::rename(path.as_os_str(), new_path.as_os_str())?;
    Ok(())
}

/// 检查是否为fake文件
/// Check if it is a fake file
///
/// # 用例
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let path = Path::new(r"file/test.txt");
/// let flag = fake_type::check_fake(path, fake_type::FLAG)?;
/// println!("Is it a fake file : {:?}",flag);
///
/// ```
///
pub fn check(path: &Path, flag: &[u8]) -> Result<bool, io::Error> {
    let mut f_reader = File::open(path)?;
    if f_reader.seek(SeekFrom::End(0))? < 128 {
        return Ok(false);
    }
    let offset = f_reader.seek(SeekFrom::End(-128))?;
    let mut data = [0u8; 64];
    f_reader.seek_read(&mut data, offset)?;
    Ok(data.eq(flag))
}