#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
use core::f32::*;
use std::cmp::*;
use std::collections::*;
use std::f32::consts::*;
use std::fs::File;
use std::io::*;
use std::iter::FromIterator;
fn compress(mut a: Vec<i64>, mi: Option<usize>) -> HashMap<i64, usize> {
    let mut mi = mi.unwrap_or(0);
    let mut ret = HashMap::new();
    a.sort();
    for b in a {
        if ret.contains_key(&b) {
            continue;
        }
        ret.insert(b, mi);
        mi += 1;
    }
    ret
}

mod st {
    use std::ops::{Add, AddAssign};

    // 1-indexed
    // Query on range using FC function
    // for invalid value, trigger call FN
    // range addition (Note: NOT assignment)
    pub struct SegmentTreeRARQ<D, FC, FN>
    where
        D: Default + Copy + AddAssign,
        FC: FnOnce(&D, &D) -> D + Clone + Copy,
        FN: FnOnce() -> D + Clone + Copy,
    {
        pub n: usize,
        pub lazy: Vec<D>,
        pub t: Vec<D>,
        pub fc: FC,
        pub fnull: FN,
    }

    impl<D, FC, FN> SegmentTreeRARQ<D, FC, FN>
    where
        D: Default + Copy + AddAssign,
        FC: FnOnce(&D, &D) -> D + Clone + Copy,
        FN: FnOnce() -> D + Clone + Copy,
    {
        pub fn new(n: usize, fc: FC, fnull: FN) -> Self {
            let n = n + 3;
            let lazy = vec![D::default(); n << 2];
            let t = vec![D::default(); n << 2];
            Self {
                n,
                lazy,
                t,
                fc,
                fnull,
            }
        }

        // modify a and this fn if necessary
        // a is 1-indexed too
        pub fn build(n: usize, a: &[D], fc: FC, fnull: FN) -> Self {
            let sz = n + 3;
            let lazy = vec![false; sz << 2];
            let mut t = vec![D::default(); sz << 2];

            fn build_inner<D, FC>(v: usize, l: usize, r: usize, a: &[D], t: &mut [D], fc: &FC)
            where
                D: Default + Copy + AddAssign,
                FC: FnOnce(&D, &D) -> D + Clone + Copy,
            {
                if l == r {
                    t[v] = a[l];
                } else {
                    let mid = (l + r) >> 1;
                    build_inner(v << 1, l, mid, a, t, fc);
                    build_inner(v << 1 | 1, mid + 1, r, a, t, fc);
                    t[v] = fc(&t[v << 1], &t[v << 1 | 1]);
                }
            }

            build_inner(1, 1, n, &a, &mut t, &fc);
            Self {
                n,
                lazy,
                t,
                fc,
                fnull,
            }
        }

        #[inline]
        fn push(&mut self, v: usize, l: usize, r: usize) {
            self.t[v] += self.lazy[v];
            if l < r {
                let cur = self.lazy[v].clone();
                self.lazy[v << 1] += cur;
                self.lazy[v << 1 | 1] += cur;
            }
            self.lazy[v] = D::default();
        }

        #[inline]
        pub fn update(&mut self, x: usize, y: usize, to_add: D) {
            fn update_inner<D, FC, FN>(
                st: &mut SegmentTreeRARQ<D, FC, FN>,
                v: usize,
                l: usize,
                r: usize,
                x: usize,
                y: usize,
                to_add: D,
                fc: &FC,
            ) where
                D: Default + Copy + AddAssign,
                FC: FnOnce(&D, &D) -> D + Clone + Copy,
                FN: FnOnce() -> D + Clone + Copy,
            {
                st.push(v, l, r);
                if l > y || r < x {
                    return;
                }
                if l >= x && r <= y {
                    st.lazy[v] += to_add;
                    st.push(v, l, r);
                    return;
                }
                let mid = (l + r) >> 1;
                update_inner(st, v << 1, l, mid, x, y, to_add, fc);
                update_inner(st, v << 1 | 1, mid + 1, r, x, y, to_add, fc);
                st.t[v] = fc(&st.t[v << 1], &st.t[v << 1 | 1]);
            }
            let fc = self.fc.clone();
            update_inner(self, 1, 1, self.n, x, y, to_add, &fc);
        }

        #[inline]
        pub fn get(&mut self, x: usize, y: usize) -> D {
            fn get_inner<D, FC, FN>(
                st: &mut SegmentTreeRARQ<D, FC, FN>,
                v: usize,
                l: usize,
                r: usize,
                x: usize,
                y: usize,
                fc: &FC,
                fnull: &FN,
            ) -> D
            where
                D: Default + Copy + AddAssign,
                FC: FnOnce(&D, &D) -> D + Clone + Copy,
                FN: FnOnce() -> D + Clone + Copy,
            {
                if l > y || r < x {
                    return fnull();
                }
                st.push(v, l, r);
                if l >= x && r <= y {
                    return st.t[v];
                }
                let mid = (l + r) >> 1;
                fc(
                    &get_inner(st, v << 1, l, mid, x, y, fc, fnull),
                    &get_inner(st, v << 1 | 1, mid + 1, r, x, y, fc, fnull),
                )
            }
            let fc = self.fc.clone();
            let fnull = self.fnull.clone();
            get_inner(self, 1, 1, self.n, x, y, &fc, &fnull)
        }
    }

    // 1-indexed
    // Query on range using FC function
    // for invalid value, trigger call FN
    // range addition (Note: NOT assignment)
    pub struct SegmentTreeRMRQ<D, FC, FN>
    where
        D: Default + Copy + AddAssign,
        FC: FnOnce(&D, &D) -> D + Clone + Copy,
        FN: FnOnce() -> D + Clone + Copy,
    {
        pub n: usize,
        pub lazy: Vec<bool>,
        pub t: Vec<D>,
        pub fc: FC,
        pub fnull: FN,
    }

    impl<D, FC, FN> SegmentTreeRMRQ<D, FC, FN>
    where
        D: Default + Copy + AddAssign,
        FC: FnOnce(&D, &D) -> D + Clone + Copy,
        FN: FnOnce() -> D + Clone + Copy,
    {
        pub fn new(n: usize, fc: FC, fnull: FN) -> Self {
            let sz = n + 3;
            let lazy = vec![false; sz << 2];
            let t = vec![D::default(); sz << 2];
            Self {
                n,
                lazy,
                t,
                fc,
                fnull,
            }
        }

        // modify a and this fn if necessary
        // a is 1-indexed too
        pub fn build(n: usize, a: &[D], fc: FC, fnull: FN) -> Self {
            let sz = n + 3;
            let lazy = vec![false; sz << 2];
            let mut t = vec![D::default(); sz << 2];

            fn build_inner<D, FC>(v: usize, l: usize, r: usize, a: &[D], t: &mut [D], fc: &FC)
            where
                D: Default + Copy + AddAssign,
                FC: FnOnce(&D, &D) -> D + Clone + Copy,
            {
                if l == r {
                    t[v] = a[l];
                } else {
                    let mid = (l + r) >> 1;
                    build_inner(v << 1, l, mid, a, t, fc);
                    build_inner(v << 1 | 1, mid + 1, r, a, t, fc);
                    t[v] = fc(&t[v << 1], &t[v << 1 | 1]);
                }
            }

            build_inner(1, 1, n, &a, &mut t, &fc);
            Self {
                n,
                lazy,
                t,
                fc,
                fnull,
            }
        }

        #[inline]
        fn push(&mut self, v: usize, l: usize, r: usize) {
            if self.lazy[v] {
                self.t[v << 1] = self.t[v];
                self.t[v << 1 | 1] = self.t[v];
                self.lazy[v << 1] = true;
                self.lazy[v << 1 | 1] = true;
                self.lazy[v] = false;
            }
        }

        #[inline]
        pub fn update(&mut self, x: usize, y: usize, new_val: D) {
            fn update_inner<D, FC, FN>(
                st: &mut SegmentTreeRMRQ<D, FC, FN>,
                v: usize,
                l: usize,
                r: usize,
                x: usize,
                y: usize,
                new_val: D,
                fc: &FC,
            ) where
                D: Default + Copy + AddAssign,
                FC: FnOnce(&D, &D) -> D + Clone + Copy,
                FN: FnOnce() -> D + Clone + Copy,
            {
                if l > y || r < x {
                    return;
                }
                if l >= x && r <= y {
                    st.t[v] = new_val;
                    st.lazy[v] = true;
                    return;
                }
                st.push(v, l, r);
                let mid = (l + r) >> 1;
                update_inner(st, v << 1, l, mid, x, y, new_val, fc);
                update_inner(st, v << 1 | 1, mid + 1, r, x, y, new_val, fc);
                st.t[v] = fc(&st.t[v << 1], &st.t[v << 1 | 1]);
            }
            let fc = self.fc.clone();
            update_inner(self, 1, 1, self.n, x, y, new_val, &fc);
        }

        #[inline]
        pub fn get(&mut self, x: usize, y: usize) -> D {
            fn get_inner<D, FC, FN>(
                st: &mut SegmentTreeRMRQ<D, FC, FN>,
                v: usize,
                l: usize,
                r: usize,
                x: usize,
                y: usize,
                fc: &FC,
                fnull: &FN,
            ) -> D
            where
                D: Default + Copy + AddAssign,
                FC: FnOnce(&D, &D) -> D + Clone + Copy,
                FN: FnOnce() -> D + Clone + Copy,
            {
                if l > y || r < x {
                    return fnull();
                }
                if l >= x && r <= y {
                    return st.t[v];
                }
                st.push(v, l, r);
                let mid = (l + r) >> 1;
                fc(
                    &get_inner(st, v << 1, l, mid, x, y, fc, fnull),
                    &get_inner(st, v << 1 | 1, mid + 1, r, x, y, fc, fnull),
                )
            }
            let fc = self.fc.clone();
            let fnull = self.fnull.clone();
            get_inner(self, 1, 1, self.n, x, y, &fc, &fnull)
        }
    }
}

mod lazy_st {
    use crate::mint;
    #[derive(Default, Clone, Copy)]
    pub struct Node {
        pub val: modint,
        pub siz: i64,
    }

    pub fn f_combine_node(x: &Node, y: &Node) -> Node {
        Node {
            val: x.val + y.val,
            siz: x.siz + y.siz,
        }
    }

    pub fn f_invalid_node() -> Node {
        Node::default()
    }

    #[derive(Clone, Copy)]
    pub struct Act {
        pub b: modint,
        pub c: modint,
    }

    impl Default for Act {
        fn default() -> Self {
            Self {
                b: 1.into(),
                c: 0.into(),
            }
        }
    }

    pub fn f_map_act(f: &Act, x: &Node) -> Node {
        Node {
            val: f.b * x.val + f.c * x.siz,
            siz: x.siz,
        }
    }

    // Act composition function: gb((fb)x + fc) + gc = (gb fb)x + gb fc + gc
    // g is applied on f
    pub fn f_compose_act(g: &Act, f: &Act) -> Act {
        Act {
            b: g.b * f.b,
            c: g.b * f.c + g.c,
        }
    }

    // 1-indexed
    // Query on range using FC function
    // for invalid value, trigger call FN
    // range addition (Note: NOT assignment)
    pub struct LazySegmentTree<D, FC, FN, A, FM, FCA>
    where
        D: Default + Clone + Copy,
        FC: FnOnce(&D, &D) -> D + Copy,
        FN: FnOnce() -> D + Copy,
        A: Default + Clone + Copy,
        FM: FnOnce(&A, &D) -> D + Copy,
        FCA: FnOnce(&A, &A) -> A + Copy,
    {
        pub n: usize,
        pub lazy: Vec<A>,
        pub t: Vec<D>,
        pub fcombine: FC,
        pub fnull: FN,
        pub fmap: FM,
        pub fcompose: FCA,
    }

    impl<D, FC, FN, A, FM, FCA> LazySegmentTree<D, FC, FN, A, FM, FCA>
    where
        D: Default + Clone + Copy,
        FC: FnOnce(&D, &D) -> D + Copy,
        FN: FnOnce() -> D + Copy,
        A: Default + Clone + Copy,
        FM: FnOnce(&A, &D) -> D + Copy,
        FCA: FnOnce(&A, &A) -> A + Copy,
    {
        pub fn new(n: usize, fcombine: FC, fnull: FN, fmap: FM, fcompose: FCA) -> Self {
            let n = n + 3;
            let lazy = vec![A::default(); n << 2];
            let t = vec![D::default(); n << 2];
            Self {
                n,
                lazy,
                t,
                fcombine,
                fnull,
                fmap,
                fcompose,
            }
        }

        // modify a and this fn if necessary
        // a is 1-indexed too
        pub fn build(n: usize, a: &[D], fcombine: FC, fnull: FN, fmap: FM, fcompose: FCA) -> Self {
            let sz = n + 3;
            let lazy = vec![A::default(); sz << 2];
            let mut t = vec![D::default(); sz << 2];

            fn build_inner<D, FC>(v: usize, l: usize, r: usize, a: &[D], t: &mut [D], fcombine: &FC)
            where
                D: Default + Clone + Copy,
                FC: FnOnce(&D, &D) -> D + Copy,
            {
                if l == r {
                    t[v] = a[l];
                } else {
                    let mid = (l + r) >> 1;
                    build_inner(v << 1, l, mid, a, t, fcombine);
                    build_inner(v << 1 | 1, mid + 1, r, a, t, fcombine);
                    t[v] = fcombine(&t[v << 1], &t[v << 1 | 1]);
                }
            }

            build_inner(1, 1, n, &a, &mut t, &fcombine);
            Self {
                n,
                lazy,
                t,
                fcombine,
                fnull,
                fmap,
                fcompose,
            }
        }

        #[inline]
        fn push(&mut self, v: usize, l: usize, r: usize, fmap: &FM, fcompose: &FCA) {
            self.t[v] = fmap(&self.lazy[v], &self.t[v]);
            if l < r {
                self.lazy[v << 1] = fcompose(&self.lazy[v], &self.lazy[v << 1]);
                self.lazy[v << 1 | 1] = fcompose(&self.lazy[v], &self.lazy[v << 1 | 1]);
            }
            self.lazy[v] = A::default();
        }

        #[inline]
        pub fn update(&mut self, x: usize, y: usize, upd: A) {
            fn update_inner<D, FC, FN, A, FM, FCA>(
                st: &mut LazySegmentTree<D, FC, FN, A, FM, FCA>,
                v: usize,
                l: usize,
                r: usize,
                x: usize,
                y: usize,
                upd: A,
                fc: &FC,
                fmap: &FM,
                fcompose: &FCA,
            ) where
                D: Default + Clone + Copy,
                FC: FnOnce(&D, &D) -> D + Copy,
                FN: FnOnce() -> D + Copy,
                A: Default + Clone + Copy,
                FM: FnOnce(&A, &D) -> D + Copy,
                FCA: FnOnce(&A, &A) -> A + Copy,
            {
                st.push(v, l, r, fmap, fcompose);
                if l > y || r < x {
                    return;
                }
                if l >= x && r <= y {
                    st.lazy[v] = fcompose(&st.lazy[v], &upd);
                    st.push(v, l, r, fmap, fcompose);
                    return;
                }
                let mid = (l + r) >> 1;
                update_inner(st, v << 1, l, mid, x, y, upd, fc, fmap, fcompose);
                update_inner(st, v << 1 | 1, mid + 1, r, x, y, upd, fc, fmap, fcompose);
                st.t[v] = fc(&st.t[v << 1], &st.t[v << 1 | 1]);
            }
            let fcombine = self.fcombine;
            let fmap = self.fmap;
            let fcompose = self.fcompose;
            update_inner(self, 1, 1, self.n, x, y, upd, &fcombine, &fmap, &fcompose);
        }

        #[inline]
        pub fn get(&mut self, x: usize, y: usize) -> D {
            fn get_inner<D, FC, FN, A, FM, FCA>(
                st: &mut LazySegmentTree<D, FC, FN, A, FM, FCA>,
                v: usize,
                l: usize,
                r: usize,
                x: usize,
                y: usize,
                fc: &FC,
                fnull: &FN,
                fmap: &FM,
                fcompose: &FCA,
            ) -> D
            where
                D: Default + Clone + Copy,
                FC: FnOnce(&D, &D) -> D + Copy,
                FN: FnOnce() -> D + Copy,
                A: Default + Clone + Copy,
                FM: FnOnce(&A, &D) -> D + Copy,
                FCA: FnOnce(&A, &A) -> A + Copy,
            {
                if l > y || r < x {
                    return fnull();
                }
                st.push(v, l, r, fmap, fcompose);
                if l >= x && r <= y {
                    return st.t[v];
                }
                let mid = (l + r) >> 1;
                fc(
                    &get_inner(st, v << 1, l, mid, x, y, fc, fnull, fmap, fcompose),
                    &get_inner(st, v << 1 | 1, mid + 1, r, x, y, fc, fnull, fmap, fcompose),
                )
            }
            let fcombine = self.fcombine;
            let fnull = self.fnull;
            let fmap = self.fmap;
            let fcompose = self.fcompose;
            get_inner(
                self, 1, 1, self.n, x, y, &fcombine, &fnull, &fmap, &fcompose,
            )
        }
    }
}
use crate::lazy_st::*;
// let mut lazy_segtree = LazySegmentTree::build(
//     n,
//     &a,
//     f_combine_node,
//     f_invalid_node,
//     f_map_act,
//     f_compose_act,
// );

fn solve<R: BufRead, W: Write>(mut input: InputReader<R>, mut output: W) {
    use lazy_st::*;

    // read in int/float
    let n: usize = input.next();
    let q: usize = input.next();
    let mut a = vec![Node::default(); n + 1];
    for i in 1..=n {
        let x: i64 = input.next();
        a[i] = Node {
            val: x.into(),
            siz: 1,
        };
    }

    let mut lazy_segtree = LazySegmentTree::build(
        n,
        &a,
        f_combine_node,
        f_invalid_node,
        f_map_act,
        f_compose_act,
    );

    for _ in 0..q {
        let t: usize = input.next();
        match t {
            0 => {
                let l: usize = input.next();
                let r: usize = input.next();
                let b: i64 = input.next();
                let c: i64 = input.next();
                lazy_segtree.update(
                    l + 1,
                    r,
                    Act {
                        b: b.into(),
                        c: c.into(),
                    },
                );
            }
            1 => {
                let l: usize = input.next();
                let r: usize = input.next();
                let res = lazy_segtree.get(l + 1, r);
                write!(output, "{}\n", res.val);
            }
            _ => todo!(),
        }
    }
}
