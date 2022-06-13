use std::fmt;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    None,
    X,
    O,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match self {
            Player::None => " ",
            Player::X => "X",
            Player::O => "O",
        };
        write!(f, "{}", p)
    }
}

struct Game {
    board: [Player; 9],
    winner: Player,
    current_player: Player,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let divider = "-----------";
        let row1 = format!(" {} | {} | {} ", self.board[0], self.board[1], self.board[2]);
        let row2 = format!(" {} | {} | {} ", self.board[3], self.board[4], self.board[5]);
        let row3 = format!(" {} | {} | {} ", self.board[6], self.board[7], self.board[8]);
        
        write!(f, "{}\n{}\n{}\n{}\n{}", row1, divider, row2, divider, row3)
    }
}


fn main() {
    let mut game = Game {
        board: [Player::None; 9],
        winner: Player::None,
        current_player: Player::X,
    };

    loop {
        println!("\n{}", game);
        println!("Pick a field 0-8: ");
        let field = input_board_field();

        if matches!(game.board[field], Player::None) {
            game.board[field] = game.current_player;
        } else {
            println!("That field is already controlled!");
            continue;
        }

        check_winner(&mut game);

        game.current_player = match game.current_player {
            Player::None => Player::None,
            Player::X => Player::O,
            Player::O => Player::X,
        };

        match game.winner {
            Player::None => {
                if are_all_fields_taken(game.board) {
                    println!("\n{}", game);
                    println!("It's a draw!");
                    break;
                }
                continue;
            },
            _ => {
                println!("\n{}", game);
                println!("Winner: {}", game.winner);
                break;
            }
        }
    }
}

fn input_board_field() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Please type a number!");

    let input: usize = match input.trim().parse() {
        Ok(num) => {
            if num > 8 {
                println!("Please provide a number between 0 and 8!");
                return input_board_field();
            }
            num
        },
        Err(_) => {
            println!("Please provide a valid number!");
            return input_board_field();
        },
    };

    return input;
}

fn check_winner(game: &mut Game) -> &Game {
    let is_winning = match game.board {
        [Player::X,Player::X,Player::X, _,_,_, _,_,_] => true,
        [_,_,_, Player::X,Player::X,Player::X, _,_,_] => true,
        [_,_,_,  _,_,_, Player::X,Player::X,Player::X] => true,

        [Player::X,_,_,  Player::X,_,_, Player::X,_,_] => true,
        [_,Player::X,_,  _,Player::X,_, _,Player::X,_] => true,
        [_,_,Player::X,  _,_,Player::X, _,_,Player::X] => true,

        [Player::X,_,_,  _,Player::X,_, _,_,Player::X] => true,
        [_,_,Player::X,  _,Player::X,_, Player::X,_,_] => true,

        [Player::O,Player::O,Player::O, _,_,_, _,_,_] => true,
        [_,_,_, Player::O,Player::O,Player::O, _,_,_] => true,
        [_,_,_,  _,_,_, Player::O,Player::O,Player::O] => true,

        [Player::O,_,_,  Player::O,_,_, Player::O,_,_] => true,
        [_,Player::O,_,  _,Player::O,_, _,Player::O,_] => true,
        [_,_,Player::O,  _,_,Player::O, _,_,Player::O] => true,

        [Player::O,_,_,  _,Player::O,_, _,_,Player::O] => true,
        [_,_,Player::O,  _,Player::O,_, Player::O,_,_] => true,
        _ => false,
    };

    if is_winning {
        game.winner = game.current_player;
    }

    return game;
}

fn are_all_fields_taken(board: [Player; 9]) -> bool {
    return !board.iter().any(|&field| field == Player::None);
}
