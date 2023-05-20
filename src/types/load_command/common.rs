use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::fmt::Display;
use std::fmt::Debug;
use crate::reader::RcReader;
use crate::result::Result;

/// Represents `union lc_str`
pub struct LcStr {
    pub(crate) reader: RcReader,

    pub(crate) file_offset: u32,
}

impl LcStr {
    pub fn load_string(&self) -> Result<String> {
        let mut reader_mut = self.reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(self.file_offset as u64))?;

        reader_mut.read_zero_terminated_string()
    }
}

impl Debug for LcStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.load_string() {
            Ok(s) => s,
            Err(_) => "<Error>".to_string(),
        };
        write!(f, "{}", &s)
    }
}

impl Display for LcStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.load_string() {
            Ok(s) => s,
            Err(_) => "".to_string(),
        };
        write!(f, "{}", &s)
    }
}

pub struct BitVec {
    pub(super) reader: RcReader,

    pub(super) file_offset: u32,
    pub(super) bytecount: u32,
}

impl BitVec {
    pub fn load_bit_vector(&self) -> Result<Vec<u8>> {
        let mut reader_mut = self.reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(self.file_offset as u64))?;

        let mut v = vec![0u8; self.bytecount as usize];
        reader_mut.read_exact(&mut v)?;

        Ok(v)
    }
}

impl Debug for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.load_bit_vector())
    }
}