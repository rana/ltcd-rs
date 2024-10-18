#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Time complexity: O(n), n is the length of the list. We traverse the list once.
// Space complexity: O(n/k), recursive calls use up to n/k additional space.
// https://chatgpt.com/c/67116f24-7b64-8002-a365-74370d1cadf9
fn reverse_k_group(hd: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // Reverse Nodes in k-Group
    // Given the head of a linked list.
    // Reverse nodes in groups of k.
    // Don't reverse if there are fewer than k nodes.
    // Return the head of the processed list.
    // Use recursion to reverse groups.

    // Check if there are at least k nodes.
    // Return the head if there are fewer than k nodes.
    let mut cnt: i32 = 0;
    let mut ptr: &Option<Box<ListNode>> = &hd;
    while cnt < k {
        if let Some(nod) = ptr {
            // Move pointer to next node.
            ptr = &nod.next;
            cnt += 1;
        } else {
            // Too few nodes; return the head.
            return hd;
        }
    }

    // Reverse k nodes.
    let mut cur = hd;
    let mut prv = None;
    cnt = 0;
    while cnt < k {
        if let Some(mut nod) = cur.take() {
            let nxt = nod.next.take();
            nod.next = prv;
            prv = Some(nod);
            cur = nxt;
            cnt += 1;
        }
    }

    // After reversal:
    //  * prv is head of reversed.
    //  * cur is head of next segement.
    // Find tail of reversed segement.
    let mut tail = prv.as_mut();
    for _ in 0..(k - 1) {
        if let Some(nod) = tail {
            tail = nod.next.as_mut();
        }
    }

    // Connect the current segment tail to remaining list.
    if let Some(tail_nod) = tail {
        tail_nod.next = reverse_k_group(cur, k);
    }

    // Return the new head after reversal.
    prv
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a linked list from a vector
    fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut hd: Option<Box<ListNode>> = None;
        for &val in nums.iter().rev() {
            let node = Box::new(ListNode { val: val, next: hd });
            hd = Some(node);
        }
        hd
    }

    // Helper function to convert a linked list to a vector
    fn list_to_vec(mut hd: Option<Box<ListNode>>) -> Vec<i32> {
        let mut nums: Vec<i32> = Vec::new();
        while let Some(node) = hd {
            nums.push(node.val);
            hd = node.next;
        }
        nums
    }

    #[test]
    fn reverse_k_group_example1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 2;
        let hd = vec_to_list(nums);
        let res = reverse_k_group(hd, k);
        let res_vec = list_to_vec(res);
        assert_eq!(res_vec, vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn reverse_k_group_example2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 3;
        let hd = vec_to_list(nums);
        let res = reverse_k_group(hd, k);
        let res_vec = list_to_vec(res);
        assert_eq!(res_vec, vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn reverse_k_group_k_equals_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 1;
        let hd = vec_to_list(nums.clone());
        let res = reverse_k_group(hd, k);
        let res_vec = list_to_vec(res);
        assert_eq!(res_vec, nums);
    }

    #[test]
    fn reverse_k_group_k_equals_n() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 5;
        let hd = vec_to_list(nums.clone());
        let res = reverse_k_group(hd, k);
        let res_vec = list_to_vec(res);
        assert_eq!(res_vec, vec![5, 4, 3, 2, 1]);
    }
}
