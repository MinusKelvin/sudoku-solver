use enumset::{ EnumSet, EnumSetType };
use std::fmt::{ Display, self };
use std::ops::{ Index, IndexMut };

#[derive(Clone, Eq)]
pub struct Board {
    groups: Vec<Group>,
    board: [[Cell; 9]; 9]
}

pub type CellPos = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group {
    cells: [CellPos; 9],
    pub group_type: GroupType
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GroupType {
    OneToNine
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cell(EnumSet<CellValue>);

#[derive(Debug, EnumSetType)]
pub enum CellValue {
    One = 1, /* Workaround bug in enumset_derive */
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Board {
    pub fn create_blank() -> Self {
        let mut groups = vec![];
        for i in 0..9 {
            groups.push(Group {
                cells: [(i, 0), (i, 1), (i, 2), (i, 3), (i, 4), (i, 5), (i, 6), (i, 7), (i, 8)],
                group_type: GroupType::OneToNine
            });
            groups.push(Group {
                cells: [(0, i), (1, i), (2, i), (3, i), (4, i), (5, i), (6, i), (7, i), (8, i)],
                group_type: GroupType::OneToNine
            });
        }
        for i in 0..3 {
            for j in 0..3 {
                groups.push(Group {
                    cells: [
                        (i*3, j*3),   (i*3+1, j*3),   (i*3+2, j*3),
                        (i*3, j*3+1), (i*3+1, j*3+1), (i*3+2, j*3+1),
                        (i*3, j*3+2), (i*3+1, j*3+2), (i*3+2, j*3+2),
                    ],
                    group_type: GroupType::OneToNine
                });
            }
        }
        Board {
            groups, board: [[Cell::new(); 9]; 9]
        }
    }

    pub fn load(board: &str) -> Self {
        let mut this = Board::create_blank();
        let mut row = 0;
        let mut col = 0;
        for c in board.chars() {
            match c {
                '_' => {}
                '1' => this.board[row][col].set(CellValue::One),
                '2' => this.board[row][col].set(CellValue::Two),
                '3' => this.board[row][col].set(CellValue::Three),
                '4' => this.board[row][col].set(CellValue::Four),
                '5' => this.board[row][col].set(CellValue::Five),
                '6' => this.board[row][col].set(CellValue::Six),
                '7' => this.board[row][col].set(CellValue::Seven),
                '8' => this.board[row][col].set(CellValue::Eight),
                '9' => this.board[row][col].set(CellValue::Nine),
                _ => continue
            };
            if col == 8 {
                row += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        this
    }

    pub fn groups(&self) -> &[Group] {
        &self.groups
    }

    pub fn is_solved(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col].known().is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_contradiction(&self) -> bool {
        for &group in self.groups() {
            match group.group_type {
                GroupType::OneToNine => {
                    let mut known = EnumSet::empty();
                    for &pos in group.cells() {
                        if let Some(v) = self[pos].known() {
                            if known.contains(v) {
                                return true;
                            }
                            known |= v;
                        }
                    }
                }
            }
        }
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col].is_contradiction() {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Index<CellPos> for Board {
    type Output = Cell;
    fn index(&self, index: CellPos) -> &Cell {
        &self.board[index.0][index.1]
    }
}

impl IndexMut<CellPos> for Board {
    fn index_mut(&mut self, index: CellPos) -> &mut Cell {
        &mut self.board[index.0][index.1]
    }
}

use std::hash::{ Hash, Hasher };
impl Hash for Board {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.board.hash(hasher);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Group {
    pub fn cells(&self) -> &[CellPos] {
        &self.cells
    }
}

impl Cell {
    pub fn new() -> Self {
        Cell(EnumSet::all())
    }

    pub fn is_contradiction(&self) -> bool {
        self.0.is_empty()
    }

    pub fn known(&self) -> Option<CellValue> {
        if self.0.len() == 1 {
            self.0.iter().last()
        } else {
            None
        }
    }

    pub fn set(&mut self, v: CellValue) {
        self.0 = EnumSet::only(v);
    }

    pub fn remove(&mut self, v: CellValue) {
        self.0 -= v;
    }

    pub fn remove_all(&mut self, values: EnumSet<CellValue>) {
        self.0 -= values;
    }

    pub fn keep(&mut self, values: EnumSet<CellValue>) {
        self.0 &= values;
    }

    pub fn contains(&self, v: CellValue) -> bool {
        self.0.contains(v)
    }

    pub fn contains_any(&self, values: EnumSet<CellValue>) -> bool {
        !self.0.is_disjoint(values)
    }

    pub fn possiblities(&self) -> EnumSet<CellValue> {
        self.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            for subrow in 0..3 {
                for col in 0..9 {
                    if let Some(v) = self.board[row][col].known() {
                        write!(f, "{}", match (v, subrow) {
                            (CellValue::One, 0) => r"/| ",
                            (CellValue::One, 1) => r" | ",
                            (CellValue::One, 2) => r"_|_",

                            (CellValue::Two, 0) => r"/^\",
                            (CellValue::Two, 1) => r" / ",
                            (CellValue::Two, 2) => r"/__",
                            
                            (CellValue::Three, 0) => r"^^\",
                            (CellValue::Three, 1) => r"--|",
                            (CellValue::Three, 2) => r"__/",
                            
                            (CellValue::Four, 0) => r"| |",
                            (CellValue::Four, 1) => r"|_|",
                            (CellValue::Four, 2) => r"  |",
                            
                            (CellValue::Five, 0) => r"|^^",
                            (CellValue::Five, 1) => r"L-.",
                            (CellValue::Five, 2) => r"__/",
                            
                            (CellValue::Six, 0) => r"/^^",
                            (CellValue::Six, 1) => r"|-.",
                            (CellValue::Six, 2) => r"\_/",
                            
                            (CellValue::Seven, 0) => r"^^|",
                            (CellValue::Seven, 1) => r"  /",
                            (CellValue::Seven, 2) => r" / ",
                            
                            (CellValue::Eight, 0) => r"/^\",
                            (CellValue::Eight, 1) => r">-<",
                            (CellValue::Eight, 2) => r"\_/",
                            
                            (CellValue::Nine, 0) => r"/^\",
                            (CellValue::Nine, 1) => r"\_|",
                            (CellValue::Nine, 2) => r"__/",

                            _ => unreachable!()
                        })?;
                    } else if self.board[row][col].0 == EnumSet::all() {
                        write!(f, "   ")?;
                    } else {
                        for subcol in 0..3 {
                            let (v, c) = match (subrow, subcol) {
                                (0, 0) => (CellValue::One, '1'),
                                (0, 1) => (CellValue::Two, '2'),
                                (0, 2) => (CellValue::Three, '3'),
                                (1, 0) => (CellValue::Four, '4'),
                                (1, 1) => (CellValue::Five, '5'),
                                (1, 2) => (CellValue::Six, '6'),
                                (2, 0) => (CellValue::Seven, '7'),
                                (2, 1) => (CellValue::Eight, '8'),
                                (2, 2) => (CellValue::Nine, '9'),
                                _ => unreachable!()
                            };
                            if self.board[row][col].0.contains(v) {
                                write!(f, "{}", c)?;
                            } else {
                                write!(f, " ")?;
                            }
                        }
                    }
                    write!(f, " ")?;
                    if col == 2 || col == 5 {
                        write!(f, "| ")?;
                    }
                }
                if row == 8 && subrow == 2 {
                    return Ok(());
                }
                writeln!(f)?;
            }
            for _ in 0..2 {
                write!(f, "            | ")?;
            }
            writeln!(f)?;
            if row == 2 || row == 5 {
                for _ in 0..2 {
                    write!(f, "------------+-")?;
                }
                writeln!(f, "-----------")?;
                for _ in 0..2 {
                    write!(f, "            | ")?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}