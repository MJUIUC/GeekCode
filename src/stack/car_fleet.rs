struct Solution {}
impl Solution {
    /**
    * 1. sort the vehicles in order of position. so we'd make an array of tuples where (position[i], speed[i]) and then sort by the position key.
    2. itterate through the sorted position array, calculating the time of arravial at each position.
    3. each itteration will need to check the current arrival time against the arrival time of the vehicle before it. if it's less than or equal, then we can assume it's part of the previous group. if it's greater than, then it needs to be a new group.
    4. use a stack to keep track of the arrival times. an arrival time is pushed to the stack anytime a new group is found, or at the beginning of the itteration loop.
    5. at the end, the length of the stack would be the number of groups found.
    */
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arrival_time_stack: Vec<f64> = vec![]; // grouped arrival times

        let mut sorted_positions: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();

        sorted_positions.sort_by(|a, b| b.0.cmp(&a.0));
        // sorted_positions.reverse();
        // println!("{:?}", sorted_position);

        for i in 0..sorted_positions.len() {
            let arrival_time: f64 =
                (target - sorted_positions[i].0) as f64 / sorted_positions[i].1 as f64;
            if (arrival_time_stack.is_empty())
                || (arrival_time_stack.last().unwrap() < &arrival_time)
            {
                arrival_time_stack.push(arrival_time)
            }
        }

        arrival_time_stack.len() as i32
    }
}
