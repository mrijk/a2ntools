use std::io::{self, Read};
use std::str;
use utf16string::WStr;

use byteorder::{BigEndian, ReadBytesExt};

pub fn read_u8(reader: &mut dyn Read) -> u8 {
    reader.read_u8().unwrap()
}

pub fn read_bool(reader: &mut dyn Read) -> bool {
    match read_u8(reader) {
        0 => false,
        1 => true,
        _ => panic!("Oh no!")
    }
}

pub fn read_u16(reader: &mut dyn Read) -> u16 {
    reader.read_u16::<BigEndian>().unwrap()
}

pub fn read_u32(reader: &mut dyn Read) -> u32 {
    reader.read_u32::<BigEndian>().unwrap()
}

pub fn read_four_byte_string(reader: &mut dyn Read) -> io::Result<String> {
    let mut buffer = Vec::new();
    reader.take(4 as u64).read_to_end(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

pub fn read_string(reader: &mut dyn Read) -> io::Result<String> {
    let len = read_u32(reader);

    let mut buffer = Vec::new();
    reader.take(len as u64).read_to_end(&mut buffer)?;

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

pub fn read_token_or_string(reader: &mut dyn Read) -> io::Result<String> {
    let len = read_u32(reader);
    if len != 0 {
        read_unicode_string_with_len(reader, len - 1)
    } else {
        read_four_byte_string(reader)
    }
}


fn to_unicode_string(string: String) -> io::Result<String> {
    match WStr::from_utf16be(string.as_bytes()) {
        Ok(s) => Ok(s.to_utf8()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }    
}


fn read_unicode_string_with_len(reader: &mut dyn Read, len: u32) -> io::Result<String> {
    let mut buffer = Vec::new();
    reader.take(2 * len as u64).read_to_end(&mut buffer)?;

    let _ = read_u16(reader);

    match str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }.and_then(to_unicode_string)
}

pub fn read_unicode_string(reader: &mut dyn Read) -> io::Result<String> {
    let len = read_u32(reader) - 1;
    read_unicode_string_with_len(reader, len)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_bool() {
        let data: Vec<u8> = vec![0x00];
        let mut reader = Cursor::new(data);
        assert_eq!(read_bool(&mut reader), false);

        let data: Vec<u8> = vec![0x01];
        let mut reader = Cursor::new(data);
        assert_eq!(read_bool(&mut reader), true);
    }

    #[test]
    #[should_panic(expected = "Oh no!")]
    fn test_read_invalid_bool() {
        let data: Vec<u8> = vec![0x42];
        let mut reader = Cursor::new(data);
        read_bool(&mut reader);
    }

    #[test]
    fn test_read_u8() {
        let data: Vec<u8> = vec![0x2a];
        let mut reader = Cursor::new(data);
        assert_eq!(read_u8(&mut reader), 42);
    }

    #[test]
    fn test_read_u16() {
        let data: Vec<u8> = vec![0x00, 0x2a];
        let mut reader = Cursor::new(data);
        assert_eq!(read_u16(&mut reader), 42);
    }

    #[test]
    fn test_read_u32() {
        let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x2a];
        let mut reader = Cursor::new(data);
        assert_eq!(read_u32(&mut reader), 42);
    }

    #[test]
    fn test_read_string() {
        let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f];
        let mut reader = Cursor::new(data);
        assert_eq!(read_string(&mut reader).unwrap(), "Hello");
    }


    #[test]
    fn test_read_token_or_string_from_token() {
        let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x61, 0x62, 0x63, 0x64];
        let mut reader = Cursor::new(data);

        assert_eq!(read_token_or_string(&mut reader).unwrap(), "abcd");
    }

    #[test]
    fn test_read_token_or_string_from_string() {
        let data: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x06, // Length
            0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x00  // "Hello"
        ];
        let mut reader = Cursor::new(data);
        assert_eq!(read_token_or_string(&mut reader).unwrap(), "Hello");
    }

    #[test]
    fn test_read_unicode_string() {
        let data: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x06, // Length
            0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x00  // "Hello"
        ];
        let mut reader = Cursor::new(data);
        assert_eq!(read_unicode_string(&mut reader).unwrap(), "Hello");
    }

    #[test]
    fn test_read_unicode_string_with_len() {
        let data: Vec<u8> = vec![
            0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f, 0x00, 0x00  // "Hello"
        ];
        let mut reader = Cursor::new(data);
        assert_eq!(read_unicode_string_with_len(&mut reader, 5).unwrap(), "Hello");
    }
}