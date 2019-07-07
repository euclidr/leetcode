struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let (values, operators) = Solution::parse(input);
        let n = values.len();

        let mut table = vec![vec![vec![]; n]; n];
        for col in 0..n {
            table[col][0].push(values[col]);
        }

        for span in 1..n {
            for col in 0..(n - span) {
                // [col, col+span] = table[col][0] op table[col+1][span-1] ...
                let mut cell = vec![];

                for sub_span in 0..span {
                    println!("{} {} {}", span, col, sub_span);
                    let op = operators[col + sub_span];
                    Solution::resolve(
                        op,
                        &table[col][sub_span],
                        &table[col + sub_span + 1][span - sub_span - 1],
                        &mut cell,
                    );
                }

                table[col][span] = cell;
            }
        }

        // println!("{:?}", operators);
        // println!("{:?}", values);
        // println!("{:?}", table);

        table[0][n - 1].clone()
    }

    fn parse(input: String) -> (Vec<i32>, Vec<char>) {
        let mut operators = vec![];
        let mut values = vec![];
        let mut start: usize = 0;
        let input_str = input.as_str();
        for (i, c) in input_str.chars().enumerate() {
            match c {
                '+' | '-' | '*' => {
                    let v: i32 = (&input_str[start..i]).parse().unwrap();
                    start = i + 1;
                    values.push(v);
                    operators.push(c);
                }
                _ => (),
            }
        }
        let last: i32 = (&input_str[start..input_str.len()]).parse().unwrap();
        values.push(last);

        (values, operators)
    }

    fn resolve(op: char, left: &Vec<i32>, right: &Vec<i32>, result: &mut Vec<i32>) {
        for l in left.iter() {
            for r in right.iter() {
                result.push(match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    _ => unreachable!(),
                })
            }
        }
    }

}

fn main() {
    Solution::diff_ways_to_compute("2*3-4*5".to_string());
    Solution::diff_ways_to_compute("2-1-1".to_string());
}
