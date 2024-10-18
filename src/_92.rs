#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub nxt: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { nxt: None, val }
    }
}

// Time complexity: O(n), n is the length of the list. We traverse the list up to two times.
// Space complexity: O(1), constant additional space used. We perform in-place reversal.
// https://chatgpt.com/c/67115317-3e44-8002-899e-798bb95ab861
pub fn reverse_between(hd: Option<Box<ListNode>>, lft: i32, rht: i32) -> Option<Box<ListNode>> {
    // Reverse Linked List II
    // Given the head of a singly linked list.
    // Given two integers lft, rht: lft <= rht.
    // Reverse nodes from lft to rht.
    // Return the head of the processed list.
    // Use a dummy node technique.
    // Use in-place reversal.

    // Check input minimum edge case.
    // No nodes; or, no reversal.
    if hd.is_none() || lft == rht {
        return hd;
    }

    // Initialize vairables.
    let mut dum: Box<ListNode> = Box::new(ListNode { val: 0, nxt: hd });
    let mut prv: &mut Box<ListNode> = &mut dum;
    let mut idx: i32 = 1;

    // Move prv pointer to just before reversal start.
    while idx < lft {
        prv = prv.nxt.as_mut().unwrap();
        idx += 1;
    }

    // Setup reversal variables.
    let mut cur: Option<Box<ListNode>> = prv.nxt.take();
    let mut nxt: Option<Box<ListNode>> = None;

    // Reverse node between lft and rht.
    for _ in lft..=rht {
        let tmp: Option<Box<ListNode>> = cur.as_mut().unwrap().nxt.take();
        cur.as_mut().unwrap().nxt = nxt;
        nxt = cur;
        cur = tmp;
    }

    // Connect the front of the list.
    prv.nxt = nxt;

    // Move the prv pointer to the end of the reversal.
    while prv.nxt.is_some() {
        prv = prv.nxt.as_mut().unwrap();
    }

    // Connect the back of the list.
    prv.nxt = cur;

    // Return the processed list.
    dum.nxt
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to convert a vector to a linked list
    fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for val in v.into_iter().rev() {
            let mut node: Box<ListNode> = Box::new(ListNode::new(val));
            node.nxt = head;
            head = Some(node);
        }
        head
    }

    // Helper function to convert a linked list to a vector
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        while let Some(node) = head {
            v.push(node.val);
            head = node.nxt;
        }
        v
    }

    #[test]
    fn example1() {
        let lst: Option<Box<ListNode>> = vec_to_list(vec![1, 2, 3, 4, 5]);
        let res: Option<Box<ListNode>> = reverse_between(lst, 2, 4);
        let res_vec: Vec<i32> = list_to_vec(res);
        assert_eq!(res_vec, vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn example2() {
        let lst: Option<Box<ListNode>> = vec_to_list(vec![5]);
        let res: Option<Box<ListNode>> = reverse_between(lst, 1, 1);
        let res_vec: Vec<i32> = list_to_vec(res);
        assert_eq!(res_vec, vec![5]);
    }

    #[test]
    fn single_node() {
        let lst: Option<Box<ListNode>> = vec_to_list(vec![1]);
        let res: Option<Box<ListNode>> = reverse_between(lst, 1, 1);
        let res_vec: Vec<i32> = list_to_vec(res);
        assert_eq!(res_vec, vec![1]);
    }

    #[test]
    fn full_reverse() {
        let lst: Option<Box<ListNode>> = vec_to_list(vec![1, 2, 3, 4, 5]);
        let res: Option<Box<ListNode>> = reverse_between(lst, 1, 5);
        let res_vec: Vec<i32> = list_to_vec(res);
        assert_eq!(res_vec, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn no_change() {
        let lst: Option<Box<ListNode>> = vec_to_list(vec![1, 2, 3, 4, 5]);
        let res: Option<Box<ListNode>> = reverse_between(lst, 3, 3);
        let res_vec: Vec<i32> = list_to_vec(res);
        assert_eq!(res_vec, vec![1, 2, 3, 4, 5]);
    }
}
