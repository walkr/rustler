use wrapper::binary::{alloc, realloc, ErlNifBinary};
use wrapper::nif_interface;
use {Decoder, Encoder, Env, Error, NifResult, Term};

use std::borrow::{Borrow, BorrowMut};
use std::io::Write;
use std::ops::{Deref, DerefMut};

// Owned

pub struct OwnedBinary {
    inner: ErlNifBinary,
    release: bool,
}

impl<'a> OwnedBinary {
    /// Allocates a new OwnedBinary with size `size`.
    ///
    /// Note that the memory is not initially guaranteed to be any particular value.
    /// If an empty buffer is needed, you should manually zero it.
    pub fn new(size: usize) -> Option<OwnedBinary> {
        unsafe { alloc(size) }.map(|binary| OwnedBinary {
            inner: binary,
            release: true,
        })
    }

    /// Copies a given `Binary`.
    pub fn from_unowned(from_bin: &Binary) -> Option<OwnedBinary> {
        let len = from_bin.len();
        if let Some(mut bin) = OwnedBinary::new(len) {
            if let Ok(write_len) = bin.as_mut_slice().write(from_bin.as_slice()) {
                if write_len != len {
                    panic!("Could not copy binary");
                }
                Some(bin)
            } else {
                panic!("Could not copy binary");
            }
        } else {
            None
        }
    }

    /// Attempts to reallocate the buffer with the new size.
    /// Returns false if the buffer cannot be reallocated.
    #[must_use]
    pub fn realloc(&mut self, size: usize) -> bool {
        unsafe { realloc(&mut self.inner, size) }
    }

    /// Attempts to reallocate the buffer with the new size.
    /// If reallocation fails, it will perform a copy instead.

    /// Memory outside the range of the original buffer will
    /// not be initialized. If this needs to be empty, clear it manually.
    pub fn realloc_or_copy(&mut self, size: usize) {
        if !self.realloc(size) {
            let mut new = OwnedBinary::new(size).unwrap();
            if let Ok(num_written) = new.as_mut_slice().write(self.as_slice()) {
                if !(num_written == self.len() || num_written == new.len()) {
                    panic!("Could not copy binary");
                }
                ::std::mem::swap(&mut self.inner, &mut new.inner);
            } else {
                panic!("Could not copy binary");
            }
        }
    }

    pub fn as_slice(&self) -> &'a [u8] {
        unsafe { ::std::slice::from_raw_parts(self.inner.data, self.inner.size) }
    }

    pub fn as_mut_slice(&mut self) -> &'a mut [u8] {
        unsafe { ::std::slice::from_raw_parts_mut(self.inner.data, self.inner.size) }
    }

    /// Releases control of the binary to the VM. After this point
    /// the binary will be immutable.
    pub fn release<'b>(self, env: Env<'b>) -> Binary<'b> {
        Binary::from_owned(self, env)
    }
}

impl Borrow<[u8]> for OwnedBinary {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}
impl BorrowMut<[u8]> for OwnedBinary {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}
impl Deref for OwnedBinary {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}
impl DerefMut for OwnedBinary {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl Drop for OwnedBinary {
    fn drop(&mut self) {
        if self.release {
            unsafe { nif_interface::enif_release_binary(self.inner.as_c_arg()) };
        }
    }
}

unsafe impl Send for OwnedBinary {}

// Borrowed

#[derive(Copy, Clone)]
pub struct Binary<'a> {
    inner: ErlNifBinary,
    term: Term<'a>,
}

impl<'a> Binary<'a> {
    /// Releases a given `OwnedBinary` to the VM.
    /// After this point the binary will be immutable.
    pub fn from_owned(mut bin: OwnedBinary, env: Env<'a>) -> Self {
        bin.release = false;
        let term = unsafe {
            Term::new(
                env,
                nif_interface::enif_make_binary(env.as_c_arg(), bin.inner.as_c_arg()),
            )
        };
        Binary {
            inner: bin.inner.clone(),
            term: term,
        }
    }

    pub unsafe fn from_raw(env: Env<'a>, bin: ErlNifBinary) -> Binary<'a> {
        Binary {
            inner: bin,
            term: Term::new(env, bin.bin_term),
        }
    }

    pub fn to_owned(&self) -> Option<OwnedBinary> {
        OwnedBinary::from_unowned(self)
    }

    pub fn from_term(term: Term<'a>) -> Result<Self, Error> {
        let mut binary = unsafe { ErlNifBinary::new_empty() };
        if unsafe {
            nif_interface::enif_inspect_binary(
                term.get_env().as_c_arg(),
                term.as_c_arg(),
                binary.as_c_arg(),
            )
        } == 0
        {
            return Err(Error::BadArg);
        }
        Ok(Binary {
            inner: binary,
            term: term,
        })
    }

    pub fn to_term<'b>(&self, env: Env<'b>) -> Term<'b> {
        self.term.in_env(env)
    }

    pub fn as_slice(&self) -> &'a [u8] {
        unsafe { ::std::slice::from_raw_parts(self.inner.data, self.inner.size) }
    }

    /// Returns a new view into the same binary.
    /// This will not copy anything.
    pub fn make_subbinary(&self, offset: usize, length: usize) -> NifResult<Binary<'a>> {
        let min_len = length.checked_add(offset);
        if try!(min_len.ok_or(Error::BadArg)) > self.inner.size {
            return Err(Error::BadArg);
        }

        let raw_term = unsafe {
            nif_interface::enif_make_sub_binary(
                self.term.get_env().as_c_arg(),
                self.inner.bin_term,
                offset,
                length,
            )
        };
        let term = unsafe { Term::new(self.term.get_env(), raw_term) };
        // This should never fail, as we are always passing in a binary term.
        Ok(Binary::from_term(term).ok().unwrap())
    }
}

impl<'a> Borrow<[u8]> for Binary<'a> {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}
impl<'a> Deref for Binary<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> Decoder<'a> for Binary<'a> {
    fn decode(term: Term<'a>) -> Result<Self, Error> {
        Binary::from_term(term)
    }
}
impl<'a> Encoder for Binary<'a> {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        self.to_term(env)
    }
}

/// ## Binary terms
impl<'a> Term<'a> {
    pub fn into_binary(self) -> NifResult<Binary<'a>> {
        Binary::from_term(self)
    }
}
