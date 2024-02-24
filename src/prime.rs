use std::{collections::HashMap, ops::Range};

use bitvec::vec::BitVec;

pub struct PrimeSiv {
    vec: BitVec,
    cur_max: usize,
}

impl PrimeSiv {
    pub fn new(max: usize) -> Self {
        Self {
            vec: BitVec::repeat(true, max),
            cur_max: 2,
        }
    }

    pub fn is_prime(&mut self, n: usize) -> bool {
        assert!(n < self.vec.len(), "Number out of bounds");
        while n >= self.cur_max {
            self.check_next();
        }
        self.vec[n]
    }

    fn check_next(&mut self) -> Option<usize> {
        debug_assert!(self.cur_max < self.vec.len());
        if self.vec[self.cur_max] {
            // Is prime
            for i in 2.. {
                let v = i * self.cur_max;
                if v >= self.vec.len() {
                    break;
                }
                self.vec.set(v, false);
            }
            let p = self.cur_max;
            self.cur_max += 1;
            Some(p)
        } else {
            self.cur_max += 1;
            None
        }
    }

    fn iter(&mut self) -> impl Iterator<Item = usize> + '_ {
        (2..self.vec.len()).filter(|n| self.is_prime(*n))
    }
}

impl Iterator for PrimeSiv {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_max < self.vec.len() {
            match self.check_next() {
                Some(v) => return Some(v),
                None => (),
            }
        }
        return None;
    }
}

#[derive(Debug)]
pub struct PrimeFactorization {
    vec: HashMap<usize, usize>,
}

impl PrimeFactorization {
    pub fn new(n: usize, siv: &mut PrimeSiv) -> Self {
        let mut vec = HashMap::new();
        let mut rem = n;
        for prime in siv.iter() {
            let mut count = 0;
            while rem % prime == 0 {
                count += 1;
                rem /= prime;
            }
            vec.insert(prime, count);
        }
        Self { vec }
    }

    pub fn or_factors(&mut self, other: &Self) {
        for (p, c) in &other.vec {
            let v = self.vec.entry(*p).or_default();
            *v = (*v).max(*c);
        }
    }

    pub fn compute(&self) -> usize {
        self.vec.iter().map(|(p, c)| p.pow(*c as u32)).product()
    }
}
