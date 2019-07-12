struct Solution;

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::LinkedList;

        if matrix.len() == 0 {
            return vec![];
        }

        let mut result = vec![vec![-1; matrix[0].len()]; matrix.len()];
        let mut queue = LinkedList::new();
        let (width, height) = (matrix[0].len(), matrix.len());
        for row in 0..height {
            for col in 0..width {
                if matrix[row][col] == 0 {
                    result[row][col] = 0;
                    queue.push_back((row*width+col, 0));
                }
            }
        }

        while let Some(item) = queue.pop_front() {
            let (row, col) = (item.0 / width, item.0 % width);
            // top
            if row > 0 && result[row-1][col] == -1 {
                queue.push_back(((row-1)*width+col, item.1+1));
                result[row-1][col] = item.1+1;
            }


            // right
            if col+1 < width && result[row][col+1] == -1 {
                queue.push_back((row*width+col+1, item.1+1));
                result[row][col+1] = item.1+1;
            }

            // bottom
            if row+1 < height && result[row+1][col] == -1 {
                queue.push_back(((row+1)*width+col, item.1+1));
                result[row+1][col] = item.1+1;
            }

            // left
            if col > 0 && result[row][col-1] == -1 {
                queue.push_back((row*width+col-1, item.1+1));
                result[row][col-1] = item.1+1;
            }

        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::update_matrix(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![1, 1, 1],
    ]));
}