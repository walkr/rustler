use super::nif_interface;
use super::nif_interface::{NifResourceDtor, NifResourceFlags, NIF_ENV, NIF_RESOURCE_HANDLE,
                           NIF_RESOURCE_TYPE, NIF_TERM};

pub use super::nif_interface::{enif_alloc_resource as alloc_resource,
                               enif_keep_resource as keep_resource,
                               enif_make_resource as make_resource};

#[allow(dead_code)]
pub use super::nif_interface::enif_release_resource as release_resource;

use std::mem;
use std::ptr;

pub unsafe fn open_resource_type(
    env: NIF_ENV,
    name: &[u8],
    dtor: Option<NifResourceDtor>,
    flags: NifResourceFlags,
) -> Option<NIF_RESOURCE_TYPE> {
    // Panic if name is not null-terminated.
    assert_eq!(name.last().cloned(), Some(0u8));

    // Currently unused as per erlang nif documentation
    let module_p: *const u8 = ptr::null();
    let name_p = name.as_ptr();
    let res = {
        let mut tried: NifResourceFlags = mem::uninitialized();
        nif_interface::enif_open_resource_type(env, module_p, name_p, dtor, flags, &mut tried)
    };

    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

// Functionally incomplete
pub unsafe fn get_resource(
    env: NIF_ENV,
    term: NIF_TERM,
    typ: NIF_RESOURCE_TYPE,
) -> Option<NIF_RESOURCE_HANDLE> {
    let mut ret_obj: NIF_RESOURCE_HANDLE = mem::uninitialized();
    let res = nif_interface::enif_get_resource(env, term, typ, &mut ret_obj);

    if res == 0 {
        None
    } else {
        Some(ret_obj)
    }
}
