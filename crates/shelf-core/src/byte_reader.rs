#[derive(Debug, PartialEq)]
pub struct OutOfBytes;

pub struct ByteReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ByteReader { bytes, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn seek(&mut self, position: usize) {
        self.position = position;
    }

    pub fn skip(&mut self, count: usize) -> Result<(), OutOfBytes> {
        self.take(count).map(|_| ())
    }

    pub fn read_u8(&mut self) -> Result<u8, OutOfBytes> {
        Ok(self.read_array::<1>()?[0])
    }

    pub fn read_u32(&mut self) -> Result<u32, OutOfBytes> {
        self.read_array().map(u32::from_le_bytes)
    }

    pub fn read_i32(&mut self) -> Result<i32, OutOfBytes> {
        self.read_array().map(i32::from_le_bytes)
    }

    pub fn read_f32(&mut self) -> Result<f32, OutOfBytes> {
        self.read_array().map(f32::from_le_bytes)
    }

    pub fn read_u64(&mut self) -> Result<u64, OutOfBytes> {
        self.read_array().map(u64::from_le_bytes)
    }

    pub fn read_i64(&mut self) -> Result<i64, OutOfBytes> {
        self.read_array().map(i64::from_le_bytes)
    }

    pub fn read_cstring(&mut self) -> Result<String, OutOfBytes> {
        let rest = self.bytes.get(self.position..).ok_or(OutOfBytes)?;
        let length = rest.iter().position(|&byte| byte == 0).ok_or(OutOfBytes)?;
        self.position += length + 1;
        Ok(String::from_utf8_lossy(&rest[..length]).into_owned())
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], OutOfBytes> {
        let end = self.position.checked_add(count).ok_or(OutOfBytes)?;
        let slice = self.bytes.get(self.position..end).ok_or(OutOfBytes)?;
        self.position = end;
        Ok(slice)
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], OutOfBytes> {
        self.take(N)?.try_into().map_err(|_| OutOfBytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_little_endian_and_strings() {
        let bytes = [1, 0, 0, 0, b'h', b'i', 0];
        let mut reader = ByteReader::new(&bytes);
        assert_eq!(reader.read_u32(), Ok(1));
        assert_eq!(reader.read_cstring(), Ok("hi".to_string()));
        assert_eq!(reader.read_u8(), Err(OutOfBytes));
    }
}
