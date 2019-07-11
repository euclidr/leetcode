struct Solution;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut alphabet = vec![0; 26];
        for c in tiles.chars() {
            alphabet[c as usize - 'A' as usize] += 1;
        }

        let mut stat = vec![];
        for a in alphabet.iter() {
            if *a > 0 {
                stat.push(*a);
            }
        }

        Solution::solve(&mut stat, 0) - 1
    }

    fn fac(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }

        let mut rs = 1;
        for i in 2..n + 1 {
            rs *= i;
        }
        rs
    }

    fn solve(stat: &mut Vec<i32>, idx: usize) -> i32 {
        let mut result = 0;

        if idx == stat.len() {
            let mut div = 1;
            let mut cnt = 0;
            for i in stat.iter() {
                if *i > 0 {
                    cnt += *i;
                    div *= Solution::fac(*i)
                }
            }
            return Solution::fac(cnt) / div;
        }

        let org = stat[idx];

        while stat[idx] >= 0 {
            result += Solution::solve(stat, idx + 1);
            stat[idx] -= 1;
        }

        stat[idx] = org;
        result
    }
}

fn main() {
    println!("{}", Solution::num_tile_possibilities("AAABBC".to_string()));
}
