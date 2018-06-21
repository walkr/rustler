extern crate erlang_nif_sys;
use types::atom;
use {Decoder, Encoder, Env, Error, NifResult, Term};

macro_rules! impl_number_transcoder {
    ($dec_type:ty, $nif_type:ty, $encode_fun:ident, $decode_fun:ident) => {
        impl Encoder for $dec_type {
            fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
                unsafe {
                    Term::new(
                        env,
                        erlang_nif_sys::$encode_fun(env.as_c_arg(), *self as $nif_type),
                    )
                }
            }
        }
        impl<'a> Decoder<'a> for $dec_type {
            fn decode(term: Term) -> NifResult<$dec_type> {
                #![allow(unused_unsafe)]
                let mut res: $nif_type = Default::default();
                if unsafe {
                    erlang_nif_sys::$decode_fun(
                        term.get_env().as_c_arg(),
                        term.as_c_arg(),
                        &mut res,
                    )
                } == 0
                {
                    return Err(Error::BadArg);
                }
                Ok(res as $dec_type)
            }
        }
    };
}

// Base number types
impl_number_transcoder!(i32, i32, enif_make_int, enif_get_int);
impl_number_transcoder!(u32, u32, enif_make_uint, enif_get_uint);
impl_number_transcoder!(i64, i64, enif_make_int64, enif_get_int64);
impl_number_transcoder!(u64, u64, enif_make_uint64, enif_get_uint64);
impl_number_transcoder!(f64, f64, enif_make_double, enif_get_double);

// Casted number types
impl_number_transcoder!(i8, i32, enif_make_int, enif_get_int);
impl_number_transcoder!(u8, u32, enif_make_uint, enif_get_uint);
impl_number_transcoder!(i16, i32, enif_make_int, enif_get_int);
impl_number_transcoder!(u16, u32, enif_make_uint, enif_get_uint);
impl_number_transcoder!(f32, f64, enif_make_double, enif_get_double);
impl_number_transcoder!(usize, u64, enif_make_uint64, enif_get_uint64);
impl_number_transcoder!(isize, i64, enif_make_int64, enif_get_int64);

impl Encoder for bool {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        if *self {
            atom::true_().to_term(env)
        } else {
            atom::false_().to_term(env)
        }
    }
}
impl<'a> Decoder<'a> for bool {
    fn decode(term: Term<'a>) -> NifResult<bool> {
        Ok(atom::is_truthy(term))
    }
}
