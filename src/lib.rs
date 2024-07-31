use std::hash::Hash;
use std::fmt::{Debug, Display};

pub trait ObjIdTraits: Hash + Eq + PartialEq + Clone + Debug + Display {}
impl ObjIdTraits for usize {}
impl ObjIdTraits for u64 {}
impl ObjIdTraits for u32 {}
impl ObjIdTraits for i64 {}
impl ObjIdTraits for i32 {}
impl ObjIdTraits for String {}

pub trait Cache <ObjId:ObjIdTraits> {
    fn get_mr(&mut self,trace: dyn Iterator::<Item = ObjId>, cache_size: usize) -> f64;
}

