
// Definition for singly-linked list.
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

// Time complexity: O(m + n), m is the length of the left linked list. n is the length of the right linked list. We traverse each list once.
// Space complexity: O(1) additional space, excluding output.
// https://chatgpt.com/c/670ec5cc-ca58-8002-83d4-ab59fb7db6c5
pub fn merge_two_lists(
    mut lft: Option<Box<ListNode>>,
    mut rht: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Merge Two Sorted Lists
    // Given two linked lists heads.
    // Lists are in ascending sort order.
    // Merge the two lists into one.
    // Condition:
    //  * Merge linked list is in ascending sort order.
    // Return the head of the merged linked list.

    // Use a dummy head.
    let mut dum = ListNode::new(0);
    // Create a pointer to the current result node.
    let mut cur = &mut dum;

    // Loop through each linked list.
    while lft.is_some() && rht.is_some() {
        // Get left and right node.
        let lft_node = lft.as_mut().unwrap();
        let rht_node = rht.as_mut().unwrap();

        // Compare nodes for merging.
        // Ascending sort order.
        if lft_node.val <= rht_node.val {
            // Get the next left node.
            let nxt = lft_node.next.take();
            // Merge the left node to the result.
            cur.next = lft.take();
            // Move the left node forward.
            lft = nxt;
        } else {
            // Get next right node.
            let nxt = rht_node.next.take();
            // Merge right node to the result.
            cur.next = rht.take();
            // Move the right node forward.
            rht = nxt;
        }

        // Move the current node forward.
        cur = cur.next.as_mut().unwrap();
    }

    // Attach an remaining nodes.
    // Occurs when lists are uneven.
    cur.next = if lft.is_some() { lft } else { rht };

    dum.next
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to convert a vector to a linked list
    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &val in vec.iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }

    // Helper function to convert a linked list to a vector
    fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(n) = node {
            vec.push(n.val);
            node = n.next;
        }
        vec
    }

    #[test]
    fn merge_both_non_empty() {
        let lst1 = vec_to_list(vec![1, 2, 4]);
        let lst2 = vec_to_list(vec![1, 3, 4]);
        let merged = merge_two_lists(lst1, lst2);
        let res = list_to_vec(merged);
        assert_eq!(res, vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn merge_one_empty() {
        let lst1 = vec_to_list(vec![]);
        let lst2 = vec_to_list(vec![0]);
        let merged = merge_two_lists(lst1, lst2);
        let res = list_to_vec(merged);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn merge_both_empty() {
        let lst1 = vec_to_list(vec![]);
        let lst2 = vec_to_list(vec![]);
        let merged = merge_two_lists(lst1, lst2);
        let res = list_to_vec(merged);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn merge_negative_values() {
        let lst1 = vec_to_list(vec![-3, -1, 2]);
        let lst2 = vec_to_list(vec![-2, 0, 1]);
        let merged = merge_two_lists(lst1, lst2);
        let res = list_to_vec(merged);
        assert_eq!(res, vec![-3, -2, -1, 0, 1, 2]);
    }
}
