#![allow(unused)]
pub mod prime;

use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use itertools::Itertools;
use prime::PrimeSiv;

use crate::prime::PrimeFactorization;

fn main() {
    println!("Use `cargo test {{problem}}` to execute a problem");
}

#[test]
fn prob_010() {
    const MAX: usize = 2_000_000;
    let mut siv = BitVec::<usize, Lsb0>::repeat(true, MAX + 1);
    for n in 2..(MAX as f64).sqrt() as usize + 1 {
        if *siv.get(n).unwrap() {
            for i in 2.. {
                let pos = i * n;
                if pos > MAX {
                    break;
                }
                siv.set(pos, false);
            }
        }
    }
    let sum: usize = (2..MAX)
        .into_iter()
        .filter(|&i| *siv.get(i).unwrap())
        // .inspect(|i| println!("{i}"))
        .sum();
    println!("{sum}");
}

#[test]
fn prob_008() {
    let num = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";
    let val = num
        .chars()
        .filter_map(|c| c.to_digit(10))
        .tuple_windows()
        .map(|(a, b, c, d)| a * b * c * d)
        .max()
        .unwrap();
    println!("{val}");
}

#[test]
fn prob_007() {
    // 10001 is index 10000, b/c zero indexing
    let siv = PrimeSiv::new(1000000).nth(10000);
    println!("{}", siv.unwrap());
}

#[test]
fn prob_006() {
    // (sum * sum) - (sum of sq)
    let sum_of_sq: usize = (1..=100).map(|n| n * n).sum();
    let sq_of_sum: usize = (1..=100).sum::<usize>().pow(2);
    dbg!(sum_of_sq);
    dbg!(sq_of_sum);
    println!("{}", sq_of_sum - sum_of_sq);
}

#[test]
fn prob_005() {
    let mut siv = PrimeSiv::new(20);
    let mut accum = PrimeFactorization::new(2, &mut siv);
    for i in 2..20 {
        let factors = PrimeFactorization::new(i, &mut siv);
        dbg!(&factors);
        accum.or_factors(&factors);
    }
    println!("{}", accum.compute());
}

#[test]
fn prob_004() {
    fn is_pallindrome(n: usize) -> bool {
        let mut num = n;
        let mut rev = 0;
        while (num > 0) {
            let dig = num % 10;
            rev = rev * 10 + dig;
            num = num / 10;
        }
        rev == n
    }
    // TODO
    let mut max = 0;
    for a in (100..=999).rev() {
        if a * 999 <= max {
            break;
        }
        for b in (100..=999).rev() {
            let v = a * b;
            if v <= max {
                break;
            }
            if is_pallindrome(v) {
                max = v;
            }
        }
    }
    println!("{max}")
}

#[test]
fn prob_003() {
    let max = 600851475143;
    // Reduce max for memory and timing
    let siv = prime::PrimeSiv::new(max / 32 / 32 / 32 / 32);
    let mut rem = max;
    for prime in siv {
        while rem % prime == 0 {
            if rem == prime {
                break;
            }
            rem /= prime;
        }
        if rem == prime {
            break;
        }
    }
    println!("{rem}");
}

#[test]
fn prob_002() {
    struct FibIter {
        prev: usize,
        cur: usize,
    }
    impl Iterator for FibIter {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.prev + self.cur;
            self.prev = self.cur;
            self.cur = next;
            Some(self.cur)
        }
    }
    let sum: usize = FibIter { prev: 1, cur: 1 }
        .take_while(|&x| x <= 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("{sum}")
}

#[test]
fn prob_001() {
    let sum: usize = (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("{sum}");
}
