
trait Priority {
    fn priority(&self) -> i64;
}

#[derive(Debug)]
struct PriorityQueue<T: Priority> {
    values: Vec<T>
}

impl<T: Priority> PriorityQueue<T> {
    fn new(default: T) -> Self {
        PriorityQueue{values: vec![default]}
    }
    fn push(&mut self, val: T) {
        self.values.push(val);
        let mut i = self.values.len()-1;
        while i > 1 {
            let father_idx = i / 2;
            if self.values[father_idx].priority() < self.values[i].priority() {
                self.values.swap(father_idx, i);
            } else {
                break;
            }
            i = father_idx;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.values.len() < 2 {
            return None;
        }
        let last_idx = self.values.len() - 1;
        self.values.swap(1, last_idx);

        // do sink
        let mut cur_i = 1;
        while cur_i < last_idx {
            let left_child = 2 * cur_i;
            let right_child = left_child + 1;
            if left_child >= last_idx {
                break;
            }
            let mut child_max_priority_idx = left_child;
            if right_child < last_idx {
                if self.values[right_child].priority() > self.values[left_child].priority() {
                    child_max_priority_idx = right_child;
                }
            }

            if self.values[cur_i].priority() > self.values[child_max_priority_idx].priority() {
                break;
            }

            self.values.swap(cur_i, child_max_priority_idx);
            cur_i = child_max_priority_idx;
        }

        self.values.pop()
    }
}




#[cfg(test)]
mod test {
    use crate::queue::priority_queue::{Priority, PriorityQueue};

    #[test]
    fn test_demo() {
        #[derive(Debug)]
        struct Node {
            i: i32
        }
        impl Node {
            fn new(i: i32) -> Self {
                Node{i}
            }
        }

        impl Priority for Node {
            fn priority(&self) -> i64 {
                self.i as i64
            }
        }

        let mut pq = PriorityQueue::new(Node::new(-1));
        pq.push(Node::new(2));
        pq.push(Node::new(4));
        pq.push(Node::new(7));
        pq.push(Node::new(1));
        pq.push(Node::new(3));

        eprintln!("{:?}", pq);
        pq.pop();
        eprintln!("{:?}", pq);


    }
}