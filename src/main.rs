mod board;
mod piece;
fn main() {
    let board = board::new_board(10, 20);
    board.print();
}
