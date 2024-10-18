#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// https://chatgpt.com/c/6711a60d-2008-8002-a017-31f54811a4bd
fn delete_duplicates(hed: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Remove Duplicates from Sorted List II
    // Given the head of sorted singly linked list.
    // Remove all nodes with duplicate values.
    // Return the head of the processed list.

    // Initialize variables.
    let mut dum = Box::new(ListNode { val: 0, next: hed });
    let mut prv = &mut dum;

    // Loop through each node.
    while let Some(ref mut cur) = prv.next {
        // Search for duplicate nodes.
        let mut dup_fnd = false;
        while let Some(ref nxt) = cur.next {
            if cur.val == nxt.val {
                dup_fnd = true;
                // Skip over next value.
                cur.next = nxt.next.clone();
            } else {
                break;
            }
        }
        if dup_fnd {
            // Remove the current node.
            prv.next = cur.next.clone();
        } else {
            // Move the pointer forward.
            prv = prv.next.as_mut().unwrap();
        }
    }

    dum.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: [1,2,3,3,4,4,5]
        // Expected Output: [1,2,5]

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let res = delete_duplicates(head);

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));

        assert_eq!(res, expected);
    }

    #[test]
    fn example2() {
        // Input: [1,1,1,2,3]
        // Expected Output: [2,3]

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            })),
        }));

        let res = delete_duplicates(head);

        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        }));

        assert_eq!(res, expected);
    }

    #[test]
    fn empty_list() {
        // Input: []
        // Expected Output: []

        let head = None;
        let res = delete_duplicates(head);
        let expected = None;

        assert_eq!(res, expected);
    }

    #[test]
    fn all_duplicates() {
        // Input: [1,1,1,1]
        // Expected Output: []

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));

        let res = delete_duplicates(head);
        let expected = None;

        assert_eq!(res, expected);
    }

    #[test]
    fn no_duplicates() {
        // Input: [1,2,3,4,5]
        // Expected Output: [1,2,3,4,5]

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let res = delete_duplicates(head.clone());
        assert_eq!(res, head);
    }
}
