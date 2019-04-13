mod board;
mod strategies;

use std::collections::HashMap;
use board::{ Board, CellPos, CellValue };
use strategies::STRATEGIES;

fn main() {
    let board = Board::load(include_str!("sudoku.txt"));
    println!("Solving:");
    println!("{}", board);
    let board = solve(board, &mut HashMap::new(), 0);
    if board.is_contradiction() {
        println!("No solutions exist.");
    } else if board.is_solved() {
        println!("{}", board);
        println!("Solved.");
    } else {
        println!("{}", board);
        println!("More than one solution exists. Above is minimal form.");
    }
}

fn solve(mut board: Board, seen: &mut HashMap<Board, bool>, recursion_level: usize) -> Board {
    'solveloop: for step in 1.. {
        if board.is_contradiction() || board.is_solved() {
            break;
        }
        let current = board.clone();
        for (strat, name) in STRATEGIES {
            board = strat(&current);
            if board != current {
                for _ in 0..recursion_level {
                    print!(" ")
                }
                println!("Step {}, applying {}.", step, name);
                continue 'solveloop;
            }
        }
        for _ in 0..recursion_level {
            print!(" ")
        }
        println!("Solving strategies could not fully solve board. Now guessing & checking.");
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
    board
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
            board = solve(board, seen, recursion_level + 1);
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