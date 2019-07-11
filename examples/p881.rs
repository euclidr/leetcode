struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let (mut head, mut tail) = (0, people.len()-1);

        let mut result = 0;

        while head < tail {
            if people[head] + people[tail] <= limit {
                result += 1;
                head += 1;
                tail -= 1;
            } else {
                result += 1;
                tail -= 1;
            }
        }

        if head == tail {
            result += 1;
        }

        result
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![2,1], 3));
}