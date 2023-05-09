use std::{io::{BufReader, Read, Error, Seek, SeekFrom}, fs::File, mem::transmute};

pub struct BinaryReader {
    reader: BufReader<File>,
}

impl BinaryReader {
    pub fn new(file: File) -> Self {
        let reader = BufReader::new(file);

        Self {
            reader
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let mut byte = 0;
        self.reader.read(std::slice::from_mut(&mut byte))?;
        Ok(byte)
    }

    pub fn read_i8(&mut self) -> Result<i8, Error> {
        let mut byte = 0;
        self.reader.read(std::slice::from_mut(&mut byte))?;
        Ok(unsafe { transmute(byte) })
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut bytes = [0; 2];
        self.reader.read(&mut bytes)?;

        Ok(bytes[0] as u16 | ((bytes[1] as u16) << 8))
    }

    pub fn read_i16(&mut self) -> Result<i16, Error> {
        let mut bytes = [0; 2];
        self.reader.read(&mut bytes)?;

        Ok(bytes[0] as i16 | ((bytes[1] as i16) << 8))
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let mut bytes = [0; 4];
        self.reader.read(&mut bytes)?;

        Ok(bytes[0] as u32 | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16) | ((bytes[3] as u32) << 24))
    }

    pub fn read_i32(&mut self) -> Result<i32, Error> {
        let mut bytes = [0; 4];
        self.reader.read(&mut bytes)?;

        Ok(bytes[0] as i32 | ((bytes[1] as i32) << 8) | ((bytes[2] as i32) << 16) | ((bytes[3] as i32) << 24))
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let mut bytes = [0; 8];
        self.reader.read(&mut bytes)?;

        Ok(bytes[0] as u64 | ((bytes[1] as u64) << 8) | ((bytes[2] as u64) << 16) | ((bytes[3] as u64) << 24) | ((bytes[4] as u64) << 32) | ((bytes[5] as u64) << 40) | ((bytes[6] as u64) << 48) | ((bytes[7] as u64) << 56))
    }

    pub fn read_i64(&mut self) -> Result<i64, Error> {
        let mut bytes = [0; 8];
        self.reader.read(&mut bytes)?;

        Ok(bytes[0] as i64 | ((bytes[1] as i64) << 8) | ((bytes[2] as i64) << 16) | ((bytes[3] as i64) << 24) | ((bytes[4] as i64) << 32) | ((bytes[5] as i64) << 40) | ((bytes[6] as i64) << 48) | ((bytes[7] as i64) << 56))
    }

    pub fn read_f32(&mut self) -> Result<f32, Error> {
        let i = self.read_i32()?;

        Ok(unsafe { transmute(i) })
    }

    pub fn read_f64(&mut self) -> Result<f64, Error> {
        let i = self.read_i64()?;

        Ok(unsafe { transmute(i) })
    }

    pub fn read_to_buf(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.reader.read(buf)
    }

    pub fn read_bytes(&mut self, num_bytes: usize) -> Result<Vec<u8>, Error> {
        let mut vec: Vec<u8> = std::iter::repeat(0).take(num_bytes).collect();
        self.reader.read(&mut vec)?;

        Ok(vec)
    }

    pub fn position(&mut self) -> Result<usize, Error> {
        let pos = self.reader.stream_position()?;

        Ok(pos as usize)
    }

    pub fn set_position(&mut self, position: usize) -> Result<(), Error> {
        let curr_pos = self.reader.stream_position()?;
        self.reader.seek(SeekFrom::Current(position as i64 - curr_pos as i64))?;

        Ok(())
    }
}