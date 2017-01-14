#[macro_export]
macro_rules! default_encode {
    ($typ:ty, $target:path) => {
        impl NifEncoder for $typ {
            fn encode<'b>(&self, env: NifEnv<'b>) -> NifTerm<'b>{
                SpecificEncode::<$target>::encode(self, env)
            }
        }
    }
}
#[macro_export]
macro_rules! default_decode {
    ($typ:ty, $target:path) => {
        impl<'a> NifDecoder<'a> for $typ {
            fn decode(term: NifTerm<'a>) -> NifResult<$typ> {
                SpecificDecode::<'a, $target>::decode(term)
            }
        }
    }
}

use ::{
    NifEnv,
    NifTerm,
    NifResult,
};

#[macro_use]
pub mod atom;
pub mod binary;
pub mod list;
pub mod map;
pub mod primitive;
pub mod string;
pub mod tuple;

pub mod elixir_struct;

pub trait NifEncoder {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a>;
}
pub trait NifDecoder<'a>: Sized+'a {
    fn decode(term: NifTerm<'a>) -> NifResult<Self>;
}

impl<'a> NifEncoder for NifTerm<'a> {
    fn encode<'b>(&self, env: NifEnv<'b>) -> NifTerm<'b> {
        self.in_env(env)
    }
}
impl<'a> NifDecoder<'a> for NifTerm<'a> {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        Ok(term)
    }
}


pub trait TranscodeTarget {}

macro_rules! define_target {
    ($name:ident) => {
        pub struct $name;
        impl $crate::types::TranscodeTarget for $name {}
    }
}

pub mod target {
    define_target!(Atom);
    define_target!(Binary);
    define_target!(List);
    define_target!(Map);
    define_target!(Tuple);

    define_target!(Resource);
    define_target!(KeywordList);
    define_target!(ElixirStruct);
}


pub trait SpecificEncode<'a, Target> where Target: TranscodeTarget {
    fn encode(&self, env: NifEnv<'a>) -> NifTerm<'a>;
}
pub trait SpecificDecode<'a, Target>: Sized + 'a where Target: TranscodeTarget {
    fn decode(term: NifTerm<'a>) -> NifResult<Self>;
}
