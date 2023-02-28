use std::io;

fn parser(message: &String) -> usize {
    loop {
        println!("Input {}", message);
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim().parse::<usize>();
        match trimmed {
            Ok(i) => {
                let range = 1..=3;
                if range.contains(&i) {
                    return i
                } else {
                    continue
                }
            }
            Err(_) => {
                println!("Input numbers!");
                continue
            }
        }
    }
}

fn swap_player(ch: &char) ->char {
    if *ch == 'X' {
        'O'
    } else {
        'X'
    }
}

fn main() {
    let mut state = [[' '; 3]; 3];
    let mut  symbol = 'X';
    loop {
        view_board(&state);
        let row  = parser(&String::from("row")) - 1;
        let column = parser(&String::from("column")) - 1;
        let success = change_board(&mut state, row, column, symbol);
        if win (row as i32,column as i32,&state,&symbol) {
            println!("{} wins", symbol);
            break;
        }
         if success {
             symbol = swap_player(&symbol);
        }
    }
}

fn change_board(board: &mut [[char;3];3], row: usize, column: usize, ch: char) -> bool {
    if board[row][column] != ' ' {
        println!("Cell is occupied");
        false
    }  else {
        board[row][column] = ch;
        true
    }
}

fn view_board(board: &[[char;3];3]) {
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            print!("{} |", board[i][j]);
        }
        print!("\n");
    }
}

fn looping(row: i32, column: i32, board:  &[[char;3];3], hor: i32, ver: i32, r_diagonal: i32, ch: &char) -> bool {
    let mut count = 0;
    let range = 0..3;
    for i in -2..2 {
        let r = row  + i as i32  * hor  * r_diagonal;
        let c = column  + i  * ver;
        if range.contains(&r) &&  range.contains(&c) {
            let char = board[r as usize][c as usize];
            //print!("{} {}", char,  *ch);
            if char == *ch  {
                count+= 1
            }
        }
    }
   count == 3
}

fn win (row: i32, column: i32, board: &[[char;3];3], ch: &char) -> bool {
    //println!("{}", ch);
    looping(row,column,board,1,0,1,ch) ||
        looping(row,column,board,0,1,1,ch) ||
        looping(row,column,board,1,1,1,ch) ||
        looping(row,column,board,1,1,-1,ch)
}
