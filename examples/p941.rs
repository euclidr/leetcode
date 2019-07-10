struct Solution;

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 {
            return false;
        }

        let mut top = 0;
        for i in 1..a.len() {
            if a[i] == a[i - 1] {
                return false;
            }

            if a[i - 1] > a[i] {
                top = i - 1;
                break;
            }
        }

        if top == 0 {
            return false;
        }

        for i in (top + 1)..a.len() {
            if a[i] == a[i - 1] {
                return false;
            }
            if a[i] > a[i - 1] {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!("{:?}", Solution::valid_mountain_array(vec![0, 3, 2, 1]));
}
