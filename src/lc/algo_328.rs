
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_fake_head = Some(Box::new(ListNode::new(0)));
        let mut even_fake_head = Some(Box::new(ListNode::new(0)));

        let mut odd_fake_head_cursor = &mut odd_fake_head;
        let mut even_fake_head_cursor = &mut even_fake_head;


        let mut i = 1;
        let mut ori_cursor = head;
        while let Some(mut node) = ori_cursor{
            ori_cursor = node.next.take();
            if i % 2 != 0 {
                if let Some(odd_cursor) = odd_fake_head_cursor {
                    odd_cursor.next = Some(node);
                    odd_fake_head_cursor = &mut odd_cursor.next;
                }
            } else {
                if let Some(even_cursor) = even_fake_head_cursor {
                    even_cursor.next = Some(node);
                    even_fake_head_cursor = &mut even_cursor.next;
                }
            }
            i += 1;
        }

        if let Some(node2) = even_fake_head.unwrap().next.take() {
            odd_fake_head_cursor.as_mut().unwrap().next = Some(node2);

        }
        odd_fake_head.unwrap().next
    }
}