use std::fs::File;

#[test]
fn test_read() {
    let file = File::open("/home/ollie/Music/Kalimba.wav").unwrap();

    let mut reader = skyetils::binary::BinaryReader::new(file);

    println!("{}, {}, {}, {}", reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char);

    println!("{}", reader.read_u32().unwrap());

    println!("{}, {}, {}, {}", reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char);
    println!("{}, {}, {}, {}", reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char, reader.read_u8().unwrap() as char);
}