use std::cmp::max;
use std::collections::HashSet;

pub fn get_skyline1(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut xs = HashSet::new();
    buildings.iter().for_each(|item| {
        xs.insert(item[0]);
        xs.insert(item[1]);
    });

    let mut result = vec![];

    let mut xs: Vec<i32> = xs.iter().map(|v| *v).collect();
    xs.sort();
    let mut prev_max_height = 0;
    for current_x in xs {
        let mut max_height = 0;
        for rect in buildings.iter() {
            if current_x >= rect[0] && current_x < rect[1] {
                max_height = max(max_height, rect[2]);
            }
        }
        if prev_max_height != max_height {
            result.push(vec![current_x, max_height]);
            prev_max_height = max_height;
        }
    }
    result
}