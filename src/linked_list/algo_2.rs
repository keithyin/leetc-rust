use std::collections::LinkedList;

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(value: i32) -> Self {
        ListNode {
            val: value,
            next: None,
        }
    }

    pub fn from_vec(values: Vec<i32>) -> Option<Box<Self>> {
        return ListNode::from_vec_core(&values, 0);
    }

    fn from_vec_core(values: &Vec<i32>, i: i32) -> Option<Box<ListNode>>{
        if (i as usize) >= values.len() {
            return None;
        }
        let mut node = Box::new(ListNode::new(values[i as usize]));
        node.next = ListNode::from_vec_core(values, i+1);
        return Some(node);
    }

    pub fn print(&self) {
        print!("{} ", self.val);
        if let Some(ref node) = self.next {
            node.print();
        }
    }
}

pub fn add_two_numbers(node1: &Option<Box<ListNode>>, node2: &Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    if let (&None, &None) = (node1, node2) {
        return if carry == 0 {
            None
        } else {
            Some(Box::new(ListNode::new(carry)))
        };
    }
    let mut value = 0;
    if let Some(node1) = node1 {
        value += node1.val;
    }
    if let Some(node2) = node2 {
        value += node2.val;
    }
    value += carry;
    carry = value / 10;
    value %= 10;

    let mut cur_node = Box::new(ListNode::new(value));
    cur_node.next = add_two_numbers(
        if let Some(node1) = node1 {
            &node1.next
        }else {
            &None
        },
        if let Some(node2) = node2 {
            &node2.next
        } else {
            &None
        }, carry);
    return Some(cur_node);
}

#[cfg(test)]
mod test {
    use crate::linked_list::algo_2::{ListNode, add_two_numbers};

    #[test]
    fn test_list_node() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4]);
        if let Some(head) = head {
            head.print();
        }
    }

    #[test]
    fn test_add_two_numbers() {
        let node1 = ListNode::from_vec(vec![1, 2, 3]);
        let node2 = ListNode::from_vec(vec![1, 9, 6]);
        let res = add_two_numbers(&node1, &node2, 0);
        if let Some(res) = res {
            res.print();
        }
    }

    #[test]
    fn test_box() {
        let mut val = Box::new(10);
        let mut_val = val.as_mut();
        *mut_val = 10;
        *val = 100;

        println!("{}", val);
    }

}