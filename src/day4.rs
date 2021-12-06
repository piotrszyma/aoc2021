use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;

type Row = Vec<i8>;

type DrawnNumbers = Vec<i8>;

#[derive(Debug)]
struct Board {
    rows: Vec<Row>,
}

impl Board {
    fn new(rows: Vec<Row>) -> Self {
        Board { rows: rows }
    }
}

#[derive(Debug)]
struct BoardGame {
    boards: Vec<Board>,
    numbers: DrawnNumbers,
}

fn read_boards(boards_lines: Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut boards_iter = boards_lines.iter();

    boards_iter.next(); // Skip first line - it is empty.

    'outer: loop {
        let mut rows: Vec<Row> = Vec::new();
        loop {
            let line = boards_iter.next();

            match line {
                Some(value) => {
                    if value == "" {
                        boards.push(Board::new(rows.to_owned()));
                        rows = Vec::new();
                    } else {
                        let line_nums: Vec<i8> = value
                            .split(' ')
                            .filter(|l| l != &"")
                            .map(|l| {
                                l.parse()
                                    .expect(&format!("Expected value to be int, bot got {}", &l))
                            })
                            .collect();
                        rows.push(line_nums)
                    }
                }
                _ => {
                    boards.push(Board::new(rows.to_owned()));
                    break 'outer;
                }
            }
        }
    }
    boards
}

fn read_nums(line: String) -> Vec<i8> {
    line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn read_nums_and_boards(reader: BufReader<&std::fs::File>) -> BoardGame {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let numbers = read_nums(lines.first().unwrap().to_owned());
    let boards = read_boards(lines[1..].to_owned());
    BoardGame {
        boards: boards,
        numbers: numbers,
    }
}

fn mark_num(boards: &mut Vec<Board>, num: i8) {
    for board in boards {
        board.rows = board
            .rows
            .iter()
            .map(|r| r.iter().map(|c| if c == &num { -1 } else { *c }).collect())
            .collect();
    }
}

fn is_board_winner(board: &Board) -> bool {
    for row in &board.rows {
        let mut is_winner = true;
        for cell in row {
            if cell != &-1 {
                is_winner = false;
            }
        }
        if is_winner {
            return true;
        }
    }

    let row_idx_range = Range { start: 0, end: 4 };
    for row_idx in row_idx_range {
        let mut is_winner = true;
        for row in board.rows.iter() {
            if row[row_idx] != -1 {
                is_winner = false;
            }
        }

        if is_winner {
            return true;
        }
    }
    false
}

fn winner_latest(boards: &Vec<Board>) -> Option<&Board> {
    // The only looser is the true winner.
    let loosers: Vec<_> = boards.iter().filter(|b| !is_board_winner(b)).collect();
    if loosers.len() == 1 {
        Some(*loosers.first().unwrap())
    } else {
        None
    }
}

fn calculate_result_extended(board: &Board, next_num: i8) -> i64 {
    let mut result: i64 = 0;
    for row in &board.rows {
        for cell in row {
            if cell != &-1 && cell != &next_num {
                result += i64::from(*cell);
            }
        }
    }
    result * i64::from(next_num)
}

fn find_winning_board_extended(boards: Vec<Board>, nums: DrawnNumbers) -> Option<i64> {
    let mut boards = boards;

    for window in nums.windows(2) {
        let num = &window[0];
        let next_num = &window[1];

        mark_num(&mut boards, *num);

        match winner_latest(&boards) {
            Some(board) => {
                return Some(calculate_result_extended(board, *next_num));
            }
            _ => (),
        }
    }
    panic!("Failed to find a winner")
}

fn play_game_extended(boards: Vec<Board>, nums: DrawnNumbers) -> i64 {
    find_winning_board_extended(boards, nums).expect("Failed to find winning board")
}

fn winner(boards: &Vec<Board>) -> Option<&Board> {
    for board in boards {
        for row in board.rows.iter() {
            let mut is_winner = true;
            for cell in row {
                if cell != &-1 {
                    is_winner = false;
                    break;
                }
            }
            if is_winner {
                return Some(board);
            }
        }
        // TODO: Check also columns.
    }
    None
}

fn calculate_result(board: &Board, num: i8) -> i64 {
    let mut result: i64 = 0;
    for row in &board.rows {
        for cell in row {
            if cell != &-1 {
                result += i64::from(*cell);
            }
        }
    }
    result * i64::from(num)
}

fn find_winning_board(mut boards: Vec<Board>, nums: DrawnNumbers) -> Option<i64> {
    for num in nums {
        mark_num(&mut boards, num);
        match winner(&boards) {
            Some(board) => {
                return Some(calculate_result(board, num));
            }
            _ => (),
        }
    }
    None
}

fn play_game(boards: Vec<Board>, nums: DrawnNumbers) -> i64 {
    find_winning_board(boards, nums).expect("Failed to find winning board")
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let board_game = read_nums_and_boards(BufReader::new(&file));

    let numbers = board_game.numbers;
    let boards = board_game.boards;
    play_game(boards, numbers)
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let board_game = read_nums_and_boards(BufReader::new(&file));

    let numbers = board_game.numbers;
    let boards = board_game.boards;

    play_game_extended(boards, numbers)
}

pub fn task1() -> i64 {
    task1_run("data/day4.txt")
}

pub fn task2() -> i64 {
    task2_run("data/day4.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_test_data() {
        assert_eq!(4512, task1_run("data/day4_test.txt"))
    }

    #[test]
    fn task1() {
        assert_eq!(39984, task1_run("data/day4.txt"))
    }

    #[test]
    fn task2_test_data() {
        assert_eq!(1924, task2_run("data/day4_test.txt"))
    }

    #[test]
    fn task2() {
        assert_eq!(8468, task2_run("data/day4.txt"))
    }
}



