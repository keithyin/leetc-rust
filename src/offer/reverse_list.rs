// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}


pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = None;
    let mut curr_node = head;
    while let Some(mut node) = curr_node {
        curr_node = node.next.take();
        node.next = result;
        result = Some(node);
    }
    result
}


#[cfg(test)]
mod test {
    #[test]
    fn test_box_mov() {
        let a = Box::new(10);
        let b = a;
    }
}