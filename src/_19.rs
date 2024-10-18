// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>, // Use 'nxt' instead of 'next' per naming conventions
}

impl ListNode {
    // Creates a new ListNode with the given value
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    // Appends a new node with the given value at the end of the list
    pub fn apd(&mut self, val: i32) {
        let mut cur: &mut ListNode = self;
        while let Some(ref mut nxt) = cur.next {
            cur = nxt;
        }
        cur.next = Some(Box::new(ListNode::new(val)));
    }
}

// Time complexity: O(n), n is the length of the list. We traverse the list twice. Once to calculate the length. A second time to remove the node.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/67118b6f-eaa8-8002-814a-ae5823ac47d5
fn remove_nth_from_end(hed: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // Remove the Nth Node From the End of List
    // Given the head of a singly linked list.
    // Given an integer n.
    // Remove node length-n.
    // Return the head of the processed list.
    // Use a two-pass approach.

    // Initialize variables.
    let mut len: i32 = 0;
    let mut dum = Some(Box::new(ListNode { val: 0, next: hed }));
    let mut cur = &dum;

    // Pass one: calculate list length.
    while let Some(nod) = cur {
        cur = &nod.next;
        len += 1;
    }
    // Adjust by one for dummy node.
    len -= 1;

    // Pass two: move to the node prior to the removal node.
    let mut prv = &mut dum;
    for _ in 0..len - n {
        prv = &mut prv.as_mut().unwrap().next;
    }
    // Get valid node after removal node.
    let nxt = prv.as_mut().unwrap().next.as_mut().unwrap().next.take();
    // Connect valid previous and next nodes.
    prv.as_mut().unwrap().next = nxt;

    // Return head of processed list.
    dum.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_middle_node() {
        // Create list [1, 2, 3, 4, 5]
        let mut lst: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
        lst.as_mut().unwrap().apd(2);
        lst.as_mut().unwrap().apd(3);
        lst.as_mut().unwrap().apd(4);
        lst.as_mut().unwrap().apd(5);

        // Remove 2nd node from end
        let res: Option<Box<ListNode>> = remove_nth_from_end(lst, 2);

        // Expected list is [1, 2, 3, 5]
        let mut exp: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
        exp.as_mut().unwrap().apd(2);
        exp.as_mut().unwrap().apd(3);
        exp.as_mut().unwrap().apd(5);

        assert_eq!(res, exp);
    }

    #[test]
    fn remove_first_node() {
        // Create list [1]
        let lst: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));

        // Remove 1st node from end
        let res: Option<Box<ListNode>> = remove_nth_from_end(lst, 1);

        // Expected list is []
        let exp: Option<Box<ListNode>> = None;

        assert_eq!(res, exp);
    }

    #[test]
    fn remove_last_node() {
        // Create list [1, 2]
        let mut lst: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
        lst.as_mut().unwrap().apd(2);

        // Remove 1st node from end
        let res: Option<Box<ListNode>> = remove_nth_from_end(lst, 1);

        // Expected list is [1]
        let exp: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));

        assert_eq!(res, exp);
    }
}
