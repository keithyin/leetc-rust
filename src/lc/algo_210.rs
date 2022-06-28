use std::collections::{HashMap, HashSet};

fn has_circle_210(graph: &HashMap<i32, HashSet<i32>>, cur: i32, visited: &mut HashSet<i32>) -> bool{
    if visited.contains(&cur) {
        return true;
    }
    visited.insert(cur);
    for next_node in graph[&cur].iter() {
        if has_circle_210(graph, *next_node, visited) {
            return true;
        }
    }
    visited.remove(&cur);

    false
}

fn dfs_210(graph: &HashMap<i32, HashSet<i32>>, cur: i32, visited: &mut HashSet<i32>,
           results: &mut Vec<i32>, collected: &mut HashSet<i32>) {
    if visited.contains(&cur) || collected.contains(&cur){
        return;
    }

    visited.insert(cur);
    for next in graph[&cur].iter() {
        dfs_210(graph, *next, visited, results, collected);
    }
    results.push(cur);
    collected.insert(cur);
    visited.remove(&cur);
}

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {

    let mut graph = HashMap::new();
    let mut visited = HashSet::new();
    for course in 0..num_courses {
        graph.insert(course, HashSet::new());
    }
    for pair in prerequisites.iter() {
        if let Some(next) = graph.get_mut(&pair[1]) {
            next.insert(pair[0]);
        }
    }

    for course in 0..num_courses {
        if has_circle_210(&graph, course, &mut visited) {
            return vec![];
        }
    }

    // topology sort
    let mut results = vec![];
    let mut collected = HashSet::new();
    for course in 0..num_courses {
        dfs_210(&graph, course, &mut visited, &mut results, &mut collected);
    }

    results.into_iter().rev().collect()

}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;
    use std::cell::{Ref, RefCell};
    use std::rc::{Rc, Weak};
    use crate::lc::algo_210::find_order;

    #[test]
    fn test_circle_ref() {

        struct Node {
            data: i32,
            next: Option<Rc<RefCell<Node>>>
        }

        impl Drop for Node {
            fn drop(&mut self) {
                println!("drop node {}", self.data);
            }
        }

        let first = Rc::new(RefCell::new(Node{data: 10, next: None}));
        let second = Rc::new(RefCell::new(Node{data: 11, next: None}));

        first.as_ref().borrow_mut().next = Some(Rc::clone(&second));
        second.as_ref().borrow_mut().next = Some(Rc::clone(&first));

    }

    #[test]
    fn test_find_order() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1]];
        println!("{:?}", find_order(num_courses, prerequisites));
    }
}