use std::{path::Path, io::{self, Read, Seek, SeekFrom, BufReader, BufRead}, fs::File};
use crate::fmt_ext;

use super::result::*;

use std::rc::Rc;
use std::cell::RefCell;
pub(super) type RcReader = super::RcCell<Reader>;

pub struct Reader {
    buf_read: BufReader<File>,
}

impl Reader {
    pub(super) fn build(path: &Path) -> Result<RcReader> {
        let file = File::open(path)?;
        let buf_read = BufReader::new(file);
        Ok(Rc::new(RefCell::new(Reader { buf_read })))
    }
}

impl Seek for Reader {
    fn seek(&mut self, style: SeekFrom) -> io::Result<u64> {
        self.buf_read.seek(style)
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buf_read.read(buf)
    }
}

impl Reader {
    pub fn read_zero_terminated_string(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        self.buf_read.read_until(0, &mut buf)?;

        Ok(fmt_ext::printable_string(&buf))
    }
}