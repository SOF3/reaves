use std::io::{Read, Result, Write, BufRead, Seek, SeekFrom};

/// Delegates reads from the wrapped `Read` and writes a copy to the wrapped `Write`
#[derive(Debug, Clone)]
pub struct Reaves<R: Read, W: Write> {
    r: R,
    w: W,
}

impl<R: Read, W: Write> Reaves<R, W> {
    pub fn new(r: R, w: W) -> Self {
        Self { r, w }
    }

    pub fn into_inner(self) -> (R, W) {
        (self.r, self.w)
    }
}

impl<R: Read, W: Write> Read for Reaves<R, W> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let ret = self.r.read(buf)?;
        if ret > 0 {
            self.w.write_all(&buf[..ret])?;
        }
        Ok(ret)
    }
}

impl<R: BufRead, W: Write> BufRead for Reaves<R, W> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.r.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.r.consume(amt)
    }
}

impl<R: Read + Seek, W: Write> Seek for Reaves<R, W> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.r.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};

    use super::Reaves;

    #[test]
    fn test() {
        let input = b"Hello world";
        let mut output = Vec::<u8>::new();

        let r = Cursor::new(input);
        let w = &mut output;

        let mut reaves = Reaves::new(r, w);

        let mut verify = vec![];
        assert_eq!(
            reaves
                .read_to_end(&mut verify)
                .expect("Cannot fail reading a Vec"),
            input.len()
        );
        assert_eq!(&verify, input);
        assert_eq!(&output, input);
    }
}
