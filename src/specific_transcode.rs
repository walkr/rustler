use super::{ NifEnv, NifTerm, NifResult };

pub trait TranscodeTarget {}

macro_rules! define_target {
    ($name:ident) => {
        pub struct $name;
        impl $crate::specific_transcode::TranscodeTarget for $name {}
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

pub trait SpecificEncode<S> where S: TranscodeTarget {
    fn encode<'a>(self, env: &'a NifEnv) -> NifTerm<'a>;
}

pub trait SpecificDecode<'a, S>: Sized + 'a where S: TranscodeTarget {
    fn decode(term: NifTerm<'a>) -> NifResult<Self>;
}
