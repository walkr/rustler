use super::NifEnv;
use super::wrapper::nif_interface::enif_consume_timeslice;

impl NifEnv {

    pub fn consume_timeslice(&self, percent: i32) -> bool {
        let success = unsafe { enif_consume_timeslice(self.as_c_arg(), percent) };
        success == 1
    }

    //#[cfg(feature = "experimental")]
    //pub fn schedule_nif<'a>(&'a self,
    //                        fun_name: &str,
    //                        function: for<'a> fn(&'a NifEnv, &Vec<NifTerm>) -> NifResult<NifTerm<'a>>,
    //                        args: &[NifTerm]) -> NifTerm<'a> {
    //    unreachable!();
    //}

}
