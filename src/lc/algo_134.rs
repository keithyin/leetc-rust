
fn can_complete_circuit_core(gas: &Vec<i32>, cost: &Vec<i32>, init_pos: usize) -> i32{
    let mut remain_gas = 0;
    for i in 0..gas.len() {
        let cur_pos = (i + init_pos) % gas.len();
        remain_gas += gas[cur_pos];
        remain_gas -= cost[cur_pos];
        if remain_gas < 0 {
            return -1;
        }
    }
    init_pos as i32
}

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    for i in 0..gas.len() {
        let res = can_complete_circuit_core(&gas, &cost, i);
        if res >= 0 {
            return res;
        }
    }
    -1
}