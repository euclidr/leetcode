use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut relations = HashMap::new();
        for (equation, value) in equations.iter().zip(values.iter()) {
            Solution::add_relationship(&mut relations, &equation[0], &equation[1], *value);
            Solution::add_relationship(&mut relations, &equation[1], &equation[0], 1.0 / (*value));
        }

        let mut result = vec![];
        for query in queries.iter() {
            let f = match Solution::solve(&relations, &query[0], &query[1]) {
                Some(f) => f,
                None => -1.0,
            };
            result.push(f);
        }

        result
    }

    fn solve(
        relations: &HashMap<String, Vec<(String, f64)>>,
        start: &String,
        end: &String,
    ) -> Option<f64> {
        let mut checked = HashSet::new();
        let mut stack = vec![];
        stack.push((start, 1.0));

        while let Some((node, result)) = stack.pop() {
            checked.insert(node.clone());
            match relations.get(node) {
                Some(lst) => {
                    for &(ref k, v) in lst.iter() {
                        if k == &end[..] {
                            return Some(result * v);
                        }

                        if checked.contains(k) {
                            continue;
                        }

                        stack.push((k, result * v));
                    }
                }
                None => break,
            }
        }

        None
    }

    fn add_relationship(
        mapper: &mut HashMap<String, Vec<(String, f64)>>,
        a: &String,
        b: &String,
        val: f64,
    ) {
        match mapper.get_mut(a) {
            Some(rel) => match rel.iter().find(|v| v.0 == &b[..]) {
                Some(_) => (),
                None => rel.push((b.clone(), val)),
            },
            None => {
                mapper.insert(a.clone(), vec![(b.clone(), val)]);
            }
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::calc_equation(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()],
            ],
            vec![2.0, 3.0],
            vec![
                vec!["a".to_string(), "c".to_string()],
                vec!["b".to_string(), "a".to_string()],
                vec!["a".to_string(), "e".to_string()],
                vec!["a".to_string(), "a".to_string()],
                vec!["x".to_string(), "x".to_string()],
            ]
        )
    );
}
