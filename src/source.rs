use core::str;
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub struct VexationsSource<'src> {
    pub(crate) source: &'src str,
}

impl<'src> VexationsSource<'src> {
    pub const fn try_from_bytes(bytes: &'src [u8]) -> Option<Self> {
        if bytes.is_ascii() {
            let string = unsafe { str::from_utf8_unchecked(bytes) };
            Some(VexationsSource {
                source: string,
            })
        } else {
            None
        }
    }

    #[inline]
    pub const fn as_bytes(&self) -> &'src [u8] {
        self.source.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct LineCol {
    line: NonZeroUsize,
    col: usize,
}
