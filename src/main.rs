use std::io::{self, Write};

use othello::{Cell, Game, GreedyStrategy, MoveResult, Player, Position, Strategy, SIZE};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";

enum Command {
    Play(Position),
    Help,
    Hint,
    Ai,
    Score,
    Undo,
    Quit,
}

fn main() {
    let mut game = Game::new();
    let mut history: Vec<Game> = Vec::new();
    let mut message = String::from("Black moves first. Enter a highlighted move like: 3 4");

    loop {
        render(&game, &message);

        if game.is_game_over() {
            print_winner(&game);
            break;
        }

        match read_command() {
            Ok(Command::Play(position)) => {
                message = play_position(&mut game, &mut history, position);
            }
            Ok(Command::Help) => {
                message = help_text();
            }
            Ok(Command::Hint) => {
                message = hint_text(&game);
            }
            Ok(Command::Ai) => {
                message = play_ai_move(&mut game, &mut history);
            }
            Ok(Command::Score) => {
                let counts = game.disc_counts();
                message = format!(
                    "Scoreboard: Black {}, White {}, Empty {}.",
                    counts.black,
                    counts.white,
                    game.empty_count()
                );
            }
            Ok(Command::Undo) => {
                if let Some(previous) = history.pop() {
                    game = previous;
                    message = String::from("Last move undone.");
                } else {
                    message = String::from("No moves to undo.");
                }
            }
            Ok(Command::Quit) => {
                println!();
                println!("Game ended early.");
                print_winner(&game);
                break;
            }
            Err(error) => {
                message = error;
            }
        }
    }
}

fn render(game: &Game, message: &str) {
    print!("{CLEAR_SCREEN}");
    println!("{BOLD}Othello / Reversi{RESET}");
    println!("{DIM}Commands: row column | hint | ai | undo | score | help | quit{RESET}");
    println!();

    print_board(game);
    print_status(game);
    println!("{CYAN}{message}{RESET}");
    println!();
}

fn print_board(game: &Game) {
    let legal_moves = game.valid_positions(game.turn());

    println!("    1 2 3 4 5 6 7 8");
    println!("   -----------------");

    for row in 0..SIZE {
        print!("{} | ", row + 1);

        for col in 0..SIZE {
            let position = Position::new(row, col).expect("board coordinates are in bounds");
            let cell = game.cell_at(position);
            let is_legal = legal_moves.contains(&position);

            print_cell(cell, is_legal);
        }

        println!();
    }
}

fn print_cell(cell: Cell, is_legal: bool) {
    match (cell, is_legal) {
        (Cell::Disc(Player::Black), _) => print!("{BOLD}B{RESET} "),
        (Cell::Disc(Player::White), _) => print!("{YELLOW}W{RESET} "),
        (Cell::Empty, true) => print!("{GREEN}*{RESET} "),
        (Cell::Empty, false) => print!("{DIM}.{RESET} "),
    }
}

fn print_status(game: &Game) {
    let counts = game.disc_counts();

    println!();
    println!(
        "Turn: {BOLD}{}{RESET} | Black: {} | White: {} | Empty: {}",
        game.turn(),
        counts.black,
        counts.white,
        game.empty_count()
    );

    if game.current_player_has_moves() {
        println!("Legal moves: {}", legal_moves_text(game));
    } else {
        println!("Legal moves: none");
    }

    println!();
}

fn read_command() -> Result<Command, String> {
    loop {
        print!("Move: ");
        io::stdout()
            .flush()
            .map_err(|error| format!("Could not write prompt: {error}"))?;

        let mut input = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .map_err(|error| format!("Could not read input: {error}"))?;

        if bytes_read == 0 {
            return Ok(Command::Quit);
        }

        match parse_command(&input) {
            Ok(command) => return Ok(command),
            Err(error) => {
                println!("{error}");
                println!();
            }
        }
    }
}

fn parse_command(input: &str) -> Result<Command, String> {
    let trimmed = input.trim().to_ascii_lowercase();

    match trimmed.as_str() {
        "" => return Err(String::from("Enter a move or command.")),
        "h" | "help" => return Ok(Command::Help),
        "hint" => return Ok(Command::Hint),
        "ai" => return Ok(Command::Ai),
        "s" | "score" => return Ok(Command::Score),
        "u" | "undo" => return Ok(Command::Undo),
        "q" | "quit" | "exit" => return Ok(Command::Quit),
        _ => {}
    }

    let parts = trimmed.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(String::from("Use two numbers, like: 3 4"));
    }

    let row = parts[0]
        .parse::<usize>()
        .map_err(|_| String::from("Rows and columns must be numbers from 1 to 8."))?;
    let col = parts[1]
        .parse::<usize>()
        .map_err(|_| String::from("Rows and columns must be numbers from 1 to 8."))?;

    Position::from_human(row, col)
        .map(Command::Play)
        .ok_or_else(|| String::from("Rows and columns must be numbers from 1 to 8."))
}

fn legal_moves_text(game: &Game) -> String {
    let moves = game
        .valid_positions(game.turn())
        .into_iter()
        .map(|position| position.to_string())
        .collect::<Vec<_>>();

    if moves.is_empty() {
        String::from("none")
    } else {
        moves.join(", ")
    }
}

fn help_text() -> String {
    [
        "Enter moves as row column, for example: 3 4",
        "Use hint for a suggested move, ai to let the greedy strategy play, and undo to revert.",
        "Use score to show counts and quit to end early.",
        "Green * marks every legal move for the current player.",
    ]
    .join("\n")
}

fn hint_text(game: &Game) -> String {
    match game.best_move_for(game.turn()) {
        Some(position) => format!(
            "Suggested move for {}: {}. It flips {} disc(s).",
            game.turn(),
            position,
            game.flip_count_for(game.turn(), position)
        ),
        None => format!("{} has no legal moves.", game.turn()),
    }
}

fn play_ai_move(game: &mut Game, history: &mut Vec<Game>) -> String {
    let strategy = GreedyStrategy;
    let player = game.turn();

    match strategy.choose_move(game, player) {
        Some(position) => {
            let result = play_position(game, history, position);
            format!("Greedy AI chose {position} for {player}. {result}")
        }
        None => format!("{player} has no legal moves."),
    }
}

fn play_position(game: &mut Game, history: &mut Vec<Game>, position: Position) -> String {
    let previous = game.clone();

    match game.play_position(position) {
        Ok(result) => {
            history.push(previous);
            move_result_text(game, result)
        }
        Err(error) => error.to_string(),
    }
}

fn move_result_text(game: &Game, result: MoveResult) -> String {
    match result {
        MoveResult::Played => format!("Move accepted. {} is up next.", game.turn()),
        MoveResult::OpponentPassed => format!(
            "The opponent has no legal moves, so {} gets another turn.",
            game.turn()
        ),
        MoveResult::GameOver => String::from("No legal moves remain for either player."),
    }
}

fn print_winner(game: &Game) {
    let counts = game.disc_counts();

    println!(
        "Final score: Black {}, White {}",
        counts.black, counts.white
    );

    match game.winner() {
        Some(player) => println!("{player} wins."),
        None => println!("It is a tie."),
    }
}
