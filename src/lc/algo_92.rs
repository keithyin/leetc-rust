use crate::lc::algo_328::ListNode;

pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let span_len = right - left + 1;

    if span_len == 1 {
        return head;
    }

    let mut fake_head = Some(Box::new(ListNode::new(0)));
    fake_head.as_mut().unwrap().next = head;
    let mut first_cursor = fake_head.as_mut().unwrap();
    let mut counter = 0;
    let mut reverse_node = loop {
        counter += 1;
        if counter == left {
            break first_cursor.next.take();
        }
        first_cursor = first_cursor.next.as_mut().unwrap();
    };
    let mut pre_node = None;
    let mut cur_node = reverse_node;
    for i in 0..span_len {
        let next_node = cur_node.as_mut().unwrap().next.take();
        cur_node.as_mut().unwrap().next = pre_node;
        pre_node = cur_node;
        cur_node = next_node;
    }
    first_cursor.next = pre_node;

    let mut cursor = first_cursor;
    while cursor.next.is_some() {
        cursor = cursor.next.as_mut().unwrap();
    }
    cursor.next = cur_node;

    fake_head.unwrap().next
}