mod game;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rows: u32 = 20;
    let mut columns: u32 = 10;
    let mut pieces: u32 = 0;

    // Loop through the arguments to find -d=mxn or --dimensions=mxn
    for arg in args.iter() {
        if arg.starts_with("-d=") || arg.starts_with("--dimensions=") {
            let dimensions = if let Some(dim_str) = arg.split('=').nth(1) {
                dim_str
            } else {
                continue;
            };

            if let Some((m_str, n_str)) = dimensions.split_once('x') {
                rows = m_str.parse::<u32>().expect("Invalid rows value");
                columns = n_str.parse::<u32>().expect("Invalid columns value");
            } else {
                eprintln!("Error: Invalid dimensions format. Expected format is -d=<rows>x<columns> or --dimensions=<rows>x<columns>.");
                return;
            }
        } else if arg == "-p" || arg == "--pieces" {
            pieces = 30;
        } else if arg.starts_with("-p=") || arg.starts_with("--pieces=") {
            // Parse the value for pieces
            if let Some(pieces_str) = arg.split('=').nth(1) {
                pieces = pieces_str.parse::<u32>().expect("Invalid pieces value");
            } else {
                eprintln!("Error: Invalid pieces format. Expected format is -p=n or --pieces=n.");
                return;
            }
        }
    }

    let _ = game::game(rows, columns, pieces);
}
