use rustler::Encoder;
use rustler::{Env, Term, NifResult};

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

pub fn on_load(_env: Env) {
}

pub fn atom_to_string<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let atom_string = try!(args[0].atom_to_string());
    Ok(atom_string.encode(env))
}

pub fn atom_equals_ok<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    Ok((atoms::ok() == args[0]).encode(env))
}
