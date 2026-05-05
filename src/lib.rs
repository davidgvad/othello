use std::fmt;

pub const SIZE: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opponent(self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Black => write!(f, "Black"),
            Player::White => write!(f, "White"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Disc(Player),
}

impl Cell {
    pub fn player(self) -> Option<Player> {
        if let Cell::Disc(player) = self {
            Some(player)
        } else {
            None
        }
    }

    pub fn is_empty(self) -> bool {
        self.player().is_none()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveResult {
    Played,
    OpponentPassed,
    GameOver,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Option<Position> {
        if row < SIZE && col < SIZE {
            Some(Position { row, col })
        } else {
            None
        }
    }

    pub fn from_human(row: usize, col: usize) -> Option<Position> {
        if (1..=SIZE).contains(&row) && (1..=SIZE).contains(&col) {
            Position::new(row - 1, col - 1)
        } else {
            None
        }
    }

    pub fn row(self) -> usize {
        self.row
    }

    pub fn col(self) -> usize {
        self.col
    }

    pub fn human_row(self) -> usize {
        self.row + 1
    }

    pub fn human_col(self) -> usize {
        self.col + 1
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.human_row(), self.human_col())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MoveError {
    OutOfBounds { row: usize, col: usize },
    Occupied { position: Position },
    NoFlips { player: Player, position: Position },
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::OutOfBounds { row, col } => write!(
                f,
                "Rows and columns must be between 1 and {}. You entered row {}, column {}.",
                SIZE,
                row.saturating_add(1),
                col.saturating_add(1)
            ),
            MoveError::Occupied { position } => {
                write!(f, "Position {position} is already occupied.")
            }
            MoveError::NoFlips { player, position } => {
                write!(
                    f,
                    "{player} cannot move at {position} because it flips no discs."
                )
            }
        }
    }
}

impl std::error::Error for MoveError {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct DiscCounts {
    pub black: usize,
    pub white: usize,
}

impl DiscCounts {
    pub fn total(self) -> usize {
        self.black + self.white
    }

    pub fn winner(self) -> Option<Player> {
        if self.black > self.white {
            Some(Player::Black)
        } else if self.white > self.black {
            Some(Player::White)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Game {
    board: [[Cell; SIZE]; SIZE],
    turn: Player,
}

pub trait Strategy {
    fn choose_move(&self, game: &Game, player: Player) -> Option<Position>;
}

pub struct GreedyStrategy;

impl Strategy for GreedyStrategy {
    fn choose_move(&self, game: &Game, player: Player) -> Option<Position> {
        game.best_move_for(player)
    }
}

pub struct MinimaxStrategy {
    depth: usize,
}

impl MinimaxStrategy {
    pub fn new(depth: usize) -> MinimaxStrategy {
        MinimaxStrategy {
            depth: depth.max(1),
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn score_move_for(&self, game: &Game, player: Player, position: Position) -> Option<i32> {
        let mut future = game.clone();
        future.turn = player;

        if !future.is_valid_position(player, position) {
            return None;
        }

        future
            .play_position(position)
            .expect("position was checked as legal");

        Some(self.minimax(&future, self.depth - 1, player))
    }

    fn minimax(&self, game: &Game, depth: usize, ai_player: Player) -> i32 {
        if depth == 0 || game.is_game_over() {
            return game.evaluate_for(ai_player);
        }

        let current_player = game.turn();
        let moves = game.valid_positions(current_player);

        if moves.is_empty() {
            let mut passed = game.clone();
            passed.turn = current_player.opponent();
            return self.minimax(&passed, depth, ai_player);
        }

        let scores = moves.into_iter().map(|position| {
            let mut future = game.clone();
            future
                .play_position(position)
                .expect("position came from valid_positions");
            self.minimax(&future, depth - 1, ai_player)
        });

        if current_player == ai_player {
            scores.max().expect("moves is not empty")
        } else {
            scores.min().expect("moves is not empty")
        }
    }
}

impl Strategy for MinimaxStrategy {
    fn choose_move(&self, game: &Game, player: Player) -> Option<Position> {
        let mut search_game = game.clone();
        search_game.turn = player;

        search_game
            .valid_positions(player)
            .into_iter()
            .max_by_key(|position| {
                self.score_move_for(&search_game, player, *position)
                    .expect("position came from valid_positions")
            })
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Game {
        let mut board = [[Cell::Empty; SIZE]; SIZE];

        board[3][3] = Cell::Disc(Player::White);
        board[3][4] = Cell::Disc(Player::Black);
        board[4][3] = Cell::Disc(Player::Black);
        board[4][4] = Cell::Disc(Player::White);

        Game {
            board,
            turn: Player::Black,
        }
    }

    pub fn current_player_has_moves(&self) -> bool {
        !self.valid_positions(self.turn).is_empty()
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<Cell> {
        if row < SIZE && col < SIZE {
            Some(self.board[row][col])
        } else {
            None
        }
    }

    pub fn cell_at(&self, position: Position) -> Cell {
        self.board[position.row()][position.col()]
    }

    pub fn valid_positions(&self, player: Player) -> Vec<Position> {
        let mut positions = Vec::new();

        for row in 0..SIZE {
            for col in 0..SIZE {
                if let Some(position) = Position::new(row, col) {
                    if self.is_valid_position(player, position) {
                        positions.push(position);
                    }
                }
            }
        }

        positions
    }

    pub fn is_valid_position(&self, player: Player, position: Position) -> bool {
        self.cell_at(position) == Cell::Empty && !self.flips_for(player, position).is_empty()
    }

    pub fn flip_count_for(&self, player: Player, position: Position) -> usize {
        self.flips_for(player, position).len()
    }

    pub fn best_move_for(&self, player: Player) -> Option<Position> {
        self.valid_positions(player)
            .into_iter()
            .max_by_key(|position| self.flip_count_for(player, *position))
    }

    pub fn play(&mut self, row: usize, col: usize) -> Result<MoveResult, MoveError> {
        let Some(position) = Position::new(row, col) else {
            return Err(MoveError::OutOfBounds { row, col });
        };

        self.play_position(position)
    }

    pub fn play_position(&mut self, position: Position) -> Result<MoveResult, MoveError> {
        if self.cell_at(position) != Cell::Empty {
            return Err(MoveError::Occupied { position });
        }

        let flips = self.flips_for(self.turn, position);
        if flips.is_empty() {
            return Err(MoveError::NoFlips {
                player: self.turn,
                position,
            });
        }

        self.board[position.row()][position.col()] = Cell::Disc(self.turn);
        for flip in flips {
            self.board[flip.row()][flip.col()] = Cell::Disc(self.turn);
        }

        Ok(self.advance_turn())
    }

    pub fn score(&self) -> (usize, usize) {
        let counts = self.disc_counts();

        (counts.black, counts.white)
    }

    pub fn disc_counts(&self) -> DiscCounts {
        let mut black = 0;
        let mut white = 0;

        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.board[row][col] == Cell::Disc(Player::Black) {
                    black += 1;
                } else if self.board[row][col] == Cell::Disc(Player::White) {
                    white += 1;
                }
            }
        }

        DiscCounts { black, white }
    }

    pub fn empty_count(&self) -> usize {
        SIZE * SIZE - self.disc_counts().total()
    }

    pub fn winner(&self) -> Option<Player> {
        if self.is_game_over() {
            self.disc_counts().winner()
        } else {
            None
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.valid_positions(Player::Black).is_empty()
            && self.valid_positions(Player::White).is_empty()
    }

    fn evaluate_for(&self, player: Player) -> i32 {
        let disc_difference = self.disc_difference_for(player);

        if self.is_game_over() {
            return disc_difference * 1000;
        }

        let mobility_difference = self.valid_positions(player).len() as i32
            - self.valid_positions(player.opponent()).len() as i32;
        let corner_difference =
            self.corner_count(player) as i32 - self.corner_count(player.opponent()) as i32;

        disc_difference * 10 + mobility_difference * 5 + corner_difference * 50
    }

    fn disc_difference_for(&self, player: Player) -> i32 {
        let counts = self.disc_counts();
        let black = counts.black as i32;
        let white = counts.white as i32;

        match player {
            Player::Black => black - white,
            Player::White => white - black,
        }
    }

    fn corner_count(&self, player: Player) -> usize {
        [(0, 0), (0, SIZE - 1), (SIZE - 1, 0), (SIZE - 1, SIZE - 1)]
            .into_iter()
            .filter(|(row, col)| self.board[*row][*col] == Cell::Disc(player))
            .count()
    }

    fn advance_turn(&mut self) -> MoveResult {
        let next = self.turn.opponent();

        if !self.valid_positions(next).is_empty() {
            self.turn = next;
            MoveResult::Played
        } else if !self.valid_positions(self.turn).is_empty() {
            MoveResult::OpponentPassed
        } else {
            MoveResult::GameOver
        }
    }

    fn flips_for(&self, player: Player, position: Position) -> Vec<Position> {
        if self.cell_at(position) != Cell::Empty {
            return Vec::new();
        }

        let mut all_flips = Vec::new();

        for (row_step, col_step) in DIRECTIONS {
            let mut line = self.flips_in_direction(player, position, row_step, col_step);
            all_flips.append(&mut line);
        }

        all_flips
    }

    fn flips_in_direction(
        &self,
        player: Player,
        position: Position,
        row_step: isize,
        col_step: isize,
    ) -> Vec<Position> {
        self.collect_flips_in_direction(
            player,
            position.row() as isize + row_step,
            position.col() as isize + col_step,
            row_step,
            col_step,
            Vec::new(),
        )
    }

    fn collect_flips_in_direction(
        &self,
        player: Player,
        current_row: isize,
        current_col: isize,
        row_step: isize,
        col_step: isize,
        mut flips: Vec<Position>,
    ) -> Vec<Position> {
        if !Game::in_bounds(current_row, current_col) {
            return Vec::new();
        }

        let row_index = current_row as usize;
        let col_index = current_col as usize;
        let cell = self.board[row_index][col_index];

        if cell == Cell::Disc(player.opponent()) {
            flips.push(Position::new(row_index, col_index).expect("in_bounds was checked"));
            self.collect_flips_in_direction(
                player,
                current_row + row_step,
                current_col + col_step,
                row_step,
                col_step,
                flips,
            )
        } else if cell == Cell::Disc(player) && !flips.is_empty() {
            flips
        } else {
            Vec::new()
        }
    }

    fn in_bounds(row: isize, col: isize) -> bool {
        row >= 0 && row < SIZE as isize && col >= 0 && col < SIZE as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(row: usize, col: usize) -> Position {
        Position::new(row, col).expect("test position should be in bounds")
    }

    fn game_from(board: [[Cell; SIZE]; SIZE], turn: Player) -> Game {
        Game { board, turn }
    }

    #[test]
    fn new_game_uses_standard_setup() {
        let game = Game::new();

        assert_eq!(game.turn(), Player::Black);
        assert_eq!(game.score(), (2, 2));
        assert_eq!(game.empty_count(), 60);
        assert_eq!(game.cell(3, 3), Some(Cell::Disc(Player::White)));
        assert_eq!(game.cell(3, 4), Some(Cell::Disc(Player::Black)));
        assert_eq!(game.cell(4, 3), Some(Cell::Disc(Player::Black)));
        assert_eq!(game.cell(4, 4), Some(Cell::Disc(Player::White)));
    }

    #[test]
    fn starting_legal_moves_are_found_in_board_order() {
        let game = Game::new();

        assert_eq!(
            game.valid_positions(Player::Black),
            vec![
                position(2, 3),
                position(3, 2),
                position(4, 5),
                position(5, 4)
            ]
        );
        assert_eq!(
            game.valid_positions(Player::White),
            vec![
                position(2, 4),
                position(3, 5),
                position(4, 2),
                position(5, 3)
            ]
        );
    }

    #[test]
    fn playing_a_move_places_and_flips_discs() {
        let mut game = Game::new();

        let result = game.play(2, 3);

        assert_eq!(result, Ok(MoveResult::Played));
        assert_eq!(game.turn(), Player::White);
        assert_eq!(game.score(), (4, 1));
        assert_eq!(game.cell(2, 3), Some(Cell::Disc(Player::Black)));
        assert_eq!(game.cell(3, 3), Some(Cell::Disc(Player::Black)));
    }

    #[test]
    fn invalid_moves_return_specific_errors_without_mutating_board() {
        let mut game = Game::new();

        assert_eq!(
            game.play(3, 3),
            Err(MoveError::Occupied {
                position: position(3, 3)
            })
        );
        assert_eq!(
            game.play(0, 0),
            Err(MoveError::NoFlips {
                player: Player::Black,
                position: position(0, 0)
            })
        );
        assert_eq!(
            game.play(SIZE, 0),
            Err(MoveError::OutOfBounds { row: SIZE, col: 0 })
        );
        assert_eq!(game.turn(), Player::Black);
        assert_eq!(game.score(), (2, 2));
    }

    #[test]
    fn turn_is_skipped_when_opponent_has_no_legal_moves() {
        let mut board = [[Cell::Disc(Player::Black); SIZE]; SIZE];
        board[0][0] = Cell::Empty;
        board[0][1] = Cell::Disc(Player::White);
        board[0][6] = Cell::Disc(Player::White);
        board[0][7] = Cell::Empty;
        let mut game = game_from(board, Player::Black);

        let result = game.play(0, 0);

        assert_eq!(result, Ok(MoveResult::OpponentPassed));
        assert_eq!(game.turn(), Player::Black);
        assert_eq!(game.cell(0, 1), Some(Cell::Disc(Player::Black)));
        assert!(game.is_valid_position(Player::Black, position(0, 7)));
        assert!(game.valid_positions(Player::White).is_empty());
    }

    #[test]
    fn game_over_is_reported_when_neither_player_can_move() {
        let mut board = [[Cell::Disc(Player::Black); SIZE]; SIZE];
        board[0][0] = Cell::Empty;
        board[0][1] = Cell::Disc(Player::White);
        let mut game = game_from(board, Player::Black);

        let result = game.play(0, 0);

        assert_eq!(result, Ok(MoveResult::GameOver));
        assert!(game.is_game_over());
        assert_eq!(game.winner(), Some(Player::Black));
        assert_eq!(game.empty_count(), 0);
    }

    #[test]
    fn best_move_uses_flip_count_across_directions() {
        let mut board = [[Cell::Empty; SIZE]; SIZE];
        board[0][1] = Cell::Disc(Player::White);
        board[0][2] = Cell::Disc(Player::White);
        board[0][3] = Cell::Disc(Player::Black);
        board[1][0] = Cell::Disc(Player::White);
        board[2][0] = Cell::Disc(Player::Black);
        let game = game_from(board, Player::Black);
        let best = position(0, 0);

        assert_eq!(game.flip_count_for(Player::Black, best), 3);
        assert_eq!(game.best_move_for(Player::Black), Some(best));

        let strategy = GreedyStrategy;
        assert_eq!(strategy.choose_move(&game, Player::Black), Some(best));
    }

    #[test]
    fn minimax_can_prefer_a_corner_over_more_immediate_flips() {
        let mut board = [[Cell::Empty; SIZE]; SIZE];
        board[0][1] = Cell::Disc(Player::White);
        board[0][2] = Cell::Disc(Player::Black);
        board[4][1] = Cell::Disc(Player::White);
        board[4][2] = Cell::Disc(Player::White);
        board[4][3] = Cell::Disc(Player::White);
        board[4][4] = Cell::Disc(Player::Black);
        let game = game_from(board, Player::Black);
        let corner = position(0, 0);
        let bigger_flip = position(4, 0);

        assert_eq!(game.flip_count_for(Player::Black, corner), 1);
        assert_eq!(game.flip_count_for(Player::Black, bigger_flip), 3);

        let greedy = GreedyStrategy;
        let minimax = MinimaxStrategy::new(1);

        assert_eq!(greedy.choose_move(&game, Player::Black), Some(bigger_flip));
        assert_eq!(minimax.choose_move(&game, Player::Black), Some(corner));
        assert!(
            minimax
                .score_move_for(&game, Player::Black, corner)
                .unwrap()
                > minimax
                    .score_move_for(&game, Player::Black, bigger_flip)
                    .unwrap()
        );
    }

    #[test]
    fn cloned_game_state_can_restore_a_previous_turn() {
        let mut game = Game::new();
        let previous = game.clone();

        game.play(2, 3).expect("opening move should be legal");
        game = previous;
        assert_eq!(game.turn(), Player::Black);
        assert_eq!(game.score(), (2, 2));
        assert_eq!(game.cell(3, 3), Some(Cell::Disc(Player::White)));
    }
}
