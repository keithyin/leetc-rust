use crate::lc::algo_328::ListNode;

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut less_head = Some(Box::new(ListNode::new(0)));
    let mut greater_equal_head = Some(Box::new(ListNode::new(0)));
    let mut less_cursor = less_head.as_mut().unwrap();
    let mut greater_equal_cursor = greater_equal_head.as_mut().unwrap();

    while let Some(mut cur_node) = head {
        head = cur_node.next.take();

        if cur_node.val < x {
            less_cursor.next = Some(cur_node);
            less_cursor = less_cursor.next.as_mut().unwrap();
        } else {
            greater_equal_cursor.next = Some(cur_node);
            greater_equal_cursor = greater_equal_cursor.next.as_mut().unwrap();
        }
    }
    less_cursor.next = greater_equal_head.as_mut().unwrap().next.take();
    less_head.unwrap().next
}