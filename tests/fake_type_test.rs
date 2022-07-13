use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use fake_type::{check, fake, restore, Type};


#[test]
fn fake_file_to_test() -> Result<(), io::Error>{
    let path = Path::new(r"file/test.txt");
    let flag = check(path, fake_type::FLAG)?;
    println!("Is it a fake file ? {:?}",flag);
    if !flag {
        fake(path, fake_type::FLAG, Type {
            suffix: "gz".to_string(),
            bytes: vec![31, 139, 8, 0]
        })?;
    }
    let path = Path::new(r"file/test.gz");
    let flag = check(path, fake_type::FLAG)?;
    println!("Is it a fake file ? {:?}",flag);
    Ok(())
}

#[test]
fn restore_file_to_test() -> Result<(), io::Error>{
    let path = Path::new(r"file/test.gz");
    let flag = check(path, fake_type::FLAG)?;
    println!("Is it a fake file ? {:?}",flag);
    if flag {
        restore(path)?;
    }
    let path = Path::new(r"file/test.txt");
    let flag = check(path, fake_type::FLAG)?;
    println!("Is it a fake file ? {:?}",flag);
    Ok(())
}

pub fn read_file(path: &Path) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}