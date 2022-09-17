use crate::lc::algo_328::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut pre_val = head.as_ref().unwrap().val;
    let mut cur = head.as_mut().unwrap().next.take();
    let mut cursor = head.as_mut().unwrap();
    while let Some(mut cur_node) = cur {
        cur = cur_node.next.take();
        if pre_val != cur_node.val {
            pre_val = cur_node.val;
            cursor.next = Some(cur_node);
            cursor = cursor.next.as_mut().unwrap();
        }
    }
    head
}