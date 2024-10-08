// Time Complexity: O(n), where n is the length of the arrays.
// Space Complexity: O(1), constant space used.
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // Gas stations along circular route.
    // Given arrays gas and cost.
    // * gas[i] available at gas station i.
    // * cost[i] cost to travel from i to i+1.
    // Start with empty tank at some gas station.
    // Return the starting gas station for a complete circuit;
    // or return -1 if not possible.
    // Use a local optimization "greedy" approach.

    // Initialize variables.
    let mut total_tank = 0; // Use for total circuit determiniation.
    let mut cur_tank = 0; // Use for starting station determination.
    let mut starting_station = 0;

    // Loop through each element.
    for n in 0..gas.len() {
        let dif = gas[n] - cost[n];
        total_tank += dif;
        cur_tank += dif;

        // Check starting station reset.
        if cur_tank < 0 {
            cur_tank = 0;
            starting_station = n + 1;
        }
    }

    // Check whether total circuit possible.
    if total_tank >= 0 {
        starting_station as i32
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
