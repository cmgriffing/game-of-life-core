use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum LifeState {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cellule {
    pub life_state: LifeState,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SeedOption {
    pub label: String,
    pub cellules: Vec<Cellule>,
}

impl std::fmt::Display for SeedOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub active: bool,
    pub cellules: Vec<Cellule>,
    pub cellules_width: usize,
    pub cellules_height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedGameState {
    pub cellules: String,
    pub active: bool,
    pub cellules_width: usize,
    pub cellules_height: usize,
}

impl Cellule {
    pub fn set_alive(&mut self) {
        self.life_state = LifeState::Alive;
    }

    pub fn set_dead(&mut self) {
        self.life_state = LifeState::Dead;
    }

    pub fn alive(self) -> bool {
        self.life_state == LifeState::Alive
    }

    pub fn count_alive_neighbors(neighbors: &[Cellule]) -> usize {
        neighbors.iter().filter(|n| n.alive()).count()
    }

    pub fn alone(neighbors: &[Cellule]) -> bool {
        Self::count_alive_neighbors(neighbors) < 2
    }

    pub fn overpopulated(neighbors: &[Cellule]) -> bool {
        Self::count_alive_neighbors(neighbors) > 3
    }

    pub fn can_be_revived(neighbors: &[Cellule]) -> bool {
        Self::count_alive_neighbors(neighbors) == 3
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

impl GameState {
    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    pub fn set_cellules(&mut self, cellules: Vec<Cellule>) {
        self.cellules = cellules;
    }

    pub fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    pub fn toggle_cellule(&mut self, idx: usize) {
        let cellule = self.cellules.get_mut(idx).unwrap();
        if cellule.life_state == LifeState::Alive {
            cellule.life_state = LifeState::Dead
        } else {
            cellule.life_state = LifeState::Alive
        };
    }
}

pub fn game_grids_are_identical(grid_a: Vec<Cellule>, grid_b: Vec<Cellule>) -> bool {
    let length_a = grid_a.len();
    let length_b = grid_b.len();

    // info!("grid_a {:?} {:?}", grid_a.len(), grid_a[1].alive());
    // info!("grid_b {:?} {:?}", grid_b.len(), grid_b[1].alive());

    if length_a != length_b {
        return false;
    }

    for index in 0..(length_a - 1) {
        if grid_a[index].life_state != grid_b[index].life_state {
            return false;
        }
    }

    true
}

pub fn serialize_cellules_to_string(cellules: Vec<Cellule>) -> String {
    cellules
        .iter()
        .map(|cellule| {
            if cellule.life_state == LifeState::Dead {
                "0".to_owned()
            } else {
                "1".to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn deserialize_string_into_cellules(cellules_string: String) -> Vec<Cellule> {
    cellules_string
        .chars()
        .map(|char| {
            if char.to_string() == "1".to_owned() {
                Cellule {
                    life_state: LifeState::Alive,
                }
            } else {
                Cellule {
                    life_state: LifeState::Dead,
                }
            }
        })
        .collect()
}
