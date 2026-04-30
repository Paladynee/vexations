use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use core::str;

#[derive(Debug, Clone)]
pub struct VexationsSource<'src> {
    /// Logically stores `&[u8]` where the length is 3 short than the actual
    /// allocation size.
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
            len: content.len(),
            _phantom: PhantomData,
        })
    }

    #[inline(always)]
    pub const fn base_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    #[inline(always)]
    pub const fn end_ptr(&self) -> *const u8 {
        unsafe { self.ptr.add(self.len).as_ptr() }
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug, Clone)]
pub struct LineCol {
    line: NonZeroUsize,
    col: usize,
}
