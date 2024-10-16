use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    nxt: NodeRef,
    rnd: NodeRef,
}

// Time complexity: O(n), n is the length of the linked list.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/67100a02-be54-8002-bb45-9d7d30c77d33
fn copy_random_list(hd: NodeRef) -> NodeRef {
    hd.as_ref()?; // Returns None if hd is None

    // First pass: Create new nodes and insert them next to original nodes.
    let mut cur: NodeRef = hd.clone();
    while let Some(org_nod) = cur {
        let cpy_nod: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            val: org_nod.borrow().val,
            nxt: org_nod.borrow().nxt.clone(),
            rnd: None,
        }));
        org_nod.borrow_mut().nxt = Some(cpy_nod.clone());
        cur = cpy_nod.borrow().nxt.clone();
    }

    // Second pass: Set the random pointers for the new nodes.
    cur = hd.clone();
    while let Some(org_nod) = cur {
        let org_brw = org_nod.borrow();
        let cpy_nod = org_brw.nxt.as_ref().unwrap();
        let cpy_rnd = org_brw
            .rnd
            .as_ref()
            .and_then(|rnd_nod| rnd_nod.borrow().nxt.clone());
        cpy_nod.borrow_mut().rnd = cpy_rnd;
        cur = cpy_nod.borrow().nxt.clone();
    }

    // Third pass: Separate the copied list from the original list.
    cur = hd.clone();
    let cpy_hd = hd.as_ref().unwrap().borrow().nxt.clone();
    while let Some(org_nod) = cur {
        let cpy_nod = org_nod.borrow().nxt.clone();
        let org_nxt = cpy_nod.as_ref().and_then(|n| n.borrow().nxt.clone());
        org_nod.borrow_mut().nxt = org_nxt.clone();
        if let Some(cpy_nod) = cpy_nod {
            cpy_nod.borrow_mut().nxt = org_nxt.as_ref().and_then(|n| n.borrow().nxt.clone());
        }
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
