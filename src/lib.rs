use std::hash::Hash;
use std::fmt::{Debug, Display};
pub trait ObjIdTraits: Hash + Eq + PartialEq + Clone + Debug + Display {}

impl ObjIdTraits for usize {}

impl ObjIdTraits for u128 {}
impl ObjIdTraits for u64 {}
impl ObjIdTraits for u32 {}
impl ObjIdTraits for u16 {}
impl ObjIdTraits for u8 {}

impl ObjIdTraits for i128 {}
impl ObjIdTraits for i64 {}
impl ObjIdTraits for i32 {}
impl ObjIdTraits for i16 {}
impl ObjIdTraits for i8 {}

impl ObjIdTraits for &str {}
impl ObjIdTraits for String {}

pub enum AccessResult {
    Hit,
    Miss
}

///Allows access.into()
impl From<AccessResult> for bool {
    fn from(value: AccessResult) -> Self {
        match value {
            AccessResult::Hit => true,
            AccessResult::Miss => false,
        }
    }
}

///Allows true.into(), false.int()
impl From<bool> for AccessResult {
    fn from(value: bool) -> Self {
        if value { AccessResult::Hit } else { AccessResult::Miss }
    }
}

pub trait CacheSim <ObjId:ObjIdTraits> {
    fn cache_access(&mut self, access: ObjId) -> AccessResult;

    fn set_capacity(&mut self, cache_size:usize) -> &mut Self;

    fn get_total_miss(&mut self, trace: impl Iterator::<Item = ObjId>) -> (usize, usize) {
        trace.fold((0,0), |(mut total, mut miss), o| {
            let access = self.cache_access(o);
            total += 1;
            miss = if let AccessResult::Miss = access {miss + 1} else {miss};
            (total, miss)
        })
    }

    fn get_mr(&mut self, trace: impl Iterator::<Item = ObjId>) -> f64 {
        let (total, miss) = self.get_total_miss(trace);
        miss as f64 / total as f64
    }
}

