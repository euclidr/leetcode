struct Solution;
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut all = vec![];
        let mut marks = vec![0; nums.len()];
        for (i, a_list) in nums.iter().enumerate() {
            let mut tmp = a_list.clone();
            tmp.dedup();
            for val in tmp.iter() {
                all.push((*val, i));
            }
        }

        all.sort_by(|a, b| a.0.cmp(&b.0));
        let mut table = vec![];
        table.push((all[0].0, vec![all[0].1]));

        let mut pre = table[0].0;
        for item in all.iter().skip(1) {
            if item.0 == pre {
                if let Some(last) = table.last_mut() {
                    last.1.push(item.1);
                }
            } else {
                table.push((item.0, vec![item.1]));
                pre = item.0;
            }
        }

        let (mut head, mut tail) = (0, 0);
        for i in 0..table.len() {
            if Solution::fill(&table[i].1, &mut marks) {
                if Solution::is_full(&marks) {
                    tail = i;
                    break;
                }
            }
        }

        let mut head_turn = true;
        let (mut head_val, mut tail_val) = (table[head].0, table[tail].0);
        // println!("first result: {}, {}", head_val, tail_val);
        loop {
            if head_val == tail_val {
                break;
            }

            if !head_turn && tail == (table.len() - 1) {
                break;
            }

            // println!("turn_head: {}, head: {}, tail: {}", head_turn, head, tail);

            if head_turn {
                while head <= tail {
                    if table[tail].0 - table[head].0 < tail_val - head_val {
                        head_val = table[head].0;
                        tail_val = table[tail].0;
                    }

                    head += 1;
                    if Solution::strip(&table[head - 1].1, &mut marks) {
                        break;
                    }
                }
            }

            if !head_turn {
                let mut now_full = false;
                while tail < table.len() - 1 {
                    tail += 1;
                    if !Solution::fill(&table[tail].1, &mut marks) {
                        continue;
                    }
                    if Solution::is_full(&marks) {
                        now_full = true;
                        break;
                    }
                }
                if !now_full {
                    break;
                }
            }

            head_turn = !head_turn
        }

        vec![head_val, tail_val]
    }

    fn strip(indexes: &Vec<usize>, marks: &mut Vec<i32>) -> bool {
        let mut has_strip = false;
        for index in indexes.iter() {
            marks[*index] -= 1;
            if marks[*index] == 0 {
                has_strip = true;
            }
        }
        has_strip
    }

    fn fill(indexes: &Vec<usize>, marks: &mut Vec<i32>) -> bool {
        let mut has_new = false;
        for index in indexes.iter() {
            marks[*index] += 1;
            if marks[*index] == 1 {
                has_new = true;
            }
        }
        has_new
    }

    fn is_full(marks: &Vec<i32>) -> bool {
        for i in marks.iter() {
            if *i <= 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::smallest_range(vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ])
    )
}
