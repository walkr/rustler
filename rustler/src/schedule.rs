use super::wrapper::nif_interface::enif_consume_timeslice;
use super::Env;

pub enum SchedulerFlags {
    Normal = 0,
    DirtyCpu = 1,
    DirtyIo = 2,
}

pub fn consume_timeslice<'a>(env: Env<'a>, percent: i32) -> bool {
    let success = unsafe { enif_consume_timeslice(env.as_c_arg(), percent) };
    success == 1
}
