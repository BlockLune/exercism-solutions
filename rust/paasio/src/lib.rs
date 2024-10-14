use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            bytes_through: 0, // the total number of bytes read
            reads: 0,         // the total number of read operations
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n_bytes = self.wrapped.read(buf)?;
        self.bytes_through += n_bytes;
        self.reads += 1;
        Ok(n_bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            bytes_through: 0, // the total number of bytes written
            writes: 0,        // the total number of write operations
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n_bytes = self.wrapped.write(buf)?;
        self.bytes_through += n_bytes;
        self.writes += 1;
        Ok(n_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
