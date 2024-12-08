pub mod board;
pub mod piece;

use crate::game::board::Board;
use crate::game::piece::domino::Domino;
use crate::game::piece::Piece;
use rand::Rng;

use ggez::{
    // context,
    event,
    graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context,
    GameResult,
};

const GRID_CELL_SIZE: i32 = 32;
const CELL_RADIUS: i32 = 2;

struct Params {
    grid_size: (i32, i32),
    screen_size: (f32, f32),
}

impl Params {
    fn new(rows: u32, columns: u32) -> Params {
        Params {
            grid_size: (columns as i32, rows as i32),
            screen_size: (
                columns as f32 * GRID_CELL_SIZE as f32,
                rows as f32 * GRID_CELL_SIZE as f32,
            ),
        }
    }
}

struct GameState {
    gameboard: Board,
    piece: Domino,
    params: Params,
}

impl GameState {
    pub fn new(params: Params) -> GameState {
        let gameboard = Board::new(params.grid_size.0 as usize, params.grid_size.1 as usize);
        let piece = Domino::new(&gameboard);
        GameState {
            gameboard,
            piece,
            params,
        }
    }

    fn big_square(&self, x: usize, y: usize) -> graphics::Rect {
        graphics::Rect::new_i32(
            GRID_CELL_SIZE * (x as i32),
            GRID_CELL_SIZE * (self.params.grid_size.1 - 1 - y as i32),
            GRID_CELL_SIZE,
            GRID_CELL_SIZE,
        )
    }

    fn inside_square(&self, x: usize, y: usize) -> graphics::Rect {
        graphics::Rect::new_i32(
            GRID_CELL_SIZE * x as i32 + CELL_RADIUS,
            GRID_CELL_SIZE * (self.params.grid_size.1 - 1 - y as i32) + CELL_RADIUS,
            GRID_CELL_SIZE - 2 * CELL_RADIUS,
            GRID_CELL_SIZE - 2 * CELL_RADIUS,
        )
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => self.piece.up(&self.gameboard),
            Some(KeyCode::Left) => self.piece.left_slide(&self.gameboard),
            Some(KeyCode::Right) => self.piece.right_slide(&self.gameboard),
            Some(KeyCode::Down) => self.piece.drop(&self.gameboard),
            Some(KeyCode::D) => self.piece.clock_rotation(&self.gameboard),
            Some(KeyCode::A) => self.piece.anticlock_rotation(&self.gameboard),
            Some(KeyCode::Space) => {
                if self.piece.fix(&self.gameboard) {
                    self.gameboard.merge(&self.piece).clear_rows();
                    self.piece = Domino::new(&self.gameboard);
                    // random initial orientation
                    // let mut rng = rand::thread_rng();
                    // let col = rng.gen_range(0..2);
                    // if col == 0 {
                    //     self.piece.self.piece.clock_rotation(&self.gameboard)
                    // }
                } else {
                    self.piece.hard_drop(&self.gameboard)
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        for cell in self.gameboard.cells() {
            // get the color
            let color = match cell.state() {
                board::CellState::Filled => [0.0, 1.0, 1.0, 1.0],
                board::CellState::Empty => [0.1, 0.1, 0.1, 1.0],
            };

            // outside rect
            let darker = [color[0] * 0.8, color[1] * 0.8, color[2] * 0.8, 1.0];
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(self.big_square(cell.x(), cell.y()))
                    .color(darker),
            );

            // inside rectanngle
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(self.inside_square(cell.x(), cell.y()))
                    .color(color),
            );
        }

        for cell in self.piece.positions() {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(self.big_square(cell.x, cell.y))
                    .color([0.5, 0.0, 1.0, 1.0]),
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(self.inside_square(cell.x, cell.y))
                    .color([0.3, 0.0, 0.8, 1.0]),
            );
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn game(rows: u32, columns: u32, pieces: u32) -> GameResult {
    let params = Params::new(rows, columns);

    let (ctx, events_loop) = ggez::ContextBuilder::new("snake", "Gray Olson")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(params.screen_size.0, params.screen_size.1),
        )
        .build()?;

    let mut state = GameState::new(params);
    let mut rng = rand::thread_rng();
    for _ in 0..pieces {
        let col = rng.gen_range(0..state.gameboard.width / 2);
        let left = matches!(rng.gen_range(0..=1), 1);
        for _ in 0..col {
            if left {
                state.piece.left_slide(&state.gameboard);
            } else {
                state.piece.right_slide(&state.gameboard);
            }
        }
        for _ in 0..100 {
            match rng.gen_range(0..=4) {
                0 => state.piece.drop(&state.gameboard),
                1 => state.piece.left_slide(&state.gameboard),
                2 => state.piece.right_slide(&state.gameboard),
                3 => state.piece.clock_rotation(&state.gameboard),
                4 => state.piece.anticlock_rotation(&state.gameboard),
                _ => (),
            };
        }

        for _ in 0..100 {
            match rng.gen_range(0..=4) {
                0 => state.piece.drop(&state.gameboard),
                1 => state.piece.left_slide(&state.gameboard),
                2 => state.piece.right_slide(&state.gameboard),
                3 => state.piece.clock_rotation(&state.gameboard),
                4 => state.piece.anticlock_rotation(&state.gameboard),
                _ => (),
            };
            if state.piece.fix(&state.gameboard) {
                state.gameboard.merge(&state.piece).clear_rows();
                state.piece = Domino::new(&state.gameboard);
                break;
            }
        }
    }
    event::run(ctx, events_loop, state)
}
