use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn es2_1() -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("test_file")?;

    // here it is possible to see how the encoding changes if a not UTF-8 letter is present
    // it is encoded in multiple bytes

    // let mut byte_content: Vec<u8> = Vec::new();
    // file.read_to_end(&mut byte_content)?;
    //
    // println!("{:?}", byte_content);
    // println!("{:02x?}", byte_content);

    let mut str_content = String::new();
    file.read_to_string(&mut str_content)?;
    println!("{str_content}");

    let mut new_content = String::new();
    for _ in 0..10 {
        new_content.push_str(&str_content)
    }

    file.write_all(new_content.as_bytes())?;
    println!("{new_content}");

    Ok(())
}