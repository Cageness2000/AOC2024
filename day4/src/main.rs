mod common;

fn generate_letters(contents:&str) -> Vec<Vec<char>> {
    let mut letters:Vec<Vec<char>> = vec![];
    for line in contents.split('\n'){
        let mut letter_line:Vec<char> = vec![];
        for c in line.chars() {
            letter_line.push(c);
        }
        letters.push(letter_line);
    }
    return letters[..letters.len()-1].to_owned();
}

fn walk_indicies(ab:&Vec<Vec<char>>,row:&usize,col:&usize) -> Vec<[usize;2]> {
    let mut out:Vec<[usize;2]> = vec![];
    let mut corner_detect:[bool;4] = [false,false,false,false];
    if 0<col+0 {
        out.push([row+0,col-1]);
        corner_detect[0] = true;
    }
    if col+0<ab[row+0].len()-1 {
        out.push([row+0,col+1]);
        corner_detect[1] = true;
    }
    if 0<row+0 {
        out.push([row-1,col+0]);
        corner_detect[2] = true;
    }
    if row+0<ab.len()-1 {
        out.push([row+1,col+0]);
        corner_detect[3] = true;
    }
    if corner_detect[0] && corner_detect[2] {
        out.push([row-1,col-1]);
    }
    if corner_detect[1] && corner_detect[3] { 
        out.push([row+1,col+1]);
    }
    if corner_detect[0] && corner_detect[3] {
        out.push([row+1,col-1]);
    }
    if corner_detect[1] && corner_detect[2] {
        out.push([row-1,col+1]);
    }

    return out;
}

fn walk_letters(row:&usize,col:&usize,looking_for:usize,letters :&Vec<Vec<char>>) -> i32{
    let goal = ['X','M','A','S'];
    if looking_for == goal.len() {println!("found");return 1};
    let row_cols = walk_indicies(letters, row, col);
    for row_col in row_cols {
        let l = letters[row_col[0]][row_col[1]];
        println!("row {}, col {}, c {}",row_col[0],row_col[1],l);
        if letters[row_col[0]][row_col[1]] == goal[looking_for] {
            return walk_letters(&row_col[0],&row_col[1],looking_for+1,letters);
        }
    }
    return 0;
}

fn brute_force_part_1(contents:&str) ->i32 {
    let mut found = 0;
    let letters = generate_letters(contents);
    let mut row:usize = 0;
    loop {
        if row == letters.len() {break}
        let mut col:usize = 0;
        loop {
            if col == letters[row].len() {break}
            found +=walk_letters(&row, &col, 0 , &letters);
            col+=1;
        }
        row+=1;
    }
    return found;
    
}

fn main() {
    let location = "test".to_string();
    let contents = common::read(&location);
    println!("{}",brute_force_part_1(&contents));

}
