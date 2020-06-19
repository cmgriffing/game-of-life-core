use serde_derive::{Deserialize, Serialize};

use crate::core::game::{Cellule, LifeState};

use crate::core::seeds::raw::bim::seed_raw_bim_array;
use crate::core::seeds::raw::bits::seed_raw_bits_array;
use crate::core::seeds::raw::c::seed_raw_c_array;
use crate::core::seeds::raw::coles::seed_raw_coles_array;
use crate::core::seeds::raw::cube::seed_raw_cube_array;
use crate::core::seeds::raw::forty_two::seed_raw_forty_two_array;
use crate::core::seeds::raw::glider::seed_raw_glider_array;
use crate::core::seeds::raw::hmm::seed_raw_hmm_array;
use crate::core::seeds::raw::ligma::seed_raw_ligma_array;
use crate::core::seeds::raw::mrsir::seed_raw_mrsir_array;
use crate::core::seeds::raw::wall::seed_raw_wall_array;
use crate::core::seeds::raw::windmills::seed_raw_windmills_array;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Seed {
  pub label: String,
  pub cellules: Vec<Cellule>,
}

pub fn parse_raw_seed(raw_seed: Vec<i32>) -> Vec<Cellule> {
  raw_seed
    .into_iter()
    .map(|pixel_state_number| {
      if pixel_state_number == 1 {
        Cellule {
          life_state: LifeState::Alive,
        }
      } else {
        Cellule {
          life_state: LifeState::Dead,
        }
      }
    })
    .collect::<Vec<Cellule>>()
}

pub fn get_seeds() -> Vec<Seed> {
  vec![
    Seed {
      label: "Bim".to_owned(),
      cellules: parse_raw_seed(seed_raw_bim_array()),
    },
    Seed {
      label: "C".to_owned(),
      cellules: parse_raw_seed(seed_raw_c_array()),
    },
    Seed {
      label: "Coles".to_owned(),
      cellules: parse_raw_seed(seed_raw_coles_array()),
    },
    Seed {
      label: "Cube".to_owned(),
      cellules: parse_raw_seed(seed_raw_cube_array()),
    },
    Seed {
      label: "Diamond".to_owned(),
      cellules: parse_raw_seed(seed_raw_bits_array()),
    },
    Seed {
      label: "Glider".to_owned(),
      cellules: parse_raw_seed(seed_raw_glider_array()),
    },
    Seed {
      label: "hmm".to_owned(),
      cellules: parse_raw_seed(seed_raw_hmm_array()),
    },
    Seed {
      label: "Forty Two".to_owned(),
      cellules: parse_raw_seed(seed_raw_forty_two_array()),
    },
    Seed {
      label: "Line".to_owned(),
      cellules: seed_middle_line_starter(50, 40),
    },
    Seed {
      label: "Ligma".to_owned(),
      cellules: parse_raw_seed(seed_raw_ligma_array()),
    },
    Seed {
      label: "Mr Sir".to_owned(),
      cellules: parse_raw_seed(seed_raw_mrsir_array()),
    },
    Seed {
      label: "Pentadecathlon".to_owned(),
      cellules: seed_pentadecathlon(50, 40),
    },
    Seed {
      label: "Wall".to_owned(),
      cellules: parse_raw_seed(seed_raw_wall_array()),
    },
    Seed {
      label: "Windmills".to_owned(),
      cellules: parse_raw_seed(seed_raw_windmills_array()),
    },
  ]
}

pub fn seed_pentadecathlon(cellule_width: i32, cellule_height: i32) -> Vec<Cellule> {
  let middle_row_number: i32 = cellule_height / 2;

  let middle_column_number: i32 = cellule_width / 2;

  let mut cellules: Vec<Cellule> = Vec::new();

  for row_index in 0..cellule_height {
    for column_index in 0..cellule_width {
      if (middle_row_number <= row_index + 1 && middle_row_number >= row_index - 1)
        && (column_index < middle_column_number + 4 && column_index > middle_column_number - 5)
      {
        if middle_row_number == row_index
          && (middle_column_number == column_index + 3 || middle_column_number == column_index - 2)
        {
          cellules.push(Cellule {
            life_state: LifeState::Dead,
          })
        } else {
          cellules.push(Cellule {
            life_state: LifeState::Alive,
          })
        }
      } else {
        cellules.push(Cellule {
          life_state: LifeState::Dead,
        })
      }
    }
  }

  return cellules;
}

pub fn seed_middle_line_starter(cellule_width: i32, cellule_height: i32) -> Vec<Cellule> {
  let middle_row_number: i32 = cellule_height / 2;

  let middle_column_number: i32 = cellule_width / 2;

  let mut cellules: Vec<Cellule> = Vec::new();

  for row_index in 0..cellule_height {
    for column_index in 0..cellule_width {
      if (middle_row_number == row_index)
        && (column_index < middle_column_number + 7 && column_index > middle_column_number - 7)
      {
        cellules.push(Cellule {
          life_state: LifeState::Alive,
        })
      } else {
        cellules.push(Cellule {
          life_state: LifeState::Dead,
        })
      }
    }
  }

  return cellules;
}
