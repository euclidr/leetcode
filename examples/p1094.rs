use std::cmp::Ordering;
struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        #[derive(Debug, Eq, PartialEq)]
        struct Point {
            location: i32,
            change: i32,
        };

        impl Ord for Point {
            fn cmp(&self, other: &Point) -> Ordering {
                self.location.cmp(&other.location)
            }
        }

        impl PartialOrd for Point {
            fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut road: Vec<Point> = vec![];

        for trip in trips.iter() {
            let (passengers, start_loc, end_loc) = (trip[0], trip[1], trip[2]);
            match road.binary_search(&Point{location: start_loc, change: 0}) {
                Ok(i) => {
                    road[i].change = road[i].change + passengers;
                },
                Err(i) => {
                    road.insert(i, Point{location:start_loc, change: passengers});
                }
            }

            match road.binary_search(&Point{location: end_loc, change: 0}) {
                Ok(i) => {
                    road[i].change = road[i].change - passengers;
                },
                Err(i) => {
                    road.insert(i, Point{location:end_loc, change: -passengers});
                }
            }
            // println!("{:?}", road);
        }

        let mut left = capacity;

        for point in road.iter() {
            left = left - point.change;
            if left < 0 {
                return false;
            }
        }
    
        true
    }
}

fn main() {
    let result = Solution::car_pooling(
        vec![vec![4, 5, 6], vec![6, 4, 7], vec![4, 3, 5], vec![2, 3, 5]],
        13,
    );
    println!("{}", result);
}

// Complicate and wrong answer
// impl Solution {
//     pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
//         #[derive(Debug)]
//         struct Point {
//             location: i32,
//             capacity: i32,
//         };

//         let mut road: Vec<Point> = vec![];

//         for trip in trips.iter() {
//             let (mut start_idx, mut end_idx) = (Some(0), Some(0));
//             let (passengers, start_loc, end_loc) = (trip[0], trip[1], trip[2]);
//             let mut start_capacity = capacity - passengers;
//             let mut end_capacity = capacity;
//             let mut pre_capacity = capacity;
//             for (idx, point) in road.iter_mut().enumerate() {
//                 if point.location > end_loc {
//                     break;
//                 }

//                 if point.location < start_loc {
//                     start_idx = Some(idx + 1);
//                     end_idx = Some(idx + 1);
//                     pre_capacity = point.capacity;
//                     continue;
//                 }
//                 point.capacity = point.capacity - passengers;

//                 if point.capacity < 0 && point.location != end_loc {
//                     return false;
//                 }

//                 if point.location == start_loc {
//                     start_idx = None;
//                 }

//                 if let Some(tmp_idx) = start_idx {
//                     if tmp_idx == idx {
//                         start_capacity = pre_capacity - passengers;
//                         if start_capacity < 0 {
//                             return false;
//                         }
//                     }
//                 }

//                 end_idx = Some(idx + 1);

//                 if point.location == end_loc {
//                     point.capacity = point.capacity + passengers;
//                     end_idx = None;
//                 } else {
//                     end_capacity = point.capacity + passengers;
//                 }
//             }
//             match end_idx {
//                 Some(idx) => {
//                     road.splice(
//                         idx..idx,
//                         vec![Point {
//                             capacity: end_capacity,
//                             location: end_loc,
//                         }],
//                     );
//                 }
//                 None => (),
//             };
//             match start_idx {
//                 Some(idx) => {
//                     road.splice(
//                         idx..idx,
//                         vec![Point {
//                             capacity: start_capacity,
//                             location: start_loc,
//                         }],
//                     );
//                 }
//                 None => (),
//             };
//             println!("{:?}", road);
//         }

//         true
//     }
// }