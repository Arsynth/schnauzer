use std::{path::Path, io::{self, Read, Seek, SeekFrom}, fs::File};
use super::result::*;

use std::rc::Rc;
use std::cell::RefCell;
pub(super) type RcReader = super::RcCell<Reader>;

pub struct Reader {
    file: File,
}

impl Reader {
    pub(super) fn build(path: &Path) -> Result<RcReader> {
        let file = File::open(path)?;
        Ok(Rc::new(RefCell::new(Reader { file })))
    }
}

impl Seek for Reader {
    fn seek(&mut self, style: SeekFrom) -> io::Result<u64> {
        self.file.seek(style)
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}
