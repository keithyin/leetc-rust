use crate::lc::algo_328::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fake_head = Some(Box::new(ListNode::new(102)));
    let mut cursor = fake_head.as_mut().unwrap();
    let mut pre = 101;

    while let Some(mut cur_node) = head {
        let next_node = cur_node.next.take();
        if pre==cur_node.val || (next_node.is_some() && cur_node.val == next_node.as_ref().unwrap().val) {
            pre = cur_node.val;
            head = next_node;

        } else {
            pre = cur_node.val;
            cursor.next = Some(cur_node);
            head = next_node;
            cursor = cursor.next.as_mut().unwrap();

        }
    }

    fake_head.unwrap().next

}