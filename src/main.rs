use std::error::Error;
use std::io;
use std::num::ParseIntError;

#[derive(Clone, PartialOrd, PartialEq, Debug)]
enum Intersection {
    Empty,
    Player(usize),
}

type Board = Vec<Vec<Intersection>>;

fn make_board(n: usize) -> Board {
    vec![vec![Intersection::Empty; n]; n]
}

#[derive(PartialOrd, PartialEq, Debug)]
struct Position(usize, usize); // (row, col)

fn parse_position_part(s: &str, board_size: usize) -> Result<usize, &str> {
    let pos = s.trim().parse::<usize>().map_err(|e| "Not an integer")?;
    if pos >= board_size {
        Err("Too large")
    } else {
        Ok(pos)
    }
}

fn parse_position(s: &str, board_size: usize) -> Result<Position, &str> {
    let parts = s.trim().split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 2 {
        Err("Not 2 parts")
    } else {
        Ok(Position(parse_position_part(parts[0], board_size)?, parse_position_part(parts[1], board_size)?))
    }
}

fn main() {
    let num_players = 2;

    loop {
        println!("Please input your guess.");

        let mut input_position = String::new();

        io::stdin()
            .read_line(&mut input_position)
            .expect("Failed to read line");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let n = 15;
        let board = make_board(n);
        assert_eq!(n, board.len());
        for row in board {
            assert_eq!(n, row.len());
            for intersection in row {
                assert_eq!(Intersection::Empty, intersection);
            }
        }
    }

    #[test]
    fn test_parse_position() {
        let board_size = 15;
        let s = "12 14";
        assert_eq!(Position(12, 14), parse_position(s, board_size).unwrap());

        let s = "0 1";
        assert_eq!(Position(0, 1), parse_position(s, board_size).unwrap());

        let s = " 12  14  ";
        assert_eq!(Position(12, 14), parse_position(s, board_size).unwrap());

        let bad_strings = ["12 15", "-1 2", "12.0 15", "12 1 1", "he he", "1, 2", "", "\n"];

        for s in bad_strings {
            assert!(parse_position(s, board_size).is_err());
        }
    }
}