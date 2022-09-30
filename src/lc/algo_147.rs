use crate::lc::algo_328::ListNode;

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fake_head = Box::new(ListNode::new(i32::MIN));
    fake_head.next = head;
    let mut fake_head = Some(fake_head);

    let mut cursor1 = fake_head.as_mut().unwrap().next.take();
    while cursor1.is_some() {
        let mut next = cursor1.as_mut().unwrap().next.take();

        let mut cursor2 = &mut fake_head;
        while cursor2.as_ref().unwrap().next.is_some() {
            if cursor2.as_ref().unwrap().next.as_ref().unwrap().val > cursor1.as_ref().unwrap().val {
                let cursor2_next = cursor2.as_mut().unwrap().next.take();
                cursor1.as_mut().unwrap().next = cursor2_next;

                cursor2.as_mut().unwrap().next = cursor1;
                cursor1 = None;
                break;
            }
            cursor2 = &mut cursor2.as_mut().unwrap().next;
        }
        if cursor1.is_some() {
            cursor2.as_mut().unwrap().next = cursor1;
        }
        cursor1 = next;

    }

    fake_head.unwrap().next
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {

    }
}