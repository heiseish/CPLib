mod hayasaka {
    // loop through a subset of a mask. total time complexity 3 ^ n
    pub struct BitMaskSubSet {
        mask: usize,
        cur: i32,
    }

    impl BitMaskSubSet {
        fn new(mask: usize) -> BitMaskSubSet {
            BitMaskSubSet {
                mask,
                cur: mask as i32,
            }
        }
    }

    impl Iterator for BitMaskSubSet {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.cur <= 0 {
                return None;
            }
            let ret = Some(self.cur as usize);
            self.cur = (self.cur - 1) & self.mask as i32;
            ret
        }
    }

    pub fn bitmask_subset(mask: usize) -> BitMaskSubSet {
        BitMaskSubSet::new(mask)
    }
}
use hayasaka::bitmask_subset;
