use std::io;

fn main() {
    println!("Welcome to Tic Tac Toe!");
    let mut board = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut player = 'X';
    let mut game_over = false;

    while !game_over {
        display_board(&board);
        let move_index = get_move(&player, &board);
        board[move_index] = player;

        if check_win(&board, player) {
            display_board(&board);
            println!("{} wins!", player);
            game_over = true;
        } else if check_tie(&board) {
            display_board(&board);
            println!("It's a tie!");
            game_over = true;
        } else {
            if player == 'X' {
                player = 'O';
            } else {
                player = 'X';
            }
        }
    }
}

fn display_board(board: &[char]) {
    println!(" {} | {} | {} ", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {} ", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {} ", board[6], board[7], board[8]);
}

fn get_move(player: &char, board: &[char]) -> usize {
    loop {
        println!("Player {}, enter your move (0-8):", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if input >= 0 && input <= 8 && is_available(&input, &board) {
            return input;
        } else {
            println!("Invalid move, try again.");
        }
    }
}

fn is_available(index: &usize, board: &[char]) -> bool {
    board[*index] == ' '
}

fn check_win(board: &[char], player: char) -> bool {
    // Check rows
    for i in (0..9).step_by(3) {
        if board[i] == player && board[i + 1] == player && board[i + 2] == player {
            return true;
        }
    }
    // Check columns
    for i in 0..3 {
        if board[i] == player && board[i + 3] == player && board[i + 6] == player {
            return true;
        }
    }
    // Check diagonals
    if board[0] == player && board[4] == player && board[8] == player {
        return true;
    }
    if board[2] == player && board[4] == player && board[6] == player {
        return true;
    }
    false
}

fn check_tie(board: &[char]) -> bool {
    board.iter().all(|&square| square != ' ')
}