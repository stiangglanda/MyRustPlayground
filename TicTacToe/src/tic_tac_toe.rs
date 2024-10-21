pub struct TicTacToe {
    board: [[char; 3]; 3],
}

pub enum Outcome {
    Win(String),
    Draw,
    None
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe{board: [['-'; 3]; 3]}
    }
    pub fn print(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }
    pub fn check(&self) -> Result<Outcome, ()>
    {
        Ok(Outcome::None)
    }
}