use std::{collections::btree_set::Difference, str::FromStr};

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone)]
struct GameState {
    board: Vec<Option<char>>,
    current_player: char,
    difficulty: u8, // 1: Random, 2: Best Move, 3: Minimax
}

impl GameState {
    fn new() -> Self {
        Self::with_difficulty(2)
    }

    fn with_difficulty(difficulty: u8) -> Self {
        GameState {
            board: vec![None; 9],
            current_player: 'X',
            difficulty,
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

    fn available_moves(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, &square)| if square.is_none() { Some(i) } else { None })
            .collect()
    }

    fn would_move_win(&self, player: char, i: usize) -> bool {
        let mut simulated_board = self.board.clone();
        simulated_board[i] = Some(player);
        let simulated_state = GameState {
            board: simulated_board,
            current_player: player,
            difficulty: self.difficulty,
        };

        simulated_state.check_winner() == Some(player)
    }

    pub fn best_move(&self, player: char) -> Option<usize> {
        let opponent = if player == 'X' { 'O' } else { 'X' };

        let available_moves = self.available_moves();

        // Step 1: Check for a winning move
        for i in available_moves.iter() {
            if self.would_move_win(player, *i) {
                return Some(*i);
            }
        }

        // Step 2: Check for a blocking move
        for i in available_moves.iter() {
            if self.would_move_win(opponent, *i) {
                return Some(*i);
            }
        }

        if self.difficulty > 2 {
            // Step 3: Choose the best available square
            let priority_squares = [4, 0, 2, 6, 8, 1, 3, 5, 7];
            for &i in &priority_squares {
                if self.board[i].is_none() {
                    return Some(i);
                }
            }
        }

        self.random_move()
    }

    pub fn ai_move(&mut self, player: char) {
        let move_index = match self.difficulty {
            1 => self.random_move(),
            2 => self.best_move(player),
            3 => self.best_move(player),
            _ => panic!("Unknown diffoculty level!"),
        };

        if let Some(i) = move_index {
            self.board[i] = Some('O');
        } else {
            panic!("No available moves!");
        }

        self.current_player = 'X';
    }

    fn random_move(&self) -> Option<usize> {
        let empty_squares = self.available_moves();
        if empty_squares.is_empty() {
            None
        } else {
            empty_squares.choose(&mut thread_rng()).copied()
        }
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
            difficulty: 1,
        })
    }
}

fn main() {
    println!("Choose AI difficulty: 1 (Easy), 2 (Normal) or 3 (Hard):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let difficulty = input.trim().parse::<u8>().unwrap_or(2);

    let mut game = GameState::with_difficulty(difficulty);

    loop {
        game.display();

        if game.current_player == 'X' {
            // Human player
            game.play_turn();
        } else {
            // Machine player
            game.ai_move('O');
        }

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

    #[test]
    fn test_best_move_winning() {
        let mut game = GameState::new();
        game.board = vec![
            Some('X'),
            Some('X'),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        assert_eq!(game.best_move('X'), Some(2));
    }

    #[test]
    fn test_best_move_blocking() {
        let mut game = GameState::new();
        game.board = vec![
            Some('O'),
            Some('O'),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        assert_eq!(game.best_move('X'), Some(2));
    }

    #[test]
    fn test_best_move_priority() {
        let mut game = GameState::new();
        game.board = vec![None, None, None, None, None, None, None, None, None];
        assert_eq!(game.best_move('X'), Some(4)); // Center square
    }
}
