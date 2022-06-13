use std::collections::vec_deque::VecDeque;

struct MonotonicQueue<'a> {
    ori_value: &'a Vec<i32>,
    queue: VecDeque<usize>,
    window_size: i32,
}

impl<'a> MonotonicQueue<'a> {
    pub fn new(value: &'a Vec<i32>, window_size: i32) -> Self {
        Self {
            ori_value: value,
            queue: VecDeque::new(),
            window_size
        }
    }

    pub fn push(&mut self, idx: usize) {
        while let Some(&idx_in_the_queue) = self.queue.front() {
            if (idx_in_the_queue + self.window_size as usize) <= idx {
                self.queue.pop_front();
            } else {
                break;
            }
        }

        let cur_value = self.ori_value[idx];
        while let Some(&queue_val) = self.queue.back() {
            if self.ori_value[queue_val] <= cur_value {
                self.queue.pop_back();
            } else {
                break;
            }
        }

        self.queue.push_back(idx);
    }

    pub fn max_in_the_window(&self) -> Option<i32> {
        if let Some(&idx) = self.queue.front() {
            Some(self.ori_value[idx])
        } else {
            None
        }
    }

}


pub fn sliding_window_max_val(value: Vec<i32>, k: i32) -> Vec<i32>{
    let mut idx = 0;
    let length = value.len();
    let mut monotonic_queue = MonotonicQueue::new(&value, k);
    let mut result = vec![];
    while idx < length {
        if (idx as i32) < (k-1) {
            monotonic_queue.push(idx);
            idx += 1;
            continue;
        }
        monotonic_queue.push(idx);
        if let Some(value) = monotonic_queue.max_in_the_window() {
            result.push(value);
        }
        idx += 1
    }
    return result;
}

#[cfg(test)]
mod test {
    use crate::queue::algo_239::sliding_window_max_val;

    #[test]
    fn test_sliding_window() {
        let value = vec![1, 2, 3, 4, 3, 2, 5, 1];
        let result = sliding_window_max_val(value, 3);
        println!("{:?}", result);
    }
}