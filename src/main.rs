mod board;
mod strategies;

use std::collections::HashMap;
use board::{ Board, CellPos, CellValue };
use strategies::{ STRATEGIES, DIFFICULTIES };

fn main() {
    let board = Board::load(include_str!("sudoku.txt"));
    println!("Solving:");
    println!("{}", board);
    let (board, difficulty) = solve(board, &mut HashMap::new(), 0);
    if board.is_contradiction() {
        println!("No solutions exist.");
    } else if board.is_solved() {
        println!("{}", board);
        println!("Solved. I think this is {} puzzle.", DIFFICULTIES[difficulty]);
    } else {
        println!("{}", board);
        println!("More than one solution exists. Above is minimal form.");
    }
}

fn reduce(mut board: Board, indent: usize) -> (Board, usize) {
    let mut difficulty = 0;
    'solveloop: for step in 1.. {
        if board.is_contradiction() || board.is_solved() {
            break;
        }
        let current = board.clone();
        for (i, (strat, name)) in STRATEGIES.iter().enumerate() {
            board = strat(&current);
            if board != current {
                for _ in 0..indent {
                    print!(" ")
                }
                println!("Step {}, applying {}.", step, name);
                continue 'solveloop;
            }
            difficulty = difficulty.max(i);
        }
        for _ in 0..indent {
            print!(" ")
        }
        println!("Solving strategies could not fully solve board.");
        break;
    }
    (board, difficulty)
}

fn solve(mut board: Board, seen: &mut HashMap<Board, bool>, recursion_level: usize) -> (Board, usize) {
    let mut difficulty = 0;
    'solveloop: loop {
        let (b, diff) = reduce(board, recursion_level);
        board = b;
        difficulty = difficulty.max(diff);
        if board.is_contradiction() || board.is_solved() {
            break;
        }
        for _ in 0..recursion_level {
            print!(" ")
        }
        println!("Now guessing & checking.");
        difficulty = STRATEGIES.len();
        let mut choices = vec![];
        for row in 0..9 {
            for col in 0..9 {
                if board[(row, col)].possiblities().len() > 1 {
                    choices.push((row, col));
                }
            }
        }
        choices.sort_by_key(|&pos| board[pos].possiblities().len());
        for pos in choices {
            if let Some(v) = find_contradiction(&board, pos, seen, recursion_level) {
                for _ in 0..recursion_level {
                    print!(" ")
                }
                println!("Found contradiction. Continuing...");
                board[pos].remove(v);
                continue 'solveloop;
            }
        }
        break;
    }
    (board, difficulty)
}

fn find_contradiction(
    board: &Board, pos: CellPos, seen: &mut HashMap<Board, bool>, recursion_level: usize
) -> Option<CellValue> {
    for v in board[pos].possiblities() {
        let mut board = board.clone();
        board[pos].set(v);
        let contradiction = if seen.contains_key(&board) {
            for _ in 0..recursion_level {
                print!(" ")
            }
            println!("Seen this board before. Skipping.");
            seen[&board]
        } else {
            let b = board.clone();
            let (b_, _) = solve(board, seen, recursion_level + 1);
            board = b_;
            let contradiction = board.is_contradiction();
            seen.insert(b, contradiction);
            contradiction
        };
        if contradiction {
            return Some(v);
        }
        for _ in 0..recursion_level {
            print!(" ")
        }
        println!("No contradiction found on this path.");
    }
    return None
}