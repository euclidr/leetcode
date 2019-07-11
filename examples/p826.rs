struct Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut mapper = vec![];
        for (dif, prof) in difficulty.iter().zip(profit.iter()) {
            mapper.push((*dif, *prof));
        }
        mapper.sort_by(|a, b| a.0.cmp(&b.0));

        let mut worker = worker;
        worker.sort();

        let (mut next, mut max, mut sum) = (0 as usize, 0, 0);
        for w in worker.iter() {
            while next < mapper.len() {
                if mapper[next].0 <= *w {
                    if mapper[next].1 > max {
                        max = mapper[next].1
                    }
                    next = next + 1;
                } else {
                    break;
                }
            }

            println!("{}", max);
            sum += max
        }

        sum
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_profit_assignment(
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7]
        )
    );
}
