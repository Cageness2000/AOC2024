
pub fn part1(sorted_left:Vec<i32>, sorted_right: Vec<i32> )-> i32 {
    
    let mut i: usize = 0;
    let num_rows: usize = sorted_left.len();


    let mut difference: Vec<i32> = Vec::with_capacity(num_rows);
    loop {
        if i == num_rows {break;}
        difference.push(sorted_left[i]-sorted_right[i]);
        i += 1;
    }
    i = 0;
    let mut sum_diff = 0;
    loop {
        if i == num_rows {break;}
        sum_diff += i32::abs(difference[i]);
        i += 1;
    }
    return sum_diff;
}


