use core::fmt;
use core::num::NonZeroUsize;
use core::str;
use std::hint::assert_unchecked;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct VexationsSource<'src> {
    /// The string has 3 bytes of zeros at the end of it
    /// ```
    ///           v------ padding
    /// [a, b, c, 0, 0, 0]
    /// ^---------------- src[..buffer_len]
    /// ^------- src[..src_len]
    /// ```
    src_ptr: NonNull<u8>,
    buffer_len: usize,
    src_len: usize,
    _marker: PhantomData<&'src str>,
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

        let res = VexationsSource {
            src_ptr: NonNull::from_ref(bytes).cast::<u8>(),
            buffer_len: bytes.len(),
            src_len: bytes.len() - 3,
            _marker: PhantomData,
        };
        unsafe { res.assert_invariants_for_opt() };
        Some(res)
    }

    /// this does not generate any code. all of these invariants are true for a
    /// [`VexationsSource`] that has been constructed legally.
    #[inline(always)]
    pub const unsafe fn assert_invariants_for_opt(&self) {
        unsafe {
            assert_unchecked(self.buffer_len >= 3);
            assert_unchecked(self.src_len < self.buffer_len);
            let _ = self.src_ptr.cast_slice(self.buffer_len).as_ref();
            let _ = self.src_ptr.cast_slice(self.src_len).as_ref();
        }
    }

    #[inline(always)]
    pub const fn buffer(&self) -> &'src [u8] {
        unsafe {
            self.assert_invariants_for_opt();
            self.src_ptr.cast_slice(self.buffer_len()).as_ref()
        }
    }

    #[inline(always)]
    pub const fn source(&self) -> &'src str {
        unsafe {
            self.assert_invariants_for_opt();
            str::from_utf8_unchecked(
                self.src_ptr.cast_slice(self.source_len()).as_ref(),
            )
        }
    }

    #[inline(always)]
    pub const fn buffer_len(&self) -> usize {
        unsafe { self.assert_invariants_for_opt() };
        self.buffer_len
    }

    #[inline(always)]
    pub const fn source_len(&self) -> usize {
        unsafe { self.assert_invariants_for_opt() };
        self.src_len
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
