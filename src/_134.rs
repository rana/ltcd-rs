// Time Complexity: O(n), when n is the length of the arrays.
// Space Complexity: O(1), constant space used.
// https://chatgpt.com/c/670435ce-8814-8002-aaa2-21d02e0f646a
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // Determine the starting gas station index.
    // Or, return -1 for not possible to complete circuit.
    // Use a local optimization "greedy" algorithm.

    // Initialize variables.
    let mut total_tank = 0;
    let mut cur_tank = 0;
    let mut starting_station = 0;

    // Loop through each element.
    for n in 0..gas.len() {
        let dif = gas[n] - cost[n];
        total_tank += dif;
        cur_tank += dif;
        if cur_tank < 0 {
            // Reset next starting station.
            starting_station = n as i32 + 1;
            cur_tank = 0;
        }
    }

    // Evaluate whether the circuit can be completed.
    if total_tank >= 0 {
        starting_station
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_case_1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn test_valid_case_2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(can_complete_circuit(gas, cost), -1);
    }

    #[test]
    fn test_valid_case_3() {
        let gas = vec![5, 1, 2, 3, 4];
        let cost = vec![4, 4, 1, 5, 1];
        assert_eq!(can_complete_circuit(gas, cost), 4);
    }

    #[test]
    fn test_invalid_case() {
        let gas = vec![1, 2, 3];
        let cost = vec![4, 4, 4];
        assert_eq!(can_complete_circuit(gas, cost), -1);
    }
}
