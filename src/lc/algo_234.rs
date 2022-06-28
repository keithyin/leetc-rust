use crate::lc::algo_328::ListNode;

pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {

    let mut slow = &mut head as *mut Option<Box<ListNode>>;
    let mut fast = &head;
    loop {
        if let Some(cur_fast) = fast {
            fast = &cur_fast.next;
        } else {
            break;
        }

        if let Some(cur_fast) = fast {
            fast = &cur_fast.next;
        } else {
            break;
        }

        if let Some(ref mut slow_inner) = unsafe{&mut *slow} {
            slow = &mut slow_inner.next as *mut Option<Box<ListNode>>;
        }
    }
    let slow = unsafe{&mut *slow};
    let reverse_begin = slow.take();
    let mut reversed = link_list_reverse(reverse_begin);

    while let (Some(a), Some(b)) = (head, reversed) {
        if a.val != b.val {
            return false;
        }
        head = a.next;
        reversed = b.next;
    }

    true
}

fn link_list_reverse(mut root: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut prev = None;
    let mut curr = root;
    while let Some(ref mut curr_node) = curr {
        let next = curr_node.next.take();
        curr_node.next = prev;
        prev = curr;
        curr = next;
    }
    prev
}

#[cfg(test)]
mod test {
    use crate::lc::algo_234::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        is_palindrome(None);
    }
}