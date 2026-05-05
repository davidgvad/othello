use eframe::egui::{
    self, Align, Button, CentralPanel, Color32, Context, CornerRadius, FontId, Grid, Layout,
    RichText, Stroke, Ui, Vec2,
};
use othello::{Cell, Game, MinimaxStrategy, MoveResult, Player, Position, Strategy, SIZE};

const AI_SEARCH_DEPTH: usize = 4;
const CELL_SIZE: f32 = 58.0;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([760.0, 680.0])
            .with_min_inner_size([620.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Othello",
        options,
        Box::new(|_cc| Ok(Box::new(OthelloApp::new()))),
    )
}

struct OthelloApp {
    game: Game,
    history: Vec<Game>,
    message: String,
}

impl OthelloApp {
    fn new() -> OthelloApp {
        OthelloApp {
            game: Game::new(),
            history: Vec::new(),
            message: String::from("Black moves first. Select a highlighted square."),
        }
    }

    fn restart(&mut self) {
        self.game = Game::new();
        self.history.clear();
        self.message = String::from("New game started. Black moves first.");
    }

    fn undo(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.game = previous;
            self.message = String::from("Last move undone.");
        } else {
            self.message = String::from("No moves to undo.");
        }
    }

    fn show_hint(&mut self) {
        self.message = match self.game.best_move_for(self.game.turn()) {
            Some(position) => format!(
                "Greedy hint for {}: {}. It flips {} disc(s).",
                self.game.turn(),
                position,
                self.game.flip_count_for(self.game.turn(), position)
            ),
            None => format!("{} has no legal moves.", self.game.turn()),
        };
    }

    fn play_ai_move(&mut self) {
        let strategy = MinimaxStrategy::new(AI_SEARCH_DEPTH);
        let player = self.game.turn();

        self.message = match strategy.choose_move(&self.game, player) {
            Some(position) => {
                let score = strategy
                    .score_move_for(&self.game, player, position)
                    .expect("AI chose a legal move");
                let result = self.play_position(position);

                format!(
                    "AI searched {} future move(s) and chose {position} for {player} with score {score}. {result}",
                    strategy.depth()
                )
            }
            None => format!("{player} has no legal moves."),
        };
    }

    fn play_position(&mut self, position: Position) -> String {
        let previous = self.game.clone();

        match self.game.play_position(position) {
            Ok(result) => {
                self.history.push(previous);
                self.move_result_text(result)
            }
            Err(error) => error.to_string(),
        }
    }

    fn move_result_text(&self, result: MoveResult) -> String {
        match result {
            MoveResult::Played => format!("Move accepted. {} is up next.", self.game.turn()),
            MoveResult::OpponentPassed => format!(
                "The opponent has no legal moves, so {} gets another turn.",
                self.game.turn()
            ),
            MoveResult::GameOver => self.final_result_text(),
        }
    }

    fn final_result_text(&self) -> String {
        let counts = self.game.disc_counts();
        let winner = match self.game.winner() {
            Some(player) => format!("{player} wins."),
            None => String::from("It is a tie."),
        };

        format!(
            "Game over. Final score: Black {}, White {}. {}",
            counts.black, counts.white, winner
        )
    }

    fn draw_header(&self, ui: &mut Ui) {
        let counts = self.game.disc_counts();

        ui.horizontal(|ui| {
            ui.heading("Othello");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(format!("Empty: {}", self.game.empty_count())).strong());
                ui.separator();
                ui.label(RichText::new(format!("White: {}", counts.white)).strong());
                ui.separator();
                ui.label(RichText::new(format!("Black: {}", counts.black)).strong());
            });
        });

        ui.add_space(4.0);

        if self.game.is_game_over() {
            ui.label(RichText::new(self.final_result_text()).size(18.0).strong());
        } else {
            ui.label(
                RichText::new(format!("Turn: {}", self.game.turn()))
                    .size(18.0)
                    .strong(),
            );
        }
    }

    fn draw_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Hint").clicked() {
                self.show_hint();
            }

            if ui.button("AI Move").clicked() {
                self.play_ai_move();
            }

            if ui
                .add_enabled(!self.history.is_empty(), Button::new("Undo"))
                .clicked()
            {
                self.undo();
            }

            if ui.button("Restart").clicked() {
                self.restart();
            }
        });
    }

    fn draw_board(&mut self, ui: &mut Ui) {
        let legal_moves = self.game.valid_positions(self.game.turn());

        Grid::new("board")
            .spacing(Vec2::new(5.0, 5.0))
            .show(ui, |ui| {
                ui.label("");
                for col in 0..SIZE {
                    ui.label(RichText::new((col + 1).to_string()).strong());
                }
                ui.end_row();

                for row in 0..SIZE {
                    ui.label(RichText::new((row + 1).to_string()).strong());

                    for col in 0..SIZE {
                        let position =
                            Position::new(row, col).expect("board coordinates are in bounds");
                        let is_legal = legal_moves.contains(&position);
                        let cell = self.game.cell_at(position);
                        let response = draw_cell(ui, cell, is_legal);

                        if response.clicked() {
                            self.message = self.play_position(position);
                        }
                    }

                    ui.end_row();
                }
            });
    }
}

impl eframe::App for OthelloApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            self.draw_header(ui);
            ui.add_space(12.0);
            self.draw_controls(ui);
            ui.add_space(12.0);

            ui.group(|ui| {
                ui.set_min_height(52.0);
                ui.label(RichText::new(&self.message).size(16.0));
            });

            ui.add_space(14.0);
            self.draw_board(ui);
        });
    }
}

fn draw_cell(ui: &mut Ui, cell: Cell, is_legal: bool) -> egui::Response {
    let fill = if is_legal {
        Color32::from_rgb(45, 132, 82)
    } else {
        Color32::from_rgb(36, 104, 68)
    };
    let stroke = Stroke::new(1.0, Color32::from_rgb(22, 64, 45));
    let response = ui.add(
        Button::new("")
            .min_size(Vec2::splat(CELL_SIZE))
            .fill(fill)
            .stroke(stroke)
            .corner_radius(CornerRadius::same(6)),
    );

    let painter = ui.painter_at(response.rect);
    let center = response.rect.center();
    let radius = CELL_SIZE * 0.34;

    match cell {
        Cell::Disc(Player::Black) => {
            painter.circle_filled(center, radius, Color32::from_rgb(22, 24, 28));
            painter.circle_stroke(center, radius, Stroke::new(1.5, Color32::from_gray(80)));
        }
        Cell::Disc(Player::White) => {
            painter.circle_filled(center, radius, Color32::from_rgb(235, 231, 220));
            painter.circle_stroke(center, radius, Stroke::new(1.5, Color32::from_gray(170)));
        }
        Cell::Empty if is_legal => {
            painter.circle_stroke(
                center,
                radius * 0.48,
                Stroke::new(3.0, Color32::from_rgb(181, 228, 164)),
            );
        }
        Cell::Empty => {}
    }

    if is_legal {
        painter.text(
            response.rect.left_top() + Vec2::new(6.0, 5.0),
            egui::Align2::LEFT_TOP,
            "*",
            FontId::proportional(18.0),
            Color32::from_rgb(181, 228, 164),
        );
    }

    response
}
