use core::num::NonZeroUsize;
use core::str;

#[derive(Debug, Clone)]
pub struct VexationsSource<'src> {
    pub(crate) src: &'src str,
}

impl<'src> VexationsSource<'src> {
    /// The last 3 bytes inside the source must be all zeros. This is for
    /// performance within the lexer.
    pub const fn try_from_bytes(bytes: &'src [u8]) -> Option<Self> {
        if bytes.len() < 4 {
            return None;
        }

        let len = bytes.len() - 3;
        let (content, padding) = unsafe { bytes.split_at_unchecked(len) };

        if padding[0] | padding[1] | padding[2] != 0 {
            return None;
        }

        if !bytes.is_ascii() {
            return None;
        }

        Some(VexationsSource {
            src: unsafe { str::from_utf8_unchecked(content) },
        })
    }

    #[inline(always)]
    pub const fn base_ptr(&self) -> *const u8 {
        self.src.as_ptr()
    }

    #[inline(always)]
    pub const fn end_ptr(&self) -> *const u8 {
        unsafe { self.src.as_ptr().add(self.src.len()) }
    }

    #[allow(clippy::len_without_is_empty)] // i said shut the FUCK up
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.src.len()
    }
}

#[derive(Debug, Clone)]
pub struct LineCol {
    line: NonZeroUsize,
    col: usize,
}
