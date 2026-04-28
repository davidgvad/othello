use std::fmt;

pub const SIZE: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opponent(self) -> Player {
        if self == Player::Black {
            Player::White
        } else {
            Player::Black
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Player::Black {
            write!(f, "Black")
        } else {
            write!(f, "White")
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Disc(Player),
}

impl Cell {
    pub fn player(&self) -> Option<Player> {
        if let Cell::Disc(player) = self {
            Some(*player)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.player().is_none()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MoveResult {
    Played,
    OpponentPassed,
    GameOver,
}

pub struct Game {
    board: [[Cell; SIZE]; SIZE],
    turn: Player,
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

    pub fn valid_moves(&self, player: Player) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.is_valid_move(player, row, col) {
                    moves.push((row, col));
                }
            }
        }

        moves
    }

    pub fn is_valid_move(&self, player: Player, row: usize, col: usize) -> bool {
        self.cell(row, col) == Some(Cell::Empty) && !self.flips_for(player, row, col).is_empty()
    }

    pub fn play(&mut self, row: usize, col: usize) -> Result<MoveResult, String> {
        let flips = self.flips_for(self.turn, row, col);

        if self.cell(row, col) != Some(Cell::Empty) || flips.is_empty() {
            return Err(format!(
                "Invalid move for {} at row {}, column {}",
                self.turn,
                row.saturating_add(1),
                col.saturating_add(1)
            ));
        }

        self.board[row][col] = Cell::Disc(self.turn);
        for (flip_row, flip_col) in flips {
            self.board[flip_row][flip_col] = Cell::Disc(self.turn);
        }

        Ok(self.advance_turn())
    }

    pub fn score(&self) -> (usize, usize) {
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

        (black, white)
    }

    pub fn is_game_over(&self) -> bool {
        self.valid_moves(Player::Black).is_empty() && self.valid_moves(Player::White).is_empty()
    }

    fn advance_turn(&mut self) -> MoveResult {
        let next = self.turn.opponent();

        if !self.valid_moves(next).is_empty() {
            self.turn = next;
            MoveResult::Played
        } else if !self.valid_moves(self.turn).is_empty() {
            MoveResult::OpponentPassed
        } else {
            MoveResult::GameOver
        }
    }

    fn flips_for(&self, player: Player, row: usize, col: usize) -> Vec<(usize, usize)> {
        if row >= SIZE || col >= SIZE || self.board[row][col] != Cell::Empty {
            return Vec::new();
        }

        let mut all_flips = Vec::new();

        for (row_step, col_step) in DIRECTIONS {
            let mut line = self.flips_in_direction(player, row, col, row_step, col_step);
            all_flips.append(&mut line);
        }

        all_flips
    }

    fn flips_in_direction(
        &self,
        player: Player,
        row: usize,
        col: usize,
        row_step: isize,
        col_step: isize,
    ) -> Vec<(usize, usize)> {
        let mut flips = Vec::new();
        let mut current_row = row as isize + row_step;
        let mut current_col = col as isize + col_step;

        while Game::in_bounds(current_row, current_col) {
            let row_index = current_row as usize;
            let col_index = current_col as usize;
            let cell = self.board[row_index][col_index];

            if cell == Cell::Disc(player.opponent()) {
                flips.push((row_index, col_index));
            } else if cell == Cell::Disc(player) {
                if flips.is_empty() {
                    return Vec::new();
                }

                return flips;
            } else {
                return Vec::new();
            }

            current_row += row_step;
            current_col += col_step;
        }

        Vec::new()
    }
    fn in_bounds(row: isize, col: isize) -> bool {
        row >= 0 && row < SIZE as isize && col >= 0 && col < SIZE as isize
    }
}
