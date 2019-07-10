struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let (mut result, mut range, mut meet_one) = (0, 0, false);

        for i in seats.iter() {
            if *i == 0 {
                range += 1;
            }

            if *i == 1 {
                if !meet_one {
                    result = range;
                    meet_one = true;
                } else if result < (range + 1) / 2 {
                    result = (range + 1) / 2;
                }
                range = 0;
            }
        }

        if range > result {
            return range;
        }

        result
    }
}

fn main() {
    println!("{}", Solution::max_dist_to_closest(vec![1,0,0,0,1,0,1]));
}