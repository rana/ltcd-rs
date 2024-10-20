use std::cell::RefCell;
use std::rc::Rc;
pub struct ListNode {
    pub val: i32,
    pub nxt: Option<Rc<RefCell<ListNode>>>,
}

fn has_cycle(hd: Option<Rc<RefCell<ListNode>>>) -> bool {
    // Linked List Cycle
    // Given a head node of a linked list.
    // Determine if there is a cycle.
    // Return true if there is a cycle.
    // Use Floyd's Tortoise and Hare algorithm.
    let mut slw = hd.clone();
    let mut fst = hd.clone();

    while let Some(fst_node) = fst.clone() {
        fst = fst_node.borrow().nxt.clone();
        if fst.is_none() {
            return false;
        }
        fst = fst.unwrap().borrow().nxt.clone();

        slw = slw.unwrap().borrow().nxt.clone();
        if slw.is_some()
            && fst.is_some()
            && Rc::ptr_eq(slw.as_ref().unwrap(), fst.as_ref().unwrap())
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn cycle_present() {
        let nde1: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 1, nxt: None }));
        let nde2: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 2, nxt: None }));
        let nde3: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 3, nxt: None }));

        nde1.borrow_mut().nxt = Some(nde2.clone());
        nde2.borrow_mut().nxt = Some(nde3.clone());
        nde3.borrow_mut().nxt = Some(nde1.clone()); // Creates a cycle

        let hd: Option<Rc<RefCell<ListNode>>> = Some(nde1.clone());
        assert!(has_cycle(hd));
    }

    #[test]
    fn no_cycle() {
        let nde1: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 1, nxt: None }));
        let nde2: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 2, nxt: None }));
        let nde3: Rc<RefCell<ListNode>> = Rc::new(RefCell::new(ListNode { val: 3, nxt: None }));

        nde1.borrow_mut().nxt = Some(nde2.clone());
        nde2.borrow_mut().nxt = Some(nde3.clone());

        let hd: Option<Rc<RefCell<ListNode>>> = Some(nde1.clone());
        assert!(!has_cycle(hd));
    }
}
