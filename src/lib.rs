use std::{
    io::{self, Seek, SeekFrom, BufReader, Read},
    fs::{File, OpenOptions},
    path::Path
};
use std::io::ErrorKind;
use std::os::windows::fs::FileExt;

/// The blake3 hash value of string "Borber", use to flag faked file
const FAKE_FLAG: &[u8; 64] = b"67ea716879a2881181afb79f9737553ae96ed1d65119551ab416957a01ff0f58";
const FAKE_HEAD: &[u8; 4] = b"Fake";
const GZ: &[u8; 4] = &[31, 139, 8, 0];
const MP4: &[u8; 12] = &[0, 0, 0, 24, 102, 116, 121, 112, 51, 103, 112, 53];
const TXT: &[u8; 73] = b"The life and death of Gouli country, can it avoid misfortune and fortune?";


/// 改变文件类型
///
/// # 用例
/// 请看具体实现
///
/// Change file to fake type
///
/// # Examples
/// Please see the corresponding specific implementation
///
///
pub fn fake_type(path: &Path, fake_type: &str) -> Result<(), io::Error> {
    let fake_type_bytes =  match fake_type {
        "gz" => GZ.to_vec(),
        "mp4" => MP4.to_vec(),
        "txt" => TXT.to_vec(),
        _ => return Err(io::Error::new(ErrorKind::InvalidInput, "This type is not yet supported"))
    };
    let mut buf = vec![0u8; fake_type_bytes.len()];
    let buf = buf.as_mut_slice();
    let mut f_reader = BufReader::new(File::open(path)?);
    let f_writer = OpenOptions::new().write(true).open(path)?;
    f_reader.read_exact(buf)?;
    f_writer.seek_write(&fake_type_bytes, 0)?;
    let f_len = f_reader.seek(SeekFrom::End(0))?;
    let data = vec![0u8; 188 - fake_type_bytes.len()];
    let data = [FAKE_FLAG, data.as_slice(), FAKE_HEAD, buf].concat();
    let data = data.as_slice();
    f_writer.seek_write(data, f_len)?;
    Ok(())
}

/// 检查是否为fake文件
///
/// # 用例
///
/// ```
/// let path = Path::new("file/test.txt");
/// let flag = fake_type::check_fake(path)?;
/// println!("Is it a fake file:{:?}",flag);
///
/// ```
/// Check if it is a fake file
///
/// # Examples
///
/// ```
/// let path = Path::new("file/test.txt");
/// let flag = fake_type::check_fake(path)?;
/// println!("Is it a fake file:{:?}",flag);
///
/// ```
pub fn check_fake(path: &Path) -> Result<bool, io::Error> {
    let mut f_reader = File::open(path)?;
    if f_reader.seek(SeekFrom::End(0))? < 256 {
        return Ok(false);
    }
    let flag_location = f_reader.seek(SeekFrom::End(-256))?;
    let mut data = vec![0u8; 256];
    let data = data.as_mut_slice();
    f_reader.seek_read(data, flag_location)?;
    Ok(data.split_at(64).0 == FAKE_FLAG)
}

/// 将fake文件还原为原始文件
///
/// # 用例
///
/// Restore the fake file to the original file
/// # Examples
///

pub fn restore(path: &Path) -> Result<(), io::Error> {
    let mut f_reader = File::open(path)?;
    let flag_location = f_reader.seek(SeekFrom::End(-256))?;
    let mut data = vec![0u8; 256];
    let data = data.as_mut_slice();
    f_reader.seek_read(data, flag_location)?;
    let half = data.split_at(64).1;
    let mut index = half.len();
    while &half[index-4..index] != FAKE_HEAD {
      index -= 1;
    };
    let r_type = &half[index..half.len()];
    let f_writer = OpenOptions::new().write(true).open(path)?;
    f_writer.seek_write(r_type, 0)?;
    f_writer.set_len(flag_location)
}