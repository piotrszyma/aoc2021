use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    for row in board.rows.iter() {
        let mut is_winner = true;
        for cell in row {
            if cell != &-1 {
                is_winner = false;
            }
        }
        if is_winner {
            return true
        }
    }
    false
}

fn winner(boards: &Vec<Board>) -> Option<&Board> {
    let winners: Vec<_> = boards.iter().filter(|b| !is_board_winner(b)).collect();
    if winners.len() == 1 {
        winners.first().unwrap();
    }
    None
}

fn calculate_result(board: &Board, num: i8) -> i32 {
    let mut result: i32 = 0;
    for row in board.rows.iter() {
        for cell in row {
            if cell != &-1 {
                result += i32::from(*cell);
            }
        }
    }
    result * i32::from(num)
}

fn find_winning_board(boards: Vec<Board>, nums: DrawnNumbers) -> Option<i32> {
    let mut boards = boards;
    for num in nums {
        mark_num(&mut boards, num); 
        match winner(&boards) {
            Some(board) => {
                println!("{:?}", boards);
                return Some(calculate_result(board, num));
            }
            _ => (),
        }
    }
    None
}

fn play_game(boards: Vec<Board>, nums: DrawnNumbers) -> Option<i32> {
    find_winning_board(boards, nums)
}

pub fn run() {
    let file = File::open("data/day4_task1_test.txt").unwrap();
    let board_game = read_nums_and_boards(BufReader::new(&file));

    let numbers = board_game.numbers;
    let boards = board_game.boards;

    let result = play_game(boards, numbers);

    println!("result={:?}", result)
}
