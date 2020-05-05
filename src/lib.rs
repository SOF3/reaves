use std::io::{Read, Result, Write};

/// Delegates reads from the wrapped `Read` and writes a copy to the wrapped `Write`
pub struct Reaves<R: Read, W: Write> {
    r: R,
    w: W,
}

impl<R: Read, W: Write> Reaves<R, W> {
    pub fn new(r: R, w: W) -> Self {
        Self { r, w }
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
