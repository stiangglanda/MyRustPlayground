use std::cmp::PartialEq;
use std::io;

struct TicTacToe
{
    field: [char; 9],
}

enum CheckResult {
    None,
    OWin,
    XWin,
    Draw,
}

enum SetResult {
    Ok,
    OutOfBounds,
    Taken,
}

impl PartialEq for CheckResult {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe { field: [' '; 9] }
    }

    fn print(&self) {
        println!("{} | {} | {}", self.field[0], self.field[1], self.field[2]);
        println!("----------");
        println!("{} | {} | {}", self.field[3], self.field[4], self.field[5]);
        println!("----------");
        println!("{} | {} | {}", self.field[6], self.field[7], self.field[8]);
    }

    fn set(&mut self, index: usize, ch: char) -> SetResult {
        if(index >= 0 && index <= 8) {
            if(self.field[index]==' ')
            {
                self.field[index] = ch;
                return SetResult::Ok;
            }
            return SetResult::Taken;
        }
        return SetResult::OutOfBounds;
    }

    fn play_turn(&mut self, ch: char) -> CheckResult {
        loop {
            println!("Please enter a number from 0 to 8 for {}", ch);

            let mut index = String::new();

            io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");

            let index = match index.trim().parse::<usize>() {
                Ok(index) => index,
                Err(error) => {
                    println!("Input should be a number");
                    continue
                },
            };

            match self.set(index, ch) {
                SetResult::Ok => {
                    return self.check();
                },
                SetResult::OutOfBounds => {
                    println!("Input should be between 0 and 8");
                    continue;
                },
                SetResult::Taken => {
                    println!("field is already taken");
                    continue;
                },
            }
        }
    }

    fn play(&mut self) {
        loop {
            self.print();
            if TicTacToe::react_to_result(self.play_turn('X')) == false {
                return;
            }
            self.print();
            if TicTacToe::react_to_result(self.play_turn('O')) == false {
                return;
            }
        }
    }

    fn react_to_result(result: CheckResult) -> bool {
        match result {
            CheckResult::None => return true,
            CheckResult::OWin => {
                println!("O Wins");
                return false;
            }
            CheckResult::XWin => {
                println!("X Wins");
                return false;
            }
            CheckResult::Draw => {
                println!("Draw");
                return false;
            }
        }
    }

    fn check(&self) -> CheckResult {
        //Horizontal check
        for n in [0,3,6] {
            if self.field[n] == ' ' {
                continue;
            }

            if self.field[n] == self.field[n+1] && self.field[n+1] == self.field[n+2] {
                return if self.field[n] == 'X' {
                    CheckResult::XWin
                } else {
                    CheckResult::OWin
                }
            }
        }
        //Vertical Check
        for n in 0..3 {
            if self.field[n] == ' ' {
                continue;
            }

            if self.field[n] == self.field[n+3] && self.field[n+3] == self.field[n+6] {
                return if self.field[n] == 'X' {
                    CheckResult::XWin
                } else {
                    CheckResult::OWin
                }
            }
        }
        //Diagonal Check
        if self.field[4] == ' ' {
            return CheckResult::None;
        }

        if (self.field[0] == self.field[4] && self.field[4] == self.field[8]) ||
            (self.field[2] == self.field[4] && self.field[4] == self.field[6]) {
            return if self.field[4] == 'X' {
                CheckResult::XWin
            } else {
                CheckResult::OWin
            }
        }

        //check Draw
        return if self.field.iter().all(|&square| square != ' ') == true {
            CheckResult::Draw
        } else {
            CheckResult::None
        }
    }
}

fn main() {
    println!("Rust Tic Tac Toe");

    let mut tic_tac_toe = TicTacToe::new();
    tic_tac_toe.play();
    tic_tac_toe.print();
}
