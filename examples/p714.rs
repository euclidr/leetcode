struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut start = prices[0];
        let mut pre_top = start;
        let mut profit = 0;
        for price in prices.iter() {
            if *price > pre_top {
                pre_top = *price;
                continue;
            }

            if pre_top - *price > fee {
                // sell previous
                if pre_top - start > fee {
                    profit += pre_top - start - fee;
                }
                start = *price;
                pre_top = *price;
                continue;
            }

            if *price < start {
                start = *price;
                pre_top = *price;
            }
        }

        if pre_top - start > fee {
            profit += pre_top - start - fee;
        }

        profit
    }
}

fn main() {

    println!("{}", Solution::max_profit(vec![1,3,2,8,4,9], 2));

}