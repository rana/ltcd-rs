#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub nxt: Option<Box<ListNode>>,
}

// Time complexity: O(n), n is the length of the list. We traverse the list up to two times.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/6711a60d-2008-8002-a017-31f54811a4bd
fn delete_duplicates(hed: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Remove Duplicate from Sorted List II
    // Given the head of a sorted singly linked list.
    // Remove all nodes which are duplicates.
    // Return the head of the sorted linked list.

    // Initialize variables.
    let mut dum = Box::new(ListNode { val: 0, nxt: hed });
    let mut prv = &mut dum;

    while let Some(ref mut cur) = prv.nxt {
        // Search for duplicate nodes.
        let mut dup_fnd: bool = false;
        while let Some(ref nxt) = cur.nxt {
            if cur.val == nxt.val {
                // Skip over the next node.
                cur.nxt = nxt.nxt.clone();
                dup_fnd = true;
            } else {
                break;
            }
        }
        if dup_fnd {
            // Remove the current node.
            prv.nxt = cur.nxt.clone();
        } else {
            // Move the pointer forward.
            prv = prv.nxt.as_mut().unwrap();
        }
    }

    dum.nxt
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
            nxt: Some(Box::new(ListNode {
                val: 2,
                nxt: Some(Box::new(ListNode {
                    val: 3,
                    nxt: Some(Box::new(ListNode {
                        val: 3,
                        nxt: Some(Box::new(ListNode {
                            val: 4,
                            nxt: Some(Box::new(ListNode {
                                val: 4,
                                nxt: Some(Box::new(ListNode { val: 5, nxt: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let res = delete_duplicates(head);

        let expected = Some(Box::new(ListNode {
            val: 1,
            nxt: Some(Box::new(ListNode {
                val: 2,
                nxt: Some(Box::new(ListNode { val: 5, nxt: None })),
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
            nxt: Some(Box::new(ListNode {
                val: 1,
                nxt: Some(Box::new(ListNode {
                    val: 1,
                    nxt: Some(Box::new(ListNode {
                        val: 2,
                        nxt: Some(Box::new(ListNode { val: 3, nxt: None })),
                    })),
                })),
            })),
        }));

        let res = delete_duplicates(head);

        let expected = Some(Box::new(ListNode {
            val: 2,
            nxt: Some(Box::new(ListNode { val: 3, nxt: None })),
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
            nxt: Some(Box::new(ListNode {
                val: 1,
                nxt: Some(Box::new(ListNode {
                    val: 1,
                    nxt: Some(Box::new(ListNode { val: 1, nxt: None })),
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
            nxt: Some(Box::new(ListNode {
                val: 2,
                nxt: Some(Box::new(ListNode {
                    val: 3,
                    nxt: Some(Box::new(ListNode {
                        val: 4,
                        nxt: Some(Box::new(ListNode { val: 5, nxt: None })),
                    })),
                })),
            })),
        }));

        let res = delete_duplicates(head.clone());
        assert_eq!(res, head);
    }
}
