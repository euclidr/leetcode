struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }

        let mut new_envelops = vec![];
        for envelope in envelopes.iter() {
            new_envelops.push((envelope[0], envelope[1]));
        }

        new_envelops.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut results = vec![1; new_envelops.len()];
        for left in 0..new_envelops.len() {
            let cur = new_envelops[left];
            for right in left + 1..new_envelops.len() {
                let next = new_envelops[right];
                if cur.0 < next.0 && cur.1 < next.1 {
                    if (results[left] + 1) > results[right] {
                        results[right] = results[left] + 1;
                    }
                }
            }
        }

        // println!("{:?}", new_envelops);
        // println!("{:?}", results);

        let mut maxi = results[0];
        for val in results.iter().skip(1) {
            if *val > maxi {
                maxi = *val;
            }
        }

        maxi
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3],])
    );
}
