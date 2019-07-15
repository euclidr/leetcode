extern crate rand;

use rand::Rng;

struct Solution {
    use_set: bool,
    n: i32,
    blacklist: Vec<i32>,
    set: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(N: i32, mut blacklist: Vec<i32>) -> Self {
        blacklist.sort();

        let mut use_set = false;
        if blacklist.len() * 3 > (N as usize) {
            use_set = true;
        }
        let mut set = vec![];
        if use_set {
            let mut bl_i = 0;
            for i in 0..N {
                if bl_i < blacklist.len() && i == blacklist[bl_i] {
                    bl_i += 1;
                    continue;
                }
                set.push(i);
            }
        }

        Solution {
            use_set,
            n: N,
            blacklist,
            set,
        }
    }
    fn pick(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        if self.use_set {
            let idx = rng.gen_range(0, self.set.len());
            return self.set[idx as usize];
        }

        loop {
            let rs = rng.gen_range(0, self.n);
            match self.blacklist.binary_search(&rs) {
                Ok(_) => continue,
                Err(_) => return rs,
            }
        }

        unreachable!()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(N, blacklist);
 * let ret_1: i32 = obj.pick();
 */
fn main() {
    let mut s = Solution::new(2, vec![0]);
    println!("{}", s.pick());
    println!("{}", s.pick());
    println!("{}", s.pick());
    println!("{}", s.pick());
    println!("{}", s.pick());
    println!("{}", s.pick());
    println!("{}", s.pick());
}
