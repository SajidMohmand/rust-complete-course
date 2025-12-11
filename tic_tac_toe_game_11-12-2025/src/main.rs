use std::io;

fn main() {
    let rows = 3;
    let cols = 3;

    let mut matrix = vec![vec!["-"; cols]; rows];

    let mut flag = true;

    println!("Game Start");

    loop {

        if flag {
            println!("Player 'X' Turn.");
        }else{
            println!("Player 'O' Turn.");
        }
        println!("Please enter col & row");
        let mut input = String::new();

       io::stdin().read_line(&mut input).expect("Failed to read input");

        
        let input = input.trim();


        if input.len() == 2 {

            
           let r = input.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
            let c = input.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;

            if matrix[r][c] != "-" {
                println!("!Wrong Place, Already Occupied. Choose another.");
                continue;
            }


            matrix[r][c] = if flag { "X" } else { "O" };
            flag = !flag;


            for i in &matrix {

                for j in i {
                    print!("{}", j);
                }
                println!();
            }

            if let Some(winner) = check_winner(&matrix) {
                println!("Congratulation! Play {} win",winner);
                break;
            }


        } else {
            println!("!Invalid Input - please enter two digit only.");
        }
    }
}


fn check_winner<'a>(matrix: &'a Vec<Vec<&'a str>>) -> Option<&'a str> {

    // rows
    for i in 0..3{
        if matrix[i][0] == matrix[i][1] && matrix[i][0] == matrix[i][2] && matrix[i][0] != "-" {
            return Some(matrix[i][0]);
        }
    }

    // column
    for i in 0..3 {
        if matrix[0][i] == matrix[1][i] && matrix[0][i] == matrix[2][i] && matrix[0][i] != "-" {
            return Some(matrix[0][i]);
        }
    }

    // diagnols
    if matrix[0][0] == matrix[1][1] && matrix[0][0] == matrix[2][2] && matrix[0][0] != "-" {
        return Some(matrix[0][0]);
    }
    if matrix[0][2] == matrix[2][2] && matrix[0][2] == matrix[2][0] && matrix[0][2] != "-" {
        return Some(matrix[0][2]);
    }

    return None;
}
