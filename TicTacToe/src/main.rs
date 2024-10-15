mod tic_tac_toe;

fn main() {
    let field = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut index=0;

    for val in field.iter() {
        print!("Got: {val} {}",index+1);
        index+=1;
        if(index%3==0)
        {
            println!();
        }
    }

    let mut game = tic_tac_toe::TicTacToe::new();
    game.print();
}
