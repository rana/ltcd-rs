// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn add_two_numbers(
    mut lft: Option<Box<ListNode>>,
    mut rht: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Add Two Numbers
    // Given two linked lists.
    // Lists represent positive integers.
    // Nodes are integers stored in reverse order.
    // Add the two numbers.
    // Return the sum as a linked list.
    // Notice that each linked list may have a different length.

    // Create a dummy head to help.
    let mut dum = ListNode::new(0);
    // Create a pointer to the current node.
    let mut cur = &mut dum;
    // Create a carry variable to assist the calculation.
    let mut cry: i32 = 0;

    // Loop through each linked list and carry.
    while lft.is_some() || rht.is_some() || cry != 0 {
        // Initialize sum with carry.
        let mut sum: i32 = cry;

        // Evaluate left node.
        if let Some(node) = lft {
            // Add left to sum.
            sum += node.val;
            // Move left node forward.
            lft = node.next;
        }

        // Evaluate right node.
        if let Some(node) = rht {
            // Add right to sum.
            sum += node.val;
            // Move right node forward.
            rht = node.next;
        }

        // Prepare carry value for next iteration.
        cry = sum / 10;
        // Create new node.
        let new_node = Some(Box::new(ListNode::new(sum % 10)));
        // Attach new node to result.
        cur.next = new_node;
        // Move current result forward.
        cur = cur.next.as_mut().unwrap();
    }

    // Return result.
    dum.next
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a linked list from a vector.
    fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &val in v.iter().rev() {
            let mut node: Box<ListNode> = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    // Helper function to convert a linked list to a vector.
    fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        while let Some(n) = node {
            v.push(n.val);
            node = n.next;
        }
        v
    }

    #[test]
    fn test_example1() {
        let lft: Option<Box<ListNode>> = vec_to_list(vec![2, 4, 3]);
        let rht: Option<Box<ListNode>> = vec_to_list(vec![5, 6, 4]);
        let res: Option<Box<ListNode>> = add_two_numbers(lft, rht);
        assert_eq!(list_to_vec(res), vec![7, 0, 8]);
    }

    #[test]
    fn test_example2() {
        let lft: Option<Box<ListNode>> = vec_to_list(vec![0]);
        let rht: Option<Box<ListNode>> = vec_to_list(vec![0]);
        let res: Option<Box<ListNode>> = add_two_numbers(lft, rht);
        assert_eq!(list_to_vec(res), vec![0]);
    }

    #[test]
    fn test_different_lengths() {
        let lft: Option<Box<ListNode>> = vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let rht: Option<Box<ListNode>> = vec_to_list(vec![9, 9, 9, 9]);
        let res: Option<Box<ListNode>> = add_two_numbers(lft, rht);
        assert_eq!(list_to_vec(res), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}
