# rstris

My second rust project. A Tetris _game_ where pieces are dominoes. I made it because I couldn't find a version online.

Its build to be extended with other pieces in a future.

## Usage

The default command 'rstris' loads a `20x10` grid. Dimensions can be specified with `-d=<rows>x<columns>` or with `--dimensions=<rows>x<columns>`. Also with the parameter `-p=<n>` the game starts with `n` pieces played randomly in the board.

To add arguments with cargo add `--`. For example:

```
cargo run -- -p=20
```

## Play

The following keys do something:

- `Space`: fixes the piece if it can be fixed. Otherwise, hard drops the piece.
- `Left Arrow`: left slide.
- `Right Arrow`: right slide.
- `Down Arrow`: drops the piece one row.
- `Up Arrow`: moves up the piece one row.
- `A`: anticlockwise rotation.
- `D`: clockwise rotation.

## Installation

Make sure to have `rust` and `cargo` installed. To play the game download the repo, move inside the folder and run `cargo run`.

```
git clone https://github.com/pololo300/rstris.git
cd rstris
cargo run
```
