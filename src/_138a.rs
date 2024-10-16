use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    val: i32,
    nxt: NodeRef,
    rnd: NodeRef,
}

type NodeRef = Option<Rc<RefCell<Node>>>;

// Time complexity: O(n), n is the length of the linked list. We traverse the list three times.
// Space complexity: O(1), constant additional space is used.
fn copy_random_list(hd: NodeRef) -> NodeRef {
    // Copy List with Random Pointer
    // Given the head of a singly linked list.
    // Nodes have a pointer to a random node.
    // Create a deep copy of the linked list.
    // Return the head of the deep copy linked list.
    // Use a three-pass approach.
    // Pass one: Copy and interleave copied nodes with original nodes.
    // Pass two: Set random pointers on copied nodes.
    // Pass three: Deinterleave. Separate original list and copied list.

    // Check for input minimum edge condition.
    hd.as_ref()?;

    // Pass one: copy node and interleave.
    let mut cur = hd.clone();
    while let Some(org_nod) = cur {
        // Copy current node.
        let cpy_nod = Rc::new(RefCell::new(Node {
            val: org_nod.borrow().val,
            nxt: org_nod.borrow().nxt.clone(),
            rnd: None,
        }));
        // Interleave copied node.
        org_nod.borrow_mut().nxt = Some(cpy_nod.clone());
        // Move current pointer forward.
        cur = cpy_nod.borrow().nxt.clone();
    }

    // Pass two: set random pointer fields on copied nodes.
    cur = hd.clone();
    while let Some(org_nod) = cur {
        // Get copied nodes.
        let org_brw = org_nod.borrow();
        let cpy_nod = org_brw.nxt.as_ref().unwrap();
        let cpy_rnd = org_brw
            .rnd
            .as_ref()
            .and_then(|rnd_nod| rnd_nod.borrow().nxt.clone());
        // Set random pointer on copied.
        cpy_nod.borrow_mut().rnd = cpy_rnd;
        // Move original current pointer forward.
        cur = cpy_nod.borrow().nxt.clone();
    }

    // Pass three: separate original list and copied list.
    cur = hd.clone();
    let cpy_hd = hd.as_ref().and_then(|n| n.borrow().nxt.clone());
    while let Some(org_nod) = cur {
        let cpy_nod = org_nod.borrow().nxt.clone();
        let org_nxt = cpy_nod.as_ref().and_then(|n| n.borrow().nxt.clone());
        // Set original list node.
        org_nod.borrow_mut().nxt = org_nxt.clone();
        // Set copy list node.
        if let Some(cpy_nod) = cpy_nod {
            cpy_nod.borrow_mut().nxt = org_nxt.as_ref().and_then(|n| n.borrow().nxt.clone());
        }
        // Move the original current pointer forward.
        cur = org_nxt;
    }

    cpy_hd
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(vals: Vec<(i32, Option<usize>)>) -> NodeRef {
        let mut nodes: Vec<Rc<RefCell<Node>>> = vec![];
        for &(val, _) in &vals {
            nodes.push(Rc::new(RefCell::new(Node {
                val,
                nxt: None,
                rnd: None,
            })));
        }
        for (i, &(_, rnd_idx)) in vals.iter().enumerate() {
            if i + 1 < nodes.len() {
                nodes[i].borrow_mut().nxt = Some(nodes[i + 1].clone());
            }
            if let Some(rnd_idx) = rnd_idx {
                nodes[i].borrow_mut().rnd = Some(nodes[rnd_idx].clone());
            }
        }
        nodes.first().cloned()
    }

    fn lists_equal(l1: NodeRef, l2: NodeRef) -> bool {
        use std::collections::HashMap;
        let mut visited = HashMap::new();
        let mut stack = vec![(l1, l2)];

        while let Some((n1_opt, n2_opt)) = stack.pop() {
            match (n1_opt, n2_opt) {
                (Some(n1_rc), Some(n2_rc)) => {
                    let n1_ptr = Rc::as_ptr(&n1_rc) as usize;
                    let n2_ptr = Rc::as_ptr(&n2_rc) as usize;

                    if visited.contains_key(&n1_ptr) {
                        if visited[&n1_ptr] != n2_ptr {
                            return false;
                        }
                        continue;
                    } else {
                        visited.insert(n1_ptr, n2_ptr);
                    }

                    let n1 = n1_rc.borrow();
                    let n2 = n2_rc.borrow();

                    if n1.val != n2.val {
                        return false;
                    }

                    stack.push((n1.nxt.clone(), n2.nxt.clone()));
                    stack.push((n1.rnd.clone(), n2.rnd.clone()));
                }
                (None, None) => continue,
                _ => return false,
            }
        }

        true
    }

    #[test]
    fn valid_list() {
        let hd = build_list(vec![
            (7, None),
            (13, Some(0)),
            (11, Some(4)),
            (10, Some(2)),
            (1, Some(0)),
        ]);
        let copied_hd = copy_random_list(hd.clone());
        assert!(lists_equal(hd, copied_hd));
    }

    #[test]
    fn empty_list() {
        let hd: NodeRef = None;
        let copied_hd = copy_random_list(hd.clone());
        assert!(lists_equal(hd, copied_hd));
    }

    #[test]
    fn single_node() {
        let hd = build_list(vec![(1, None)]);
        let copied_hd = copy_random_list(hd.clone());
        assert!(lists_equal(hd, copied_hd));
    }
}
