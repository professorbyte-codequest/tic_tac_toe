use std::str::FromStr;

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
            println!(
                " {} | {} | {} ",
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
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // Rows
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // Columns
            [0, 4, 8],
            [2, 4, 6], // Diagonals
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

impl FromStr for GameState {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut board = vec![None; 9];
        let mut current_player = 'X';

        let trimmed_input = input.trim();
        if !trimmed_input.is_empty() {
            for (i, ch) in trimmed_input.chars().enumerate() {
                if i >= 9 {
                    return Err("Input too long".to_string());
                }
                match ch {
                    'X' | 'O' => board[i] = Some(ch),
                    '_' => (), // Empty square
                    _ => return Err("Invalid character in input".to_string()),
                }
            }

            // Infer current player based on counts
            let x_count = board.iter().filter(|&&sq| sq == Some('X')).count();
            let o_count = board.iter().filter(|&&sq| sq == Some('O')).count();

            current_player = if x_count > o_count { 'O' } else { 'X' };
        }

        Ok(GameState {
            board,
            current_player,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_winner_row() {
        let mut game = GameState::new();
        game.board = vec![
            Some('X'),
            Some('X'),
            Some('X'),
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        assert_eq!(game.check_winner(), Some('X'));
    }

    #[test]
    fn test_check_winner_diagonal() {
        let mut game = GameState::new();
        game.board = vec![
            Some('O'),
            None,
            None,
            None,
            Some('O'),
            None,
            None,
            None,
            Some('O'),
        ];
        assert_eq!(game.check_winner(), Some('O'));
    }

    #[test]
    fn test_no_winner() {
        let mut game = GameState::new();
        game.board = vec![
            Some('X'),
            Some('O'),
            Some('X'),
            None,
            Some('X'),
            None,
            None,
            None,
            Some('O'),
        ];
        assert_eq!(game.check_winner(), None);
    }

    #[test]
    fn test_invalid_input() {
        let mut game = GameState::new();
        game.board = vec![None; 9];

        let input = "10";
        assert!(matches!(GameState::from_str(input), Err(_)));
    }

    #[test]
    fn test_from_str_valid() {
        let input = "XOX____OX";
        let game = GameState::from_str(input).unwrap();
        assert_eq!(game.board[0], Some('X'));
        assert_eq!(game.board[8], Some('X'));
        assert_eq!(game.current_player, 'O');
    }

    #[test]
    fn test_from_str_invalid_character() {
        let input = "XOQ____OX";
        assert!(GameState::from_str(input).is_err());
    }

    #[test]
    fn test_from_str_empty() {
        let input = "_________";
        let game = GameState::from_str(input).unwrap();
        assert!(game.board.iter().all(|&sq| sq.is_none()));
        assert_eq!(game.current_player, 'X');
    }
}
