use ::{NifEncoder, NifTerm};
use wrapper::nif_interface::{NIF_ENV};
use wrapper;

/// On each NIF call, a NifEnv is passed in. The NifEnv is used for most operations that involve
/// communicating with the BEAM, like decoding and encoding terms.
///
/// There is no way to allocate a NifEnv at the moment, but this may be possible in the future.
pub trait NifEnv {
    fn as_c_arg(&self) -> NIF_ENV;
    fn env_eq(&self, other: &NifEnv) -> bool {
        self.as_c_arg() == other.as_c_arg()
    }
}

pub struct CallerEnv {
    env: NIF_ENV,
}
impl CallerEnv {
    /// Unsafe because it allows you to duplicate NIF environments.
    pub unsafe fn new(env: NIF_ENV) -> CallerEnv {
        CallerEnv {
            env: env,
        }
    }
}
impl NifEnv for CallerEnv {
    fn as_c_arg(&self) -> NIF_ENV {
        self.env
    }
}

pub struct OwnedEnv {
    env: NIF_ENV,
}
impl OwnedEnv {
    pub fn new() -> Self {
        let env = wrapper::env::alloc_env();
        OwnedEnv {
            env: env,
        }
    }
}
impl NifEnv for OwnedEnv {
    fn as_c_arg(&self) -> NIF_ENV {
        self.env
    }
}

impl Drop for OwnedEnv {
    fn drop(&mut self) {
        wrapper::env::free_env(self.as_c_arg());
    }
}
