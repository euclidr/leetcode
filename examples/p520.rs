struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() == 1 {
            return true;
        }

        let first = word.chars().nth(0).unwrap() as i32;
        let second = word.chars().nth(1).unwrap() as i32;

        let thres = 'a' as i32;

        if first >= thres && second < thres {
            return false;
        }

        let is_upper = second < thres;

        for c in (&word[2..]).chars() {
            if is_upper {
                if (c as i32) >= thres {
                    return false;
                }
            } else {
                if (c as i32) < thres {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    // let s = "百科全书abcABC";
    // for c in s.chars() {
    //     println!("{}", c as i32);
    // }
    println!("{}", Solution::detect_capital_use("Gooogle".to_string()));
    println!("{}", Solution::detect_capital_use("GooogLe".to_string()));
    println!("{}", Solution::detect_capital_use("GOoogLe".to_string()));
    println!("{}", Solution::detect_capital_use("GO".to_string()));
    println!("{}", Solution::detect_capital_use("gO".to_string()));
}
