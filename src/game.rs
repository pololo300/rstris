pub mod board;
pub mod piece;

use crate::game::board::Board;
use crate::game::piece::domino::Domino;
use crate::game::piece::Piece;

use ggez::{
    // context,
    event,
    graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context,
    GameResult,
};

const GRID_SIZE: (i32, i32) = (10, 20);
const CELL_RADIUS: i32 = 2;
const GRID_CELL_SIZE: i32 = 32;
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE as f32,
);

struct GameState {
    gameboard: Board,
    piece: Domino,
}

impl GameState {
    pub fn new() -> GameState {
        let gameboard = Board::new(GRID_SIZE.0 as usize, GRID_SIZE.1 as usize);
        let piece = Domino::new(&gameboard);
        GameState { gameboard, piece }
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
            Some(KeyCode::A) => self.piece.clock_rotation(&self.gameboard),
            Some(KeyCode::D) => self.piece.anticlock_rotation(&self.gameboard),
            Some(KeyCode::Space) => {
                if self.piece.fix(&self.gameboard) {
                    self.gameboard.merge(&self.piece).clear_rows();
                    self.piece = Domino::new(&self.gameboard);
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
                    .dest_rect(big_square(cell.x(), cell.y()))
                    .color(darker),
            );

            // inside rectanngle
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(inside_square(cell.x(), cell.y()))
                    .color(color),
            );
        }

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(big_square(self.piece.cell1.x, self.piece.cell1.y))
                .color([0.0, 1.0, 1.0, 1.0]),
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(inside_square(self.piece.cell1.x, self.piece.cell1.y))
                .color([0.0, 0.8, 0.8, 1.0]),
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(big_square(self.piece.cell2.x, self.piece.cell2.y))
                .color([0.0, 1.0, 1.0, 1.0]),
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(inside_square(self.piece.cell2.x, self.piece.cell2.y))
                .color([0.0, 0.8, 0.8, 1.0]),
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn big_square(x: usize, y: usize) -> graphics::Rect {
    graphics::Rect::new_i32(
        GRID_CELL_SIZE * (x as i32),
        GRID_CELL_SIZE * (GRID_SIZE.1 - 1 - y as i32),
        GRID_CELL_SIZE,
        GRID_CELL_SIZE,
    )
}

fn inside_square(x: usize, y: usize) -> graphics::Rect {
    graphics::Rect::new_i32(
        GRID_CELL_SIZE * x as i32 + CELL_RADIUS,
        GRID_CELL_SIZE * (GRID_SIZE.1 - 1 - y as i32) + CELL_RADIUS,
        GRID_CELL_SIZE - 2 * CELL_RADIUS,
        GRID_CELL_SIZE - 2 * CELL_RADIUS,
    )
}

pub fn game() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("snake", "Gray Olson")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new();
    event::run(ctx, events_loop, state)
}
