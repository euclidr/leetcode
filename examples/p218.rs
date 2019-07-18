struct Solution;

impl Solution {
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cell::RefCell;
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;
        use std::rc::Rc;

        #[derive(Debug, Eq, PartialEq)]
        struct HeightItem {
            height: i32,
            invalid: bool,
        }

        impl Ord for HeightItem {
            fn cmp(&self, other: &Self) -> Ordering {
                self.height.cmp(&other.height)
            }
        }

        impl PartialOrd for HeightItem {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        #[derive(Debug, Eq, PartialEq)]
        struct Building {
            left: i32,
            right: i32,
            height: i32,
            item: Rc<RefCell<HeightItem>>,
        }

        impl Building {
            fn new(left: i32, right: i32, height: i32) -> Building {
                Building {
                    left,
                    right,
                    height,
                    item: Rc::new(RefCell::new(HeightItem {
                        height,
                        invalid: false,
                    })),
                }
            }
        }

        impl Ord for Building {
            fn cmp(&self, other: &Self) -> Ordering {
                other
                    .right
                    .cmp(&self.right)
                    .then_with(|| self.height.cmp(&other.height))
            }
        }

        impl PartialOrd for Building {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        fn pop_invalids(heap: &mut BinaryHeap<Rc<RefCell<HeightItem>>>) {
            loop {
                match heap.peek() {
                    Some(item) => {
                        if item.borrow().invalid {
                            heap.pop();
                        } else {
                            return;
                        };
                    }
                    None => return,
                }
            }
        }

        buildings.sort_by(|a, b| a[0].cmp(&b[0]).then_with(|| b[2].cmp(&a[2])));
        let mut height_heap = BinaryHeap::new();
        let mut building_heap = BinaryHeap::<Building>::new();
        let mut i = 0;

        let mut pre_height = 0;
        let mut result: Vec<Vec<i32>> = vec![];
        while i < buildings.len() {
            // pop_invalids(&mut height_heap);
            // println!("----------------");
            // println!("building {:?}", buildings[i]);

            let (left, new_height) = match building_heap.peek() {
                Some(left_most) => {
                    let left;
                    // println!(
                    //     "waiting_left: {}, queue_left: {}",
                    //     buildings[i][0], left_most.right
                    // );
                    if buildings[i][0] <= left_most.right {
                        let building =
                            Building::new(buildings[i][0], buildings[i][1], buildings[i][2]);
                        i += 1;
                        left = building.left;
                        height_heap.push(building.item.clone());
                        building_heap.push(building);
                    } else {
                        let left_most = building_heap.pop().unwrap();
                        left = left_most.right;
                        left_most.item.borrow_mut().invalid = true;
                    }

                    pop_invalids(&mut height_heap);

                    let new_height = match height_heap.peek() {
                        Some(item) => item.borrow().height,
                        None => 0,
                    };
                    (left, new_height)
                }
                None => {
                    let building = Building::new(buildings[i][0], buildings[i][1], buildings[i][2]);
                    i += 1;
                    let (left, new_height) = (building.left, building.height);
                    height_heap.push(building.item.clone());
                    building_heap.push(building);
                    (left, new_height)
                }
            };
            // println!("left: {}, new_height: {}", left, new_height);

            if new_height != pre_height {
                pre_height = new_height;
                match result.last_mut() {
                    Some(r) => {
                        if r[0] == left {
                            r[1] = new_height;
                        } else {
                            result.push(vec![left, new_height]);
                        }
                    }
                    None => result.push(vec![left, new_height]),
                }
            }
        }

        while let Some(building) = building_heap.pop() {
            building.item.borrow_mut().invalid = true;
            pop_invalids(&mut height_heap);
            let new_height = match height_heap.peek() {
                Some(item) => item.borrow().height,
                None => 0,
            };
            let left = building.right;

            if new_height != pre_height {
                pre_height = new_height;
                match result.last_mut() {
                    Some(r) => {
                        if r[0] == left {
                            r[1] = new_height;
                        } else {
                            result.push(vec![left, new_height]);
                        }
                    }
                    None => result.push(vec![left, new_height]),
                }
            }
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ])
    )
}
