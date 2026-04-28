use std::io::{self, Write};

use othello::{Cell, Game, MoveResult, Player, SIZE};

fn main() {
    let mut game = Game::new();

    println!("Othello/Reversi");
    println!("Enter moves as: row column. Example: 3 4");

    while !game.is_game_over() {
        print_board(&game);
        print_turn(&game);

        let (row, col) = read_move(&game);

        let result = game.play(row, col);

        if result.is_err() {
            println!("{}", result.err().unwrap());
        } else {
            let move_result = result.ok().unwrap();

            if move_result == MoveResult::OpponentPassed {
                println!("The other player has no legal moves, so {} goes again.", game.turn());
            }
        }
    }

    print_board(&game);
    print_winner(&game);
}

fn print_board(game: &Game) {
    println!();
    println!("    1 2 3 4 5 6 7 8");
    println!("   -----------------");

    for row in 0..SIZE {
        print!("{} | ", row + 1);

        for col in 0..SIZE {
            let cell = game.cell(row, col).unwrap();
            let symbol = if cell == Cell::Empty {
                '.'
            } else if cell == Cell::Disc(Player::Black) {
                'B'
            } else {
                'W'
            };

            print!("{symbol} ");
        }

        println!();
    }

    let (black, white) = game.score();
    println!();
    println!("Score: Black {black}, White {white}");
}

fn print_turn(game: &Game) {
    let moves = game
        .valid_moves(game.turn())
        .into_iter()
        .map(|(row, col)| format!("{} {}", row + 1, col + 1))
        .collect::<Vec<_>>()
        .join(", ");

    println!("{} to move.", game.turn());
    println!("Legal moves: {moves}");
}

fn read_move(game: &Game) -> (usize, usize) {
    loop {
        print!("Move: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter two numbers, like: 3 4");
            continue;
        }

        let row = parts[0].parse::<usize>();
        let col = parts[1].parse::<usize>();

        if row.is_ok() && col.is_ok() {
            let row = row.unwrap();
            let col = col.unwrap();

            if (1..=SIZE).contains(&row) && (1..=SIZE).contains(&col) {
                let row_index = row - 1;
                let col_index = col - 1;

                if game.is_valid_move(game.turn(), row_index, col_index) {
                    return (row_index, col_index);
                }

                println!("That move does not flip any discs.");
            } else {
                println!("Rows and columns must be numbers from 1 to 8.");
            }
        } else {
            println!("Rows and columns must be numbers from 1 to 8.");
        }
    }
}

fn print_winner(game: &Game) {
    let (black, white) = game.score();

    println!("Final score: Black {black}, White {white}");

    if black > white {
        println!("Black wins.");
    } else if white > black {
        println!("White wins.");
    } else {
        println!("It is a tie.");
    }
}
