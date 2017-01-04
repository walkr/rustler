macro_rules! impl_struct_tuple {
    ($type:path, $($field:ident),*) => {
        impl SpecificEncode<Tuple> for $type {
            fn encode<'a>(self, env: &'a NifEnv) -> NifTerm<'a> {
                
            }
        }
    }
}
