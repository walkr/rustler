use super::nif_interface;
use super::nif_interface::{ NIF_ENV, NIF_TERM, NIF_PID };
use std::mem;

pub fn alloc_env() -> NIF_ENV {
    unsafe { nif_interface::enif_alloc_env() }
}

pub fn free_env(env: NIF_ENV) {
    unsafe { nif_interface::enif_free_env(env) }
}

pub fn send(calling_env: NIF_ENV, to_pid: NIF_PID, msg_env: NIF_ENV, msg: NIF_TERM) -> bool {
    let success = unsafe { nif_interface::enif_send(calling_env, to_pid, msg_env, msg) };
    success == 1
}

pub fn self_pid(caller_env: NIF_ENV) -> NIF_PID {
    let pid: NIF_PID = unsafe { mem::uninitialized() };
    unsafe { nif_interface::enif_self(caller_env, pid) }
}
