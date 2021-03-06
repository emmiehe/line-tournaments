use std::io;

type PlayerId = usize;

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
enum Intersection {
    Empty,
    Player(PlayerId),
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
                Intersection::Empty => s.push('.'),
                Intersection::Player(id) => s.push_str(&format!("{}", id)),
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
    let pos = s.trim().parse::<usize>().map_err(|_| "Not an integer")?;
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

fn position_available(&Position(row, col): &Position, board: &Board) -> bool {
    board[row][col] == Intersection::Empty
}

fn check_winning(intersections: &[Intersection; WINNING_LEN]) -> Option<PlayerId> {
    let prev_id = match intersections[0] {
        Intersection::Player(prev_id) => prev_id,
        _ => return None,
    };

    for intersection in intersections.iter().take(WINNING_LEN).skip(1) {
        match intersection {
            &Intersection::Player(id) if id == prev_id => (),
            _ => return None,
        }
    }

    Some(prev_id)
}

fn get_positions_horizontal(board: &Board, &Position(row, col): &Position) -> [Intersection; WINNING_LEN] {
    let mut intersections = [Intersection::Empty; WINNING_LEN];
    intersections[..(board.len().min(col + WINNING_LEN) - col)].copy_from_slice(&board[row][col..board.len().min(col + WINNING_LEN)]);
    // // above code suggested by Clippy; below is my original code
    // for i in col..board.len().min(col + WINNING_LEN) {
    //     intersections[i - col] = board[row][i];
    // }
    intersections
}

fn get_positions_vertical(board: &Board, &Position(row, col): &Position) -> [Intersection; WINNING_LEN] {
    let mut intersections = [Intersection::Empty; WINNING_LEN];
    for i in row..board.len().min(row + WINNING_LEN) {
        intersections[i - row] = board[i][col];
    }
    intersections
}

fn get_positions_diagonal_up(board: &Board, &Position(row, col): &Position) -> [Intersection; WINNING_LEN] {
    let mut intersections = [Intersection::Empty; WINNING_LEN];
    let mut offset = 0;
    while offset < WINNING_LEN && row - offset > 0 && col + offset < board.len() {
        intersections[offset] = board[row - offset][col + offset];
        offset += 1;
    }
    intersections
}

fn get_positions_diagonal_down(board: &Board, &Position(row, col): &Position) -> [Intersection; WINNING_LEN] {
    let mut intersections = [Intersection::Empty; WINNING_LEN];
    let mut offset = 0;
    while offset < WINNING_LEN && row + offset < board.len() && col + offset < board.len() {
        intersections[offset] = board[row + offset][col + offset];
        offset += 1;
    }
    intersections
}

fn game_get_winner(board: &Board) -> Option<usize> {
    for i in 0..board.len() {
        for j in 0..board.len() {
            // horizontal
            if let Some(player_id) = check_winning(&get_positions_horizontal(board, &Position(i, j))) {
                return Some(player_id);
            }
            // vertical
            if let Some(player_id) = check_winning(&get_positions_vertical(board, &Position(i, j))) {
                return Some(player_id);
            }
            // diagonal up
            if let Some(player_id) = check_winning(&get_positions_diagonal_up(board, &Position(i, j))) {
                return Some(player_id);
            }
            // diagonal down
            if let Some(player_id) = check_winning(&get_positions_diagonal_down(board, &Position(i, j))) {
                return Some(player_id);
            }
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
    let board_size = 10;
    let mut board = make_board(board_size);

    println!("{}", board_to_string(&board));

    loop {
        println!("Please input your position.");
        let mut input_position = String::new();

        io::stdin()
            .read_line(&mut input_position)
            .expect("Failed to read line");

        match parse_position(&input_position, board_size) {
            Ok(Position(row, col)) => {
                if position_available(&Position(row, col), &board) {
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

        println!("{}", board_to_string(&board));

        if let Some(player_id) = game_get_winner(&board) {
            println!("Player {} won the game!", player_id);
            break;
        }

        if game_is_tie(&board) {
            println!("Game is tie!");
            break;
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

    #[test]
    fn test_check_winning() {
        let mut intersections = [Intersection::Empty; WINNING_LEN];
        for i in 0..WINNING_LEN {
            intersections[i] = Intersection::Player(0);
        }

        assert_eq!(Some(0), check_winning(&intersections));
        intersections[2] = Intersection::Empty;
        assert_eq!(None, check_winning(&intersections));
        intersections[2] = Intersection::Player(1);
        assert_eq!(None, check_winning(&intersections));
    }

    #[test]
    fn test_get_positions() {
        // todo: waiting for some pull request on this part :)
    }

    #[test]
    fn test_game_get_winner() {
        let board_size = 6;
        let mut board = make_board(board_size);

        // horizontal win
        for i in 1..WINNING_LEN {
            board[2][i] = Intersection::Player(0);
        }

        assert_eq!(None, game_get_winner(&board));
        board[2][0] = Intersection::Player(0);
        assert_eq!(Some(0), game_get_winner(&board));
        board[2][0] = Intersection::Player(1);
        assert_eq!(None, game_get_winner(&board));
    }
}