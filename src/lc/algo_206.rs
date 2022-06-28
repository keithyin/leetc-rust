use crate::lc::algo_328::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut prev_node = None;
    let mut cur_node = head;
    while let Some(ref mut cur_node_inner) = cur_node {
        let next_node = cur_node_inner.next.take();
        cur_node_inner.next = prev_node;

        prev_node = cur_node;
        cur_node = next_node;
    }
    prev_node
}