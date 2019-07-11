use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        struct Record {
            to: usize,
            time: i32,
        }

        impl Ord for Record {
            fn cmp(&self, other: &Record) -> Ordering {
                other.time.cmp(&self.time)
            }
        }

        impl PartialOrd for Record {
            fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let k = k - 1;

        let mut graph = vec![vec![]; n as usize];
        for line in times.iter() {
            graph[(line[0] - 1) as usize].push(((line[1] - 1) as usize, line[2]))
        }

        for node in graph.iter_mut() {
            node.sort_by(|a, b| a.1.cmp(&b.1));
        }

        let mut marks = vec![false; n as usize];
        let mut latencies = vec![None; n as usize];
        let mut queue = BinaryHeap::new();
        queue.push(Record {
            to: k as usize,
            time: 0,
        });
        while let Some(node) = queue.pop() {
            // println!("queue after: {:?}", queue);
            if marks[node.to] {
                continue;
            }
            marks[node.to] = true;
            latencies[node.to] = Some(node.time);
            for line in graph[node.to].iter() {
                let new_time = node.time + line.1;
                match latencies[line.0] {
                    Some(time) => {
                        if time > new_time {
                            latencies[line.0] = Some(new_time);
                            queue.push(Record {
                                to: line.0,
                                time: new_time,
                            });
                        }
                    }
                    None => {
                        latencies[line.0] = Some(new_time);
                        queue.push(Record {
                            to: line.0,
                            time: new_time,
                        });
                    }
                }
            }
            // println!("queue: {:?}", queue);
        }
        // println!("{:?}", latencies);

        let mut max = match latencies[0] {
            Some(time) => time,
            None => return -1,
        };

        for latency in latencies.iter() {
            match latency {
                Some(time) => {
                    if *time > max {
                        max = *time;
                    }
                }
                None => return -1,
            }
        }

        max
    }
}

fn main() {
    let test_case = "[[4,2,76],[1,3,79],[3,1,81],[4,3,30],[2,1,47],[1,5,61],[1,4,99],[3,4,68],[3,5,46],[4,1,6],[5,4,7],[5,3,44],[4,5,19],[2,3,13],[3,2,18],[1,2,0],[5,1,25],[2,5,58],[2,4,77],[5,2,74]]";
    let test_case = &test_case[2..test_case.len() - 2];
    let mut latencies = vec![];
    for arr in test_case.split("],[") {
        let mut route = vec![];
        for stop in arr.split(',') {
            let stop: i32 = stop.parse().unwrap();
            route.push(stop);
        }
        latencies.push(route);
    }
    // println!(
    //     "{}",
    //     Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1],], 4, 2)
    // );
    println!("{}", Solution::network_delay_time(latencies, 5, 3));
}
