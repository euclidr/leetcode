struct Solution;

fn main() {
    // println!("{}", Solution::racecar(6));
    println!("{}", Solution::racecar(987));
    println!("{}", Solution::racecar(2));
}

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let mut dp = vec![0; (target as usize) + 1];
        let target = target as usize;
        for n in 1..(target + 1) {
            let mut right_steps = 1;
            let mut right_value = (1 << right_steps) - 1;
            while right_value < n {
                right_steps += 1;
                right_value = (1 << right_steps) - 1;
            }

            if right_value == n {
                dp[n] = right_steps;
                continue;
            }

            dp[n] = right_steps + 1 + dp[right_value - n];
            // println!("right: {} {}", right_steps, right_value);

            let left_steps = right_steps - 1;
            let left_value = (1 << left_steps) - 1;

            for withdraw_steps in 0..left_steps {
                let withdraw = (1 << withdraw_steps) - 1;
                let steps = left_steps + 1 + dp[withdraw] + 1 + dp[n - (left_value - withdraw)];
                if steps < dp[n] {
                    dp[n] = steps
                }
            }
        }
        // println!("{:?}", &dp[0..3]);

        dp[target] as i32
    }
}

// slow
// impl Solution {
//     pub fn racecar(target: i32) -> i32 {
//         use std::collections::HashMap;

//         fn rececar_from(pos: i32, mut speed: i32, target: i32, mut max_cnt: i32, record: &mut HashMap<i32, i32>) -> Option<i32> {
//             // println!("pos:{} speed:{} target:{} max_cnt:{}", pos, speed, target, max_cnt);
//             if pos == target {
//                 return Some(0);
//             }

//             if max_cnt == 0 {
//                 return None;
//             }

//             let key = speed * target * 10 + (target - pos);
//             match record.get(&key) {
//                 Some(v) => {
//                     if *v > max_cnt {
//                         return None;
//                     } else {
//                         return Some(*v);
//                     }
//                 },
//                 None => (),
//             };

//             let result1 = rececar_from(pos + speed, speed*2, target, max_cnt-1, record);
//             max_cnt = match result1 {
//                 Some(cnt) => cnt + 1,
//                 None => max_cnt,
//             };

//             if speed > 0 {
//                 speed = -1;
//             } else {
//                 speed = 1;
//             }

//             let result2 = rececar_from(pos, speed, target, max_cnt-1, record);

//             if let Some(v) = result2 {
//                 record.insert(key, v + 1);
//                 return Some(v + 1);
//             }

//             if let Some(v) = result1 {
//                 record.insert(key, v + 1);
//                 return Some(v + 1);
//             }

//             None
//         }

//         let mut pos = 0;
//         let mut speed = 1;
//         let mut cnt = 0;
//         while pos != target {
//             if speed + pos > target {
//                 cnt += 2;
//                 speed = 1;
//                 continue;
//             }
//             pos += speed;
//             speed *= 2;
//             cnt += 1;
//         }
//         let mut record = HashMap::new();
//         rececar_from(0, 1, target, cnt, &mut record).unwrap()
//     }
// }
