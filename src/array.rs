use std::fmt::Debug;
use crate::idat::Idat;

#[derive(Clone, Debug)]
pub struct MicroArray {
    pub basename: String,
    pub green_idat: Idat,
    pub red_idat: Idat,
}

impl MicroArray {
    pub fn from_base<T: ToString>(basename: T) -> MicroArray {
        let base = basename.to_string();
        let new_array = MicroArray{
            basename: base.to_owned(),
            red_idat: Idat::red_from_base(&base),
            green_idat: Idat::green_from_base(&base)
        };
        return new_array
    }
}