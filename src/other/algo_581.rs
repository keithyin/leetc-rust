
/**
 [2, 6, 4, 8, 10, 9, 15]
 数组可以分成三个部分：
 1. 前半部分排好序的
 2. 中间部分未排序的
 3. 后面部分排好序的
 如何找到分界点呢？
 如果使用单调栈：下一个比当前数大的数。如果下一个比当前大的数不是邻接的数，那就是有问题了
    下一个比当前大的
    上一个比当前小的
*/
pub fn find_unsorted_sub_array(values: Vec<i32>) {
    let mut next_bigger_idx = vec![-1; values.len()];
    let mut mono_stack = vec![];
    for (i, v) in values.iter().enumerate().rev() {
        while mono_stack.len() > 0 && values[mono_stack[mono_stack.len()-1]] <= *v {
            mono_stack.pop();
        }
        next_bigger_idx[i] = if mono_stack.is_empty() {
            -1
        } else {
            mono_stack[mono_stack.len() - 1]
        };
        mono_stack.push(i);
    }
    let mut upper_bound = -1;
    for (i, v) in next_bigger_idx.iter().enumerate() {
        if i == next_bigger_idx.len()-1 {
            continue;
        }

        if next_bigger_idx[i] == -1 {
            upper_bound = next_bigger_idx.len();
            break;
        }

        if next_bigger_idx[i] - i == 1 {
            continue;
        }

        upper_bound = if upper_bound < next_bigger_idx[i] {
            next_bigger_idx[i]
        } else {
            upper_bound
        };
    }

    let mut last_lesser_idx = vec![];
    mono_stack = vec![];
    for (i, v) in values.iter().enumerate() {
        while !mono_stack.is_empty() && values[mono_stack[mono_stack.len()-1]] >= *v {
            mono_stack.pop();
        }

    }

}