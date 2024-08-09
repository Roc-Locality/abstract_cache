use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use indexmap::IndexMap;
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

    /* Computes the reuse interval sequence for the trace. This is done by creating a hashmap from
     * the trace elements to the last reuse interval. The upon revisiting an element in the
     * hashmap, the reuse interval is updated and added to the reuse interval sequence. */
    fn reuse_interval(&self, trace: impl Iterator<Item = ObjId>) -> Vec<usize> {
        let mut reuse_interval_vec = VecDeque::new();
        // Hashmap for storing previous reuse time of each element.
        let mut reuse_interval_map = HashMap::with_capacity(20);

        for (i, e) in trace.enumerate() {
            // Iterate over the trace
            if let Some(prev_ri) = reuse_interval_map.get(&e) {
                // Stores current time - previous RI in RI sequence
                let interval = i - prev_ri;
                reuse_interval_vec.push_back(interval);
                reuse_interval_map.insert(e, i);
            } else {
                // usize::MAX is to be viewed as infinity in this context
                reuse_interval_vec.push_back(usize::MAX);
                reuse_interval_map.insert(e, i);
            }
        }
        reuse_interval_vec.into()
    }

    fn access_times(&self, trace: impl Iterator<Item = ObjId>) -> IndexMap<ObjId, (usize, usize)> {
        let mut map = IndexMap::<ObjId, (usize, usize)>::with_capacity(20);
        for (time, x) in trace.enumerate() {
            map.entry(x)
                .and_modify(|y| {
                    let f = cmp::min(time, y.0);
                    let l = cmp::max(time, y.1);
                    *y = (f, l);
                })
                .or_insert((time, time));
        }
        map
    }

    // Gives footprint given size x. Takes average working set size.
    fn footprint(&self, trace: impl Iterator<Item = ObjId>) -> Vec<f32> {
        let t: Vec<_> = trace.collect();
        let n = t.len();
        let reuse_interval = self.reuse_interval(t.iter().cloned());
        let access_times = self.access_times(t.iter().cloned());
        // number of unique elements
        let m = access_times.keys().len();
        let ri_map = reuse_interval.iter().fold(
            HashMap::<&usize, usize>::with_capacity(m / 2),
            |mut map, ri| {
                *map.entry(ri).or_default() += 1 as usize;
                map
            },
        );

        (0..=n)
            .map(|window_length| {
                if window_length == 0 {
                    return 0 as f32;
                }
                let ri_hist_sum: usize = ((window_length + 1)..=(n - 1))
                    .map(|i: usize| (i - window_length) * ri_map.get(&i).unwrap_or(&0))
                    .sum();
                let first_access_sum: usize = (1..=m)
                    .map(|k| {
                        if let Some((_, (f_k, _))) = access_times.get_index(k - 1) {
                            if (f_k + 1) > window_length {
                                return (f_k + 1) - &window_length;
                            }
                        }
                        0
                    })
                    .sum();
                let last_access_sum: usize = (1..=m)
                    .map(|k| {
                        if let Some((_, (_, l_k))) = access_times.get_index(k - 1) {
                            if n - &window_length + 1 > (*l_k + 1) {
                                return n - &window_length + 1 - (l_k + 1);
                            }
                        }
                        0
                    })
                    .sum();
                let total = ri_hist_sum + first_access_sum + last_access_sum;
                let num_windows = n - window_length + 1;
                (m as f32) - (1.0 / (num_windows as f32) * (total as f32))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCache;

    impl CacheSim<usize> for MockCache {
        fn cache_access(&mut self, _: usize) -> AccessResult {
            unimplemented!();
        }

        fn set_capacity(&mut self, _: usize) -> &mut Self {
            unimplemented!();
        }
    }

    #[test]
    fn reuse_interval_test_1() {
        let cache = MockCache {};
        let trace: Vec<usize> = vec![1, 2, 3, 1, 2, 3];
        let reuse_interval = cache.reuse_interval(trace.into_iter());

        assert_eq!(
            &reuse_interval,
            &[usize::MAX, usize::MAX, usize::MAX, 3, 3, 3]
        );
    }

    #[test]
    fn reuse_interval_test_2() {
        let cache = MockCache {};
        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let reuse_interval = cache.reuse_interval(trace.into_iter());

        assert_eq!(
            &reuse_interval,
            &[usize::MAX, usize::MAX, usize::MAX, 1, 3, 5]
        );
    }

    #[test]
    fn footprint_test_1() {
        let cache = MockCache {};

        let trace: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        /*
        1: 1
        2: 2
        3: 3
        4: 4
        5: 5
        6: 6
        m = 6 # number of trace elements
        n = length of trace
        fp(1) = 6 - \frac{1}{6 -  1 + 1}(ri_hist + first + last)
              = 6 - (0 + first + last)
        first = sum_{k = 1}^m (f_k - x) * I(f_k > x)
              = 1 - 1 + 2 - 1 + 3 - 1 + 4 - 1 + 5 - 1 + 6 - 1
              = 15
         */
        let footprint = cache.footprint(trace.into_iter());

        assert_eq!(&footprint, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn footprint_test_2() {
        let cache = MockCache {};

        let trace: Vec<usize> = vec![1, 2, 3, 3, 2, 1];
        let footprint = cache.footprint(trace.into_iter());

        assert_eq!(
            &footprint,
            &[0.0, 1.0, 1.8, 2.5, 8 as f32 / 3 as f32, 3.0, 3.0]
        );
    }
}
