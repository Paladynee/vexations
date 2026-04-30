use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use core::slice;
use core::str;

#[derive(Debug, Clone)]
pub struct VexationsSource<'src> {
    pub(crate) ptr: NonNull<u8>,
    len: usize,
    _phantom: PhantomData<&'src str>,
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
            ptr: NonNull::<[u8]>::from_ref(content).cast::<u8>(),
            len,
            _phantom: PhantomData,
        })
    }

    #[inline]
    pub const fn as_bytes(&self) -> &'src [u8] {
        unsafe {
            slice::from_raw_parts(self.ptr.as_ptr().cast_const(), self.len)
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineCol {
    line: NonZeroUsize,
    col: usize,
}
