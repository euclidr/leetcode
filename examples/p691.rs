struct Solution;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut charset = vec![0; 26];
        for c in target.chars() {
            charset[c as usize - 'a' as usize] += 1;
        }

        let stickers = Solution::convert_stickers(&stickers, &charset);
        // println!("{:?}", charset);
        // println!("{:?}", stickers);

        let maxi = match Solution::try_solve(&stickers, charset.clone()) {
            Some(d) => d,
            None => return -1,
        };

        match Solution::solve(&stickers, 0, maxi, charset) {
            Some(d) => d,
            None => unreachable!(),
        }
    }

    fn solve(
        stickers: &Vec<Vec<(usize, i32)>>,
        start: usize,
        mut maxi: i32,
        mut charset: Vec<i32>,
    ) -> Option<i32> {
        if start == stickers.len() {
            return None;
        }

        let mut result = None;
        let mut remain = Solution::remain_count(&charset);
        let mut strips = 0;
        while maxi - strips > 0 {
            match Solution::solve(&stickers, start + 1, maxi - strips, charset.clone()) {
                Some(d) => {
                    result = Some(strips + d);
                    maxi = strips + d;
                }
                None => (),
            };

            Solution::strip(&stickers[start], &mut charset);
            let new_remain = Solution::remain_count(&charset);
            if remain == new_remain {
                break;
            }

            remain = new_remain;
            strips += 1;

            if remain == 0 {
                return Some(strips);
            }
        }

        result
    }

    fn try_solve(stickers: &Vec<Vec<(usize, i32)>>, mut charset: Vec<i32>) -> Option<i32> {
        let mut depth = 0;
        let mut remain = Solution::remain_count(&charset);
        for sticker in stickers.iter() {
            if remain == 0 {
                break;
            }

            loop {
                Solution::strip(sticker, &mut charset);
                let new_remain = Solution::remain_count(&charset);
                if new_remain != remain {
                    remain = new_remain;
                    depth += 1;
                } else {
                    break;
                }
            }
        }

        if remain == 0 {
            return Some(depth);
        }

        None
    }

    fn strip(sticker: &Vec<(usize, i32)>, charset: &mut Vec<i32>) {
        for &(idx, val) in sticker.iter() {
            charset[idx] -= val;
        }
    }

    fn remain_count(charset: &Vec<i32>) -> i32 {
        let mut result = 0;
        for v in charset.iter() {
            if *v > 0 {
                result += *v;
            }
        }
        result
    }

    // fn is_done(charset: &Vec<i32>) -> bool {
    //     for c in charset.iter() {
    //         if *c > 0 {
    //             return false;
    //         }
    //     }
    //     true
    // }

    fn convert_stickers(stickers: &Vec<String>, charset: &Vec<i32>) -> Vec<Vec<(usize, i32)>> {
        let mut tmp_stickers = vec![];
        for sticker in stickers.iter() {
            let mut item = vec![0; 26];
            for c in sticker.chars() {
                let idx = c as usize - 'a' as usize;
                if charset[idx] > 0 {
                    item[idx] += 1;
                }
            }

            if Solution::is_charset_zero(&item) {
                continue;
            }
            tmp_stickers.push(item);
        }
        // println!("{}", tmp_stickers.len());

        let mut marks: Vec<bool> = vec![false; tmp_stickers.len()];
        for i in 0..tmp_stickers.len() {
            if marks[i] == true {
                continue;
            }
            for j in i + 1..tmp_stickers.len() {
                // println!("a: {:?}", &tmp_stickers[i]);
                // println!("b: {:?}", &tmp_stickers[j]);
                match Solution::is_larger(&tmp_stickers[i], &tmp_stickers[j]) {
                    Some(true) => {
                        marks[j] = true;
                        // println!("a > b");
                    }
                    Some(false) => {
                        marks[i] = true;
                        // println!("a < b");
                        break;
                    }
                    None => (),
                }
                // println!("-----");
            }
        }

        let mut new_stickers = vec![];
        for i in 0..tmp_stickers.len() {
            if marks[i] == true {
                continue;
            }
            let mut new_sticker = vec![];
            for (k, v) in tmp_stickers[i].iter().enumerate() {
                if *v > 0 {
                    new_sticker.push((k, *v));
                }
            }
            new_stickers.push(new_sticker);
        }

        new_stickers.sort_by(|a, b| b.len().cmp(&a.len()));

        new_stickers
    }

    fn is_larger(a: &Vec<i32>, b: &Vec<i32>) -> Option<bool> {
        let mut first_diff = a.len();
        for i in 0..a.len() {
            if a[i] != b[i] {
                first_diff = i;
                break;
            }
        }

        if first_diff == a.len() {
            return Some(false);
        }

        let result = a[first_diff] > b[first_diff];
        for i in first_diff + 1..a.len() {
            if a[i] == b[i] {
                continue;
            }

            if (a[i] > b[i]) != result {
                return None;
            }
        }

        Some(result)
    }

    fn is_charset_zero(charset: &Vec<i32>) -> bool {
        for c in charset.iter() {
            if *c != 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_stickers(
            vec![
                "with".to_string(),
                "example".to_string(),
                "science".to_string()
            ],
            "thehat".to_string()
        )
    )
}

// Wrong solution
// impl Solution {
//     pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
//         let mut charset: Vec<char> = target.chars().collect();
//         charset.sort();
//         charset.dedup();
//         // println!("{:?}", charset);

//         let stickers = Solution::clean_stickers(&stickers, &charset);
//         println!("{:?}", stickers);

//         let result = Solution::min_stickers2(&stickers, charset, 0, 1, None);

//         match result {
//             Some(n) => n,
//             None => -1,
//         }
//     }

//     fn min_stickers2(
//         stickers: &Vec<String>,
//         charset: Vec<char>,
//         start: usize,
//         depth: i32,
//         mut maxi: Option<i32>,
//     ) -> Option<i32> {
//         if let Some(d) = maxi {
//             if d == depth {
//                 return None;
//             }
//         }

//         for i in start..stickers.len() {
//             let mut new_charset = vec![];
//             for c in charset.iter() {
//                 if !Solution::contain_char(&stickers[i], *c) {
//                     new_charset.push(*c);
//                 }
//             }
//             if new_charset.len() == 0 {
//                 return Some(depth);
//             }

//             if let Some(d) = Solution::min_stickers2(stickers, new_charset, i + 1, depth + 1, maxi)
//             {
//                 maxi = Some(d)
//             }
//         }

//         maxi
//     }

//     fn clean_stickers(stickers: &Vec<String>, charset: &Vec<char>) -> Vec<String> {
//         let mut tmp_stickers = vec![];

//         for sticker in stickers.iter() {
//             let mut new_sticker = "".to_string();
//             for c in charset.iter() {
//                 for sticker_c in sticker.chars() {
//                     if *c == sticker_c {
//                         new_sticker.push(*c);
//                         break;
//                     }
//                 }
//             }
//             if new_sticker.len() > 0 {
//                 tmp_stickers.push(new_sticker);
//             }
//         }

//         if tmp_stickers.len() == 0 {
//             return tmp_stickers;
//         }

//         tmp_stickers.sort();
//         tmp_stickers.dedup();
//         tmp_stickers.sort_by(|a, b| a.len().cmp(&b.len()));

//         let mut new_sticker = vec![];

//         for i in 0..tmp_stickers.len() {
//             let mut contain = false;
//             for j in i..tmp_stickers.len() {
//                 if tmp_stickers[i].len() == tmp_stickers[j].len() {
//                     continue;
//                 }
//                 if Solution::contains(&tmp_stickers[j], &tmp_stickers[i]) {
//                     contain = true;
//                     break;
//                 }
//             }
//             if !contain {
//                 new_sticker.push(tmp_stickers[i].clone());
//             }
//         }

//         new_sticker.reverse();
//         new_sticker
//     }

//     fn contains(a: &String, b: &String) -> bool {
//         // a contains b?
//         for cb in b.chars() {
//             if !Solution::contain_char(a, cb) {
//                 return false;
//             }
//         }
//         true
//     }

//     fn contain_char(a: &String, c: char) -> bool {
//         for ch in a.chars() {
//             if c == ch {
//                 return true;
//             }
//         }
//         false
//     }
// }
