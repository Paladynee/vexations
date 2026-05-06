use core::fmt;
use core::num::NonZeroUsize;
use core::slice;
use core::str;
use std::hint::unreachable_unchecked;

#[derive(Debug, Clone)]
pub struct VexationsSource<'src> {
    /// The string has 3 bytes of zeros at the end of it
    /// ```
    ///           v------ padding
    /// [a, b, c, 0, 0, 0]
    /// ^---------------- &str
    /// ```
    pub(crate) src: &'src str,
}

impl<'src> VexationsSource<'src> {
    /// The last 3 bytes inside the source must be all zeros. If it's not, push
    /// them prior to calling.
    #[inline]
    pub const fn try_from_bytes(bytes: &'src [u8]) -> Option<Self> {
        let &[.., a, b, c] = bytes else {
            return None;
        };

        if (a | b | c) != 0 {
            return None;
        }

        if !bytes.is_ascii() {
            return None;
        }

        Some(VexationsSource {
            src: unsafe { str::from_utf8_unchecked(bytes) },
        })
    }

    #[inline(always)]
    pub const fn buffer(&self) -> &'src [u8] {
        self.src.as_bytes()
    }

    #[inline(always)]
    pub const fn buffer_len(&self) -> usize {
        self.src.len()
    }

    #[inline]
    pub const fn source_len(&self) -> usize {
        unsafe { self.src.len().unchecked_sub(3) }
    }
}

#[derive(Debug, Clone)]
pub struct LineCol {
    pub line: NonZeroUsize,
    pub col: usize,
}

impl fmt::Display for LineCol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line.get(), self.col)
    }
}
