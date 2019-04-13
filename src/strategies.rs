use enumset::EnumSet;
use crate::board::{ Board, GroupType, CellValue };

pub const STRATEGIES: &[(fn(&Board) -> Board, &'static str)] = &[
    (trivial_eliminate, "Trivial Eliminations"),
    (peephole, "Peephole"),
    (group_intersection, "Group Intersection"),
    (direct_tuples, "Direct Tuples"),
    (indirect_pairs, "Indirect Pairs"),
    (indirect_triples, "Indirect Triples")
];

fn trivial_eliminate(board: &Board) -> Board {
    let mut into = board.clone();
    for &group in board.groups() {
        if group.group_type != GroupType::OneToNine {
            continue;
        }
        let mut knowns = EnumSet::empty();
        for &pos in group.cells() {
            if let Some(v) = board[pos].known() {
                knowns |= v;
            }
        }
        for &pos in group.cells() {
            if board[pos].known().is_none() {
                into[pos].remove_all(knowns);
            }
        }
    }
    into
}

fn peephole(board: &Board) -> Board {
    let mut into = board.clone();
    for &group in board.groups() {
        match group.group_type {
            GroupType::OneToNine => {
                'v: for v in EnumSet::all().iter() {
                    let mut found_one = false;
                    let mut cell = None;
                    for &pos in group.cells() {
                        if board[pos].contains(v) {
                            if found_one {
                                continue 'v;
                            }
                            cell = Some(pos);
                            found_one = true;
                        }
                    }
                    if let Some(pos) = cell {
                        into[pos].set(v);
                    }
                }
            }
        }
    }
    into
}

fn group_intersection(board: &Board) -> Board {
    let mut into = board.clone();
    for &group in board.groups() {
        if group.group_type != GroupType::OneToNine {
            continue;
        }
        for v in EnumSet::all().iter() {
            let mut positions = vec![];
            for &pos in group.cells() {
                if board[pos].contains(v) {
                    positions.push(pos);
                }
            }
            let positions = positions;
            'g2: for &group2 in board.groups() {
                for pos in &positions {
                    if !group2.cells().contains(pos) {
                        continue 'g2;
                    }
                }
                for &pos in group2.cells() {
                    if !positions.contains(&pos) {
                        into[pos].remove(v);
                    }
                }
            }
        }
    }
    into
}

fn direct_tuples(board: &Board) -> Board {
    let mut into = board.clone();
    for &group in board.groups() {
        if group.group_type != GroupType::OneToNine {
            continue;
        }
        for (i, &pos) in group.cells().iter().enumerate() {
            let set = board[(pos)].possiblities();
            let mut subgroup = vec![pos];
            for &pos in group.cells().iter().skip(i+1) {
                if board[pos].possiblities() == set {
                    subgroup.push(pos);
                }
            }
            if subgroup.len() == set.len() {
                for &pos in group.cells() {
                    if !subgroup.contains(&pos) {
                        into[pos].remove_all(set);
                    }
                }
            }
        }
    }
    into
}

fn indirect_pairs(board: &Board) -> Board {
    let mut into = board.clone();
    for &group in board.groups() {
        if group.group_type != GroupType::OneToNine {
            continue;
        }
        for (i, v1) in EnumSet::<CellValue>::all().iter().enumerate() {
            for v2 in EnumSet::<CellValue>::all().iter().skip(i+1) {
                let pair = v1 | v2;
                let mut positions = vec![];
                for &pos in group.cells() {
                    if board[pos].contains_any(pair) {
                        positions.push(pos);
                    }
                }
                if positions.len() == 2 {
                    for pos in positions {
                        into[pos].keep(pair);
                    }
                }
            }
        }
    }
    into
}

fn indirect_triples(board: &Board) -> Board {
    let mut into = board.clone();
    for &group in board.groups() {
        if group.group_type != GroupType::OneToNine {
            continue;
        }
        for (i, v1) in EnumSet::<CellValue>::all().iter().enumerate() {
            for (j, v2) in EnumSet::<CellValue>::all().iter().enumerate().skip(i+1) {
                for v3 in EnumSet::<CellValue>::all().iter().skip(j+1) {
                    let triple = v1 | v2 | v3;
                    let mut positions = vec![];
                    for &pos in group.cells() {
                        if board[pos].contains_any(triple) {
                            positions.push(pos);
                        }
                    }
                    if positions.len() == 3 {
                        for pos in positions {
                            into[pos].keep(triple);
                        }
                    }
                }
            }
        }
    }
    into
}