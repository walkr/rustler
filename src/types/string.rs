use ::{ NifTerm, NifEnv, NifEncoder, NifDecoder, NifResult, NifError };
use super::binary::{ NifBinary, OwnedNifBinary };

use super::{ SpecificEncode, SpecificDecode };
use super::target;

use std::io::Write;

#[cfg(not(erlang))]
default_encode!(str, target::Binary);
#[cfg(erlang)]
default_encode!(str, target::List);
impl<'a> SpecificEncode<'a, target::Binary> for str {
    fn encode(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        let str_len = self.len();
        let mut bin = match OwnedNifBinary::alloc(str_len) {
            Some(bin) => bin,
            None => panic!("binary term allocation fail"),
        };
        bin.as_mut_slice().write(self.as_bytes()).expect("memory copy of string failed");
        bin.release(env).get_term(env)
    }
}

#[cfg(not(erlang))]
default_decode!(String, target::Binary);
#[cfg(erlang)]
default_decode!(String, target::List);
impl<'a> SpecificDecode<'a, target::Binary> for String {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        let string: &str = try!(NifDecoder::decode(term));
        Ok(string.to_string())
    }
}

#[cfg(not(erlang))]
default_decode!(&'a str, target::Binary);
#[cfg(erlang)]
default_decode!(&'a str, target::List);
impl<'a> SpecificDecode<'a, target::Binary> for &'a str {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        let binary = try!(NifBinary::from_term(term));
        match ::std::str::from_utf8(binary.as_slice()) {
            Ok(string) => Ok(string),
            Err(_) => Err(NifError::BadArg),
        }
    }
}
