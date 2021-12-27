use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use fake_type::{check_fake, fake_type, restore};

#[test]
fn how_to_be_txt() -> Result<(), io::Error>{
    let path = Path::new(r"C:\Users\BORBER\Downloads\Lark-win32_ia32-4.10.16-signed.txt");
    println!("Original:");
    read_file(path)?;
    println!("{:?}", b"txt:");
    Ok(())
}

#[test]
fn fake_file_to_test() -> Result<(), io::Error>{
    let path = Path::new(r"C:\Users\BORBER\Downloads\Lark-win32_ia32-4.10.16-signed.txt");
    let flag = check_fake(path)?;
    println!("Is it a fake file:{:?}",flag);
    if !flag {
        fake_type(path, "txt")?;
    }
    let flag = check_fake(path)?;
    println!("Is it a fake file:{:?}",flag);
    Ok(())
}

#[test]
fn restore_file_to_test() -> Result<(), io::Error>{
    let path = Path::new(r"C:\Users\BORBER\Downloads\Lark-win32_ia32-4.10.16-signed.txt");
    let flag = check_fake(path)?;
    println!("Is it a fake file:{:?}",flag);
    if flag {
        restore(path)?;
    }
    let flag = check_fake(path)?;
    println!("Is it a fake file:{:?}",flag);
    Ok(())
}

#[test]
fn fake_type_test() -> Result<(), io::Error> {
    // println!("{:?}", fake_type_to_gz(Path::new("file/test.tar.gz"))?);
    let path = Path::new("file/test.7z");
    println!("Original:");
    read_file(path)?;
    let flag = check_fake(path)?;
    println!("Is it a fake file:{:?}",flag);
    if !flag {
        fake_type(path, "mp4")?;
    }
    println!("Conversion:");
    read_file(path)?;
    let flag = check_fake(path)?;
    println!("Is it a fake file:{:?}",flag);
    if flag {
        restore(path)?;
    }
    println!("Restore:");
    read_file(path)
}

pub fn read_file(path: &Path) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}