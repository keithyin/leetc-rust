use crate::lc::algo_328::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fake_head = Box::new(ListNode::new(0));
    fake_head.next = head;

    let mut cur = fake_head.next.take();
    let mut pre = &mut fake_head;

    while let Some(ref mut cur_node) = cur {
        if cur_node.next.is_none() {

            pre.next = cur;

            break;
        }

        let mut next = cur_node.next.take().unwrap();
        let next_next = next.next.take();
        cur_node.next = next_next;
        next.next = cur;
        pre.next = Some(next);

        pre = if let Some(ref mut node) = pre.next {
            if let Some(ref mut node) = node.next {
                node
            } else {
                panic!();
            }
        } else {
            panic!();
        };

        cur = pre.next.take();

    }
    fake_head.next
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {
        println!("hello");
    }
}