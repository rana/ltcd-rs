/// 134. Gas Station
///
/// There are n gas stations along a circular route,
/// where the amount of gas at the ith station is gas[i].
///
/// You have a car with an unlimited gas tank and it costs
/// cost[i] of gas to travel from the ith station to its next
/// (i + 1)th station. You begin the journey with an empty
/// tank at one of the gas stations.
///
/// Given two integer arrays gas and cost, return the starting
/// gas station's index if you can travel around the circuit
/// once in the clockwise direction, otherwise return -1.
/// If there exists a solution, it is guaranteed to be unique
///
/// Constraints:
/// * n == gas.length == cost.length
/// * 1 <= n <= 10^5
/// * 0 <= gas[i], cost[i] <= 10^4

fn can_complete_circuit_b(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // Variables contribute to O(1) space complexity.
    let mut fuel_total: i32 = 0;
    let mut fuel_cur: i32 = 0;
    let mut idx_fst: usize = 0;

    // Loop contributes to O(n) time complexity.
    for idx in 0..gas.len() {
        // Track total fuel to see if entire circuit is possible.
        fuel_total += gas[idx] - cost[idx];
        // Track the current fuel to see the starting station.
        fuel_cur += gas[idx] - cost[idx];
        if fuel_cur < 0 {
            // Set a potential new gas station for the next station.
            idx_fst = idx + 1;
            fuel_cur = 0;
        }
    }

    if fuel_total >= 0 {
        (idx_fst % gas.len()) as i32
    } else {
        -1
    }
}

fn can_complete_circuit_a(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut fuel: i32 = 0;
    let mut deficit: i32 = 0;
    let mut idx_fst: usize = 0;

    for idx in 0..gas.len() {
        fuel += gas[idx] - cost[idx];
        if fuel < 0 {
            deficit += fuel;
            fuel = 0;
            idx_fst = idx + 1;
        }
    }

    if (fuel + deficit) >= 0 {
        return idx_fst as i32;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_can_complete_circuit_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = can_complete_circuit_b(t.gas.clone(), t.cost.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_can_complete_circuit_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = can_complete_circuit_a(t.gas.clone(), t.cost.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                gas: vec![1, 2, 3, 4, 5],
                cost: vec![3, 4, 5, 1, 2],
                ret: 3,
            },
            Tst {
                gas: vec![2, 3, 4],
                cost: vec![3, 4, 3],
                ret: -1,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        gas: Vec<i32>,
        cost: Vec<i32>,
        ret: i32,
    }
}
