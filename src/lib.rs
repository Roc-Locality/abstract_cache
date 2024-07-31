use std::hash::Hash;
use std::fmt::{Debug, Display};

pub trait ObjIdTraits: Hash + Eq + PartialEq + Clone + Debug + Display {}
impl ObjIdTraits for usize {}
impl ObjIdTraits for u64 {}
impl ObjIdTraits for u32 {}
impl ObjIdTraits for i64 {}
impl ObjIdTraits for i32 {}
impl ObjIdTraits for String {}

pub trait CacheSim <ObjId:ObjIdTraits> {
    // fn set_trace(&mut self, trace: impl Iterator::<Item = ObjId>);

    fn get_total_miss(&mut self, trace: impl Iterator::<Item = ObjId>, cache_size: usize) -> (usize, usize);
    /// returns (total_access_count, miss_count)

    fn get_mr(&mut self, trace: impl Iterator::<Item = ObjId>, cache_size: usize) -> f64 {
        let (total, miss) = self.get_total_miss(trace, cache_size);
        miss as f64 / total as f64
    }
}

