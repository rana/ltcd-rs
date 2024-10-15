#[derive(PartialEq, Eq, Clone, Debug)]
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

// Time complexity: O(max(m, n)), m is the length of the left linked list. n is the length of the right linked list.
// Time complexity: O(1), constant auxilary space used.
// https://chatgpt.com/c/670eba1b-8428-8002-9346-932011871fde
pub fn add_two_numbers(
    mut lft: Option<Box<ListNode>>,
    mut rht: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Given two non-empty linked lists.
    // Linked lists have positive integers in reverse order.
    // Each node is a single digit.
    // Sum the two numbers.
    // Return the sum.

    // Create a dummy head to simplify code and avoid edge cases.
    let mut dmy_head: ListNode = ListNode::new(0);
    // Pointer to the current node in the result list.
    let mut cur: &mut ListNode = &mut dmy_head;
    // Variable to store the carray value.
    let mut cry: i32 = 0;

    // Loop until both lists are exhausted, and no carry remains.
    while lft.is_some() || rht.is_some() || cry != 0 {
        // Initialize the sum with the carry value.
        let mut sum: i32 = cry;

        // Add the value from the left list if available.
        if let Some(node) = lft {
            sum += node.val;
            lft = node.next;
        }

        // Add the value from the right list if available.
        if let Some(node) = rht {
            sum += node.val;
            rht = node.next;
        }

        // Update the carry for the next iteration.
        cry = sum / 10;
        // Create a new node with the digit value.
        let new_node = ListNode::new(sum % 10);
        // Link the new node to the result list.
        cur.next = Some(Box::new(new_node));
        // Move the current pointer to the new node.
        cur = cur.next.as_mut().unwrap();
    }

    // Return the actual head.
    dmy_head.next
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
