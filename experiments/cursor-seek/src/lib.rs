#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek, SeekFrom, Write};

    #[test]
    fn main() {
        let mut buf: Vec<u8> = vec![0; 32];
        let mut cursor = Cursor::new(&mut *buf);

        cursor.write_all(b"hello").unwrap();
        cursor.write_all(b"_").unwrap();
        cursor.write_all(b"world").unwrap();

        assert_eq!(&cursor.get_ref()[..12], b"hello_world\x00");

        let pos = cursor.position();
        cursor.seek(SeekFrom::Start(5)).unwrap();
        cursor.write_all(b" ").unwrap();
        cursor.seek(SeekFrom::Start(pos)).unwrap();
        cursor.write_all(b"!").unwrap();

        assert_eq!(&cursor.get_ref()[..13], b"hello world!\x00");
    }
}
