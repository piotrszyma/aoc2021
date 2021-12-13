use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static DATA_FILEPATH: &str = "data/day13.txt";

#[derive(Debug)]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    position: u32,
}

#[derive(Debug)]
struct Input {
    init_dots: Vec<Dot>,
    folds: Vec<Fold>,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CardCordState {
    Dot,
    NotDot,
}

#[derive(Debug)]
struct Card {
    cols: Vec<Vec<CardCordState>>,
}

impl Card {
    fn x_len(&self) -> usize {
        self.cols.len()
    }

    fn y_len(&self) -> usize {
        self.cols.first().expect("Card should have at least one col?").len()
    }
}

fn read_data(reader: BufReader<&File>) -> Input {
    let mut folds_phase = false;
    let mut init_dots: Vec<Dot> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "".to_string() {
            folds_phase = true;
        } else if folds_phase {
            let split = line
                .split(" ")
                .last()
                .expect("Expected first folds line split to work");
            let mut split = split.split("=");
            let axis = split.next().unwrap();
            let position: u32 = split.next().unwrap().parse().unwrap();

            folds.push(Fold {
                axis: match axis {
                    "x" => Axis::X,
                    "y" => Axis::Y,
                    _ => panic!("Invalid axis"),
                },
                position,
            })
        } else {
            let mut split = line.split(",");
            let x = split
                .next()
                .expect("Failed to extract first element from point split x,y");
            let x: usize = x
                .parse()
                .expect("Failed to parse first element from point split x,y into number");

            let y = split
                .next()
                .expect("Failed to extract second element from point split x,y");
            let y: usize = y
                .parse()
                .expect("Failed to parse second element from point split x,y into number");

            if x > max_x {
                max_x = x;
            }

            if y > max_y {
                max_y = y;
            }

            init_dots.push(Dot { x, y })
        }
    }
    Input { init_dots, folds, max_x, max_y }
}

fn create_card(max_x: usize, max_y: usize, init_dots: Vec<Dot>) -> Card {
    let mut cols: Vec<Vec<CardCordState>> = Vec::new();
    for _ in 0..max_x+1 {
        let mut col: Vec<CardCordState> = Vec::new();
        for _ in 0..max_y+1 {
            col.push(CardCordState::NotDot);
        };
        cols.push(col);
    };

    for init_dot in init_dots {
        let col = cols.get_mut(init_dot.x).expect("Invalid x in init_dots.");
        col[init_dot.y] = CardCordState::Dot;
    }

    Card {
        cols
    }
}

fn build_folded_x(card: Card, position: u32) -> Card {
    let first_chunk = &card.cols[..position as usize].to_vec(); // from 0 to pos - 1
    let second_chunk = &card.cols[position as usize + 1..].to_vec(); // from pos + 1, to end

    let chunks = if first_chunk.len() < second_chunk.len() {
        (second_chunk, first_chunk)
    } else {
        (first_chunk, second_chunk)
    };

    let mut bigger_chunk = chunks.0.to_vec();
    let smaller_chunk = chunks.1.to_vec();

    let mut bigger_idx_to_apply = bigger_chunk.len();
    for col in smaller_chunk {
        bigger_idx_to_apply -= 1;
        for (idx, cell) in col.iter().enumerate() {
            if *cell == CardCordState::Dot {
                bigger_chunk[bigger_idx_to_apply][idx] = CardCordState::Dot;
            }
        }
    }

    Card {
        cols: bigger_chunk.to_vec()
    }
}

fn build_folded_y(card: Card, position: u32) -> Card {
    let mut new_cols: Vec<Vec<CardCordState>> = Vec::new();
    for col in card.cols {
        let first_chunk = (&col[..position as usize]).to_vec(); // from 0 to pos - 1
        let second_chunk = (&col[position as usize + 1..]).to_vec(); // from pos + 1, to end

        let chunks = if first_chunk.len() < second_chunk.len() {
            (second_chunk, first_chunk)
        } else {
            (first_chunk, second_chunk)
        };

        let mut bigger_chunk = chunks.0;
        let smaller_chunk = chunks.1;

        let mut bigger_idx_to_apply = bigger_chunk.len();
        for cell in smaller_chunk.iter() {
            bigger_idx_to_apply -= 1;
            if *cell == CardCordState::Dot {
                bigger_chunk[bigger_idx_to_apply] = CardCordState::Dot;
            }
        };

        new_cols.push(bigger_chunk.to_vec());
    }

    Card {
        cols: new_cols
    }
}

fn apply_folds(card: Card, folds: &[Fold]) -> Card {
    if folds.len() == 0 {
        return card;
    }

    let mut final_card = card;

    for fold in folds {
        match fold.axis {
            Axis::X => {
                // panic!("Not implemented");
                final_card = build_folded_x(final_card, fold.position);
            },
            Axis::Y => {
                final_card = build_folded_y(final_card, fold.position);
            }
        };
    }

    // TODO: return new card, folded
    final_card
}

fn count_dots(card: Card) -> i64 {
    let mut count = 0;
    for col in card.cols {
        for cell in col {
            if cell == CardCordState::Dot {
                count += 1;
            }
        }
    };
    count
}

fn print_card(card: Card) {
    for y_idx in 0..card.y_len() {
        for x_idx in 0..card.x_len() {
            let value = &card.cols[x_idx][y_idx];
            if value == &CardCordState::Dot {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}

pub fn task1_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let input = read_data(BufReader::new(&file));
    let card = create_card(input.max_x, input.max_y, input.init_dots);
    let folds = input.folds;
    let card = apply_folds(card, &folds[..1]);
    count_dots(card)
}

pub fn task2_run(path: &str) -> i64 {
    let file = File::open(path).unwrap();
    let input = read_data(BufReader::new(&file));
    let card = create_card(input.max_x, input.max_y, input.init_dots);
    let folds = input.folds;
    let card = apply_folds(card, &folds[..]);
    print_card(card);
    0
}

pub fn task1() -> i64 {
    task1_run(DATA_FILEPATH)
}

pub fn task2() -> i64 {
    task2_run(DATA_FILEPATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA_FILEPATH: &str = "data/day13_test.txt";

    #[test]
    fn task1_test_data() {
        assert_eq!(17, task1_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task1() {
        assert_eq!(653, task1_run(DATA_FILEPATH));
    }

    #[test]
    fn task2_test_data() {
        // Solution will be printed to stdout.
        // It is "O"
        assert_eq!(0, task2_run(TEST_DATA_FILEPATH))
    }

    #[test]
    fn task2() {
        // Solution will be printed to stdout.
        // #....#..#.###..####.###..###..###..#..#.
        // #....#.#..#..#.#....#..#.#..#.#..#.#.#..
        // #....##...#..#.###..###..#..#.#..#.##...
        // #....#.#..###..#....#..#.###..###..#.#..
        // #....#.#..#.#..#....#..#.#....#.#..#.#..
        // ####.#..#.#..#.####.###..#....#..#.#..#.
        //
        // which translates into: lkrebprk
        assert_eq!(0, task2_run(DATA_FILEPATH))
    }
}
