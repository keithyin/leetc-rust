// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    core(&head, &mut result);
    result
}

pub fn core(head: &Option<Box<ListNode>>, result: &mut Vec<i32>) {
    if let Some(ref val) = head {
        core(&val.next, result);
        result.push(val.val);
    }
    return;
}

#[cfg(test)]
mod test {
    use crate::offer::reverse_print::{ListNode, reverse_print};

    #[test]
    fn test_reverse_print() {
        let mut head = ListNode::new(1);
        head.next = Some(Box::new(ListNode::new(2)));
        let result = reverse_print(Some(Box::new(head)));
        println!("{:?}", result);
    }

    fn push_val(a: &mut Vec<i32>) {
        a.push(3);
    }

    #[test]
    fn test_mut_ref() {
        let mut a = vec![1, 2, 3];
        let a_ref = &mut a;
        push_val(a_ref);
        a_ref.push(2);
    }
}