
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
    dp[0][0] = if obstacle_grid[0][0] == 1 {
        0
    } else {
        1
    };
    for row in 1..obstacle_grid.len() {
        dp[row][0] = if obstacle_grid[row][0] == 1 {
            0
        } else {
            dp[row-1][0]
        };
    }
    for col in 1..obstacle_grid[0].len() {
        dp[0][col] = if obstacle_grid[0][col] == 1{
            0
        } else {
            dp[0][col-1]
        };
    }

    for row in 1..obstacle_grid.len() {
        for col in 1..obstacle_grid[0].len() {
            dp[row][col] = if obstacle_grid[row][col] == 1 {
                0
            } else {
                dp[row-1][col] + dp[row][col-1]
            }
        }
    }

    dp[obstacle_grid.len()-1][obstacle_grid[0].len()-1]

}

