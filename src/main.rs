#[derive(Debug, Clone)]
struct GameState {
    board: Vec<Option<char>>,
    current_player: char,
}

impl GameState {
    fn new() -> Self {
        GameState {
            board: vec![None; 9],
            current_player: 'X',
        }
    }

    fn display(&self) {
        println!("\nCurrent Board:");
        for row in self.board.chunks(3) {
            println!(" {} | {} | {} ",
                Self::symbol(row[0]),
                Self::symbol(row[1]),
                Self::symbol(row[2])
            );
            println!("---+---+---");
        }
    }

    fn symbol(square: Option<char>) -> char {
        match square {
            Some(c) => c,
            None => ' ',
        }
    }

    fn play_turn(&mut self) {
        loop {
            println!("Player {}, enter a position (1-9):", self.current_player);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if let Ok(position) = input.trim().parse::<usize>() {
                if position >= 1 && position <= 9 && self.board[position - 1].is_none() {
                    self.board[position - 1] = Some(self.current_player);
                    break;
                }
            }
            println!("Invalid input. Please try again.");
        }

        self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
    }

    fn check_winner(&self) -> Option<char> {
        let winning_combinations = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],           // Diagonals
        ];

        for combo in &winning_combinations {
            if let (Some(a), Some(b), Some(c)) = (
                self.board[combo[0]],
                self.board[combo[1]],
                self.board[combo[2]],
            ) {
                if a == b && b == c {
                    return Some(a);
                }
            }
        }
        None
    }
}

fn main() {
    let mut game = GameState::new();

    loop {
        game.display();
        game.play_turn();

        if let Some(winner) = game.check_winner() {
            game.display();
            println!("Player {} wins!", winner);
            break;
        }

        if game.board.iter().all(|&square| square.is_some()) {
            game.display();
            println!("It's a draw!");
            break;
        }
    }
}