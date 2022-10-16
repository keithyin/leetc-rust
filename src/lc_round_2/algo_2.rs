use crate::lc::algo_328::ListNode;

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fake_head = Box::new(ListNode::new(0));
    let mut cursor = &mut fake_head;
    let mut pre_node =None;
    let mut carry = 0;

    while l1.is_some() && l2.is_some() {
        let mut l1_v = l1.unwrap();
        let mut l2_v = l2.unwrap();
        l1 = l1_v.next.take();
        l2 = l2_v.next.take();

        let mut cur_val = l1_v.val + l2_v.val + carry;
        carry = cur_val / 10;
        cur_val = cur_val % 10;
        let mut cur_node = Box::new(ListNode::new(cur_val));
        // cur_node.next = pre_node;
        // pre_node = Some(cur_node);
        cursor.next = Some(cur_node);
        cursor = cursor.next.as_mut().unwrap();

    }

    while l1.is_some(){
        let mut v = l1.unwrap();
        l1 = v.next.take();
        let mut cur_val = v.val  + carry;
        carry = cur_val / 10;
        cur_val = cur_val % 10;
        let mut cur_node = Box::new(ListNode::new(cur_val));
        // cur_node.next = pre_node;
        // pre_node = Some(cur_node);
        cursor.next = Some(cur_node);
        cursor = cursor.next.as_mut().unwrap();
    }

    while l2.is_some() {
        let mut v = l2.unwrap();
        l2 = v.next.take();
        let mut cur_val = v.val  + carry;
        carry = cur_val / 10;
        cur_val = cur_val % 10;
        let mut cur_node = Box::new(ListNode::new(cur_val));
        // cur_node.next = pre_node;
        // pre_node = Some(cur_node);
        cursor.next = Some(cur_node);
        cursor = cursor.next.as_mut().unwrap();
    }

    if carry > 0 {
        let mut cur_node = Box::new(ListNode::new(carry));
        // cur_node.next = pre_node;
        // pre_node = Some(cur_node);
        cursor.next = Some(cur_node);
        cursor = cursor.next.as_mut().unwrap();
    }

    pre_node
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {

    }
}