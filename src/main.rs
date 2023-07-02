fn main() {
    let sudoku: Vec<Vec<u32>> = include_str!("./sudoku.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let result: Vec<Vec<u32>> = solve(&sudoku);
    
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", result[i][j]);
        }
        println!()
    }
    ()
}


fn solve(sudoku: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_sudoku: Vec<Vec<u32>> = sudoku.clone();
    for x in 0..9 {
        for y in 0..9 {
            if new_sudoku[x][y] == 0 {
                for val in 1..10 {
                    if possible(&new_sudoku, x, y, val) {
                        new_sudoku[x][y] = val;
                        new_sudoku = solve(&new_sudoku);
                        if new_sudoku.iter().all(|row: &Vec<u32>| row.iter().all(|&x| x != 0)) {
                            return new_sudoku;
                        }
                        new_sudoku[x][y] = 0;
                    }
                }
                return new_sudoku;
            }
        }
    }
    new_sudoku
}

fn possible(sudoku: &Vec<Vec<u32>>, x: usize, y: usize, val: u32) -> bool {
    for i in 0..9 {
        if i != x && sudoku[i][y] == val {
            return false;
        }
    }

    for j in 0..9 {
        if j != y && sudoku[x][j] == val {
            return false;
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            let new_x: usize = x / 3 * 3 + i;
            let new_y: usize = y / 3 * 3 + j;
            // print!("{}, {}   ", new_x, new_y);
            if new_x != x && new_y != y && sudoku[new_x][new_y] == val {
                return false;
            }
        }
        // println!()
    }

    true
}

// 0 1 2
// 3 4 5
// 6 7 8
