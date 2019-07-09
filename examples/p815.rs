use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;

struct Solution;

#[derive(Debug, Clone)]
struct Node {
    is_start: bool,
    is_destination: bool,
    depth: Option<i32>,
    connects: Vec<usize>,
}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
        if s == t {
            return 0;
        }
        // bus stop
        let mut table = HashMap::<usize, Vec<usize>>::new();
        // bus connection
        let mut graph = vec![];
        for _ in 0..routes.len() {
            graph.push(Rc::new(RefCell::new(Node {
                is_start: false,
                is_destination: false,
                depth: None,
                connects: vec![],
            })));
        }

        for (num, route) in routes.iter().enumerate() {
            for stop in route.iter() {
                let stop = *stop as usize;
                match table.get_mut(&stop) {
                    Some(buses) => {
                        buses.push(num);
                    }
                    None => {
                        table.insert(stop, vec![num]);
                    }
                }
            }
        }

        let mut starts = vec![];

        match table.get(&(s as usize)) {
            Some(buses) => {
                for bus in buses.iter() {
                    graph[*bus].borrow_mut().is_start = true;
                    starts.push(*bus);
                }
            }
            None => return -1,
        }

        match table.get(&(t as usize)) {
            Some(buses) => {
                for bus in buses.iter() {
                    graph[*bus].borrow_mut().is_destination = true;
                }
            }
            None => return -1,
        }

        for (_, buses) in table.iter() {
            for i in 0..buses.len() {
                for j in i..buses.len() {
                    let bus_a = buses[i];
                    let bus_b = buses[j];
                    let mut dup = false;
                    for connect_bus in graph[bus_a].borrow().connects.iter() {
                        if bus_b == *connect_bus {
                            dup = true;
                            break;
                        }
                    }
                    if dup {
                        continue;
                    }

                    graph[bus_a].borrow_mut().connects.push(bus_b);
                    graph[bus_b].borrow_mut().connects.push(bus_a);
                }
            }
        }

        // println!("{:?}", graph);

        let mut max_depth = None;
        for start in starts.iter() {
            match Solution::bfs(&mut graph, *start, max_depth) {
                Some(md) => {
                    max_depth = Some(md);
                }
                None => (),
            }
        }

        match max_depth {
            Some(md) => md,
            None => -1,
        }
    }

    fn bfs(
        graph: &mut Vec<Rc<RefCell<Node>>>,
        start: usize,
        max_depth: Option<i32>,
    ) -> Option<i32> {
        if let Some(md) = max_depth {
            if md == 1 {
                return Some(1);
            }
        };

        graph[start].borrow_mut().depth = Some(1);
        let mut queue = LinkedList::new();
        queue.push_back(graph[start].clone());

        let mut result = None;
        // println!("prev max: {:?}", max_depth);

        while let Some(node) = queue.pop_front() {
            if node.borrow().is_destination {
                result = node.borrow().depth;
                // println!("depth got: {:?}", result);
                break;
            }

            let next_depth = node.borrow().depth.unwrap() + 1;

            if let Some(md) = max_depth {
                if next_depth == md {
                    continue;
                }
            }

            for connected in &node.borrow().connects {
                if graph[*connected].borrow().is_start {
                    continue;
                }
                if let Some(_) = graph[*connected].borrow().depth {
                    continue;
                }

                graph[*connected].borrow_mut().depth = Some(next_depth);
                queue.push_back(graph[*connected].clone());
            }
        }

        for node in graph.iter_mut() {
            node.borrow_mut().depth = None;
        }

        result
    }
}

fn main() {
    // println!(
    //     "{}",
    //     Solution::num_buses_to_destination(vec![vec![14,16,23,25,30,43,44,51,56,69,70,79,85,86,90,91,92,100,117,129,139,141,144,157,159,171,174,176,182,184,196,204], vec![3, 6, 7]], 44, 139)
    // );

    let test_case = "[[3,8,17,21,34,36,43,47,54,58,69,82,93,95,97,102,106,108,112,114,119,126,131,132,136,150,159,160,166,176,182,188,194],[12,27,33,67,77,86,101,144,176,190,192],[12,14,24,36,44,47,59,62,71,74,77,80,100,113,114,117,187,193,195],[2,4,30,40,41,46,67,77,85,86,101,106,135,136,148,156,169,171,186,193],[0,4,11,14,18,21,27,37,38,44,53,65,75,93,94,98,101,114,117,130,131,132,141,144,145,167,185,188],[7,21,23,31,63,82,100,106,112,114,163],[16,17,29,41,58,62,72,74,94,102,106,120,151,152,161,163,195],[5,7,11,12,26,27,42,50,68,72,75,76,86,89,92,100,102,111,113,121,129,133,138,143,145,150,168,169,175,184,185,188,195,202],[2,11,13,20,25,28,30,35,40,42,43,67,75,77,80,81,83,100,101,102,110,116,134,138,139,140,142,150,153,159,167,174,176,178,184],[13,22,78,93,120,151,178,189],[16,21,38,40,79,87,88,96,114,134,145,155,157,159,165,201],[4,12,28,30,35,37,45,46,65,70,72,75,86,103,116,137,157,163,166,169,196,197,200],[22,24,29,34,35,38,42,48,68,75,81,99,107,118,123,140,141,155,158,160,167,177,180,182,185,190,195,197,202],[23],[2,26,74,111,113,134,149,150,182,188],[0,15,35,63,68,79,80,85,92,104,138,141,146,193,194,198],[6,7,11,13,20,23,29,31,32,37,46,47,49,70,73,75,80,108,112,123,127,131,132,139,144,149,150,176,179,193,194,196,197],[132],[16,20,22,24,35,40,43,55,65,78,80,105,113,119,137,146,158,165,184,190,198],[0,1,6,11,12,15,22,24,25,33,35,39,65,66,73,76,77,81,83,97,98,99,100,101,109,111,120,131,143,151,173,184,190,192],[0,1,9,10,22,24,35,39,48,53,56,58,69,73,78,104,116,117,120,129,131,135,139,151,153,163,166,169,170,179,187,193],[1,22,30,31,32,48,50,53,57,59,61,65,71,73,76,78,80,96,98,106,110,111,113,117,131,132,134,135,137,140,152,160,170,183,186,191,200],[12,13,22,30,35,46,53,67,86,92,95,98,100,101,104,106,117,133,141,146,149,158,159,163,164,167,181,189,190,199,200],[6,65,155,157,171,177,184],[5,16,31,47,57,69,77,78,82,97,115,146,172,173,190],[41,54,91],[3,46,68,81,92,96,103,104,111,121,136,147,157,171,190,193,198],[22,48,94,126,138,139],[194],[4,8,20,22,38,41,45,46,54,55,61,79,80,85,90,93,115,128,132,135,139,140,150,158,159,166,168,169,179,181,182,190],[1,2,16,17,20,28,37,41,44,45,54,55,56,59,68,72,74,75,88,100,103,126,159,166,172,175,187,188,191],[15,34,48,52,69,87,93,105,147,190,204],[9,22,85,124,141,176,193],[17,22,28,33,34,36,43,69,70,72,81,103,111,128,129,142,154,155,156,157,172,178,183,190,191,200],[14,16,23,25,30,43,44,51,56,69,70,79,85,86,90,91,92,100,117,129,139,141,144,157,159,171,174,176,182,184,196,204],[16,45,52,56,79,122,132,154,162,163,172,197],[0,7,30,37,38,41,45,47,57,67,91,92,105,110,146,147,150,153,158,170,174,176,181,182,189,193],[2,3,13,22,25,38,39,40,43,53,56,61,66,69,76,78,82,85,86,113,115,118,132,141,147,152,156,157,162,184,199,203],[7,8,17,19,38,44,54,73,74,77,82,85,88,91,94,100,101,102,104,108,112,122,125,128,135,147,156,157,167,173,184,190,196,202],[29,78,125],[35,36,38,105,129,132,139,154,189,202]]";
    let test_case = &test_case[2..test_case.len() - 2];
    let mut routes = vec![];
    for arr in test_case.split("],[") {
        let mut route = vec![];
        for stop in arr.split(',') {
            let stop: i32 = stop.parse().unwrap();
            route.push(stop);
        }
        routes.push(route);
    }
    // println!("{:?}", routes);
    println!("{:?}", Solution::num_buses_to_destination(routes, 44, 139));
}
