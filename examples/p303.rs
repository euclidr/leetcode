struct NumArray {
    sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len()];
        if nums.len() > 0 {
            sums[0] = nums[0];
            for i in 1..nums.len() {
                sums[i] = sums[i - 1] + nums[i];
            }
        }
        Self { sums }
    }
    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (i, mut j) = (i as usize, j as usize);
        if j >= self.sums.len() {
            j = self.sums.len() - 1;
        }
        if i == 0 {
            return self.sums[j];
        }

        self.sums[j] - self.sums[i - 1]
    }
}

fn main() {
    let obj = NumArray::new(vec![1, 2, 3, 4, 5, 6]);
    let ret_1: i32 = obj.sum_range(2, 4);
    println!("{}", ret_1);
}
