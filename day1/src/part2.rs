

fn count_same_vals(value:i32,vec:&Vec<i32>) -> i32 {
    let mut j = 0;
    let mut count = 0;
    loop {
        if j == vec.len() {break;}
        if vec[j] == value {
            count += 1;
        } else if value < vec[j] {
            break
        }
        j += 1;
    }
    return count;
}


pub fn part2(sorted_left: &Vec<i32>, sorted_right: &Vec<i32>) -> i32 {
    
    let mut i: usize = 0;
    let mut similarity = 0;
    loop {
        if i == sorted_left.len() {break;}
        similarity += sorted_left[i]*count_same_vals(sorted_left[i],&sorted_right);
        i+=1;
    }
    
    return similarity;
}
