use std::error::Error;
use std::io;
use std::num::ParseIntError;

#[derive(Clone, PartialOrd, PartialEq, Debug)]
enum Intersection {
    Empty,
    Player(usize),
}

type Board = Vec<Vec<Intersection>>;

const WINNING_LEN: usize = 5;

fn make_board(n: usize) -> Board {
    vec![vec![Intersection::Empty; n]; n]
}

fn board_to_string(board: &Board) -> String {
    let mut s = String::new();

    s.push_str("   ");
    for col in 0..board.len() {
        s.push_str(&format!("{:02} ", col.to_string()));
    }
    s.push('\n');

    for (i, row) in board.iter().enumerate() {
        s.push_str(&format!("{:02} ", i.to_string()));
        for intersection in row {
            match intersection {
                Intersection::Empty => { s.push(' ') }
                Intersection::Player(id) => { s.push_str(&format!("{}", id)) }
            }
            s.push_str("  ");
        }
        s.push('\n');
    }
    s
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

fn position_available(pos: &Position, board: &Board) -> bool {
    let &Position(row, col) = pos;
    board[row][col] == Intersection::Empty
}

fn check_horizontal(board: &Board, Position(row, col): Position) -> Option<usize> {
    if board[row][col] == Intersection::Empty {
        return None;
    }

    if board.len() - col >= WINNING_LEN {
        if let &Intersection::Player(prev_id) = &board[row][col] {
            for j in col + 1..col + WINNING_LEN + 1 {
                match &board[row][j] {
                    &Intersection::Player(id) if id == prev_id => {}
                    _ => return None,
                }
            }
            return Some(prev_id);
        }
    }
    None
}

fn game_get_winner(board: &Board) -> Option<usize> {
    for i in 0..board.len() {
        for j in 0..board.len() {
            // horizontal
            // vertical
            // diagonal up
            // diagonal down
        }
    }
    None
}

fn game_is_tie(board: &Board) -> bool {
    board.iter().all(|row| row.iter().all(|intersection| intersection != &Intersection::Empty))
}

fn main() {
    let num_players = 2;
    let mut curr_player = 0;
    let board_size = 15;
    let mut board = make_board(board_size);

    loop {
        println!("{}", board_to_string(&board));
        println!("Please input your position.");

        let mut input_position = String::new();

        io::stdin()
            .read_line(&mut input_position)
            .expect("Failed to read line");

        match parse_position(&input_position, board_size) {
            Ok(pos) => {
                if position_available(&pos, &board) {
                    let Position(row, col) = pos;
                    board[row][col] = Intersection::Player(curr_player);
                } else {
                    println!("Position taken!");
                    continue;
                }
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }

        curr_player = (curr_player + 1) % num_players;
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

    #[test]
    fn test_game_is_tie() {
        let board_size = 15;
        let mut board = make_board(board_size);

        for i in 0..board_size {
            for j in 0..board_size {
                assert!(!game_is_tie(&board));
                board[i][j] = Intersection::Player(0);
            }
        }
        assert!(game_is_tie(&board));
    }
}