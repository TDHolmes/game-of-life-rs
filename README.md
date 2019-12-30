# game-of-life-rs
Rust implementation of Conway's Game of Life. Supports both random life generation as well as seeding with standard RLE life files, or my own JSON format.

# Usage

```
game-of-life
An implementation of Conway's Game of Life.

USAGE:
    gameoflife [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <COLS>                                  Number of columns in the grid
    -f, --config-filepath <config-filepath>    Board configuration file. Supports custom JSON or standard RLE. See
                                               http://www.conwaylife.com/wiki/Run_Length_Encoded for more info.
    -p <rand-density>                          Probability that a spot is alive at the beginning - [0,1]
        --rate <rate>                          Speed of the refresh cycles in miliseconds
    -r <ROWS>                                  Number of rows in the grid
```

# Example Usage
```
./gameoflife -f ./simple_glider.rle -r 10 -c 40
Board size: rows: 10, cols: 40
┌────────────────────────────────────────┐
│ ●                                      │
│  ●                                     │
│●●●                                     │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
└────────────────────────────────────────┘
```

where `./simple_glider.rle` is:
```
#C This is a glider.
x = 3, y = 3, rule = B3/S23
bo$2bo$3o!
```

# More Info

  - [RLE File Format](https://www.conwaylife.com/wiki/Run_Length_Encoded)
  - [Conway's Game of Life in general](https://www.conwaylife.com/wiki/Main_Page)
