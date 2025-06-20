use arbitrary_int::u3;

use crate::register::{Dccisw, Dccsw, Dcisw};

#[inline]
pub fn invalidate_l1_cache<const A: usize, const N: usize, const S: usize>() {
    let ways = 1 << A;
    let sets = 1 << S;

    for set in 0..sets {
        for way in 0..ways {
            unsafe { Dcisw::write(Dcisw::new::<A, N>(way, set, u3::new(0))) };
        }
    }
}

#[inline]
pub fn clean_l1_cache<const A: usize, const N: usize, const S: usize>() {
    let ways = 1 << A;
    let sets = 1 << S;

    for set in 0..sets {
        for way in 0..ways {
            unsafe { Dccsw::write(Dccsw::new::<A, N>(way, set, u3::new(0))) };
        }
    }
}

#[inline]
pub fn clean_and_invalidate_l1_cache<const A: usize, const N: usize, const S: usize>() {
    let ways = 1 << A;
    let sets = 1 << S;

    for set in 0..sets {
        for way in 0..ways {
            unsafe { Dccisw::write(Dccisw::new::<A, N>(way, set, u3::new(0))) };
        }
    }
}
