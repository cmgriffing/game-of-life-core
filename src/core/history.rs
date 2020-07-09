use crate::core::game::{game_grids_are_identical, Cellule};
#[allow(dead_code)]
use log::*;

const MAXIMUM_LOOP_TURN_COUNT: usize = 1600;

pub struct History {
  pub previous_steps: Vec<Vec<Cellule>>,
}

impl History {
  pub fn is_in_endless_loop(&mut self, current_grid: Vec<Cellule>) -> bool {
    let mut in_endless_loop = false;
    for previous_step in self.previous_steps.iter() {
      if game_grids_are_identical(previous_step.to_vec(), current_grid.clone()) {
        info!("IN A LOOP: Game grids are identical.");
        in_endless_loop = true;
      }
    }

    let previous_steps_count = self.previous_steps.len();
    if previous_steps_count >= MAXIMUM_LOOP_TURN_COUNT {
      let extra_count = previous_steps_count - MAXIMUM_LOOP_TURN_COUNT;
      self.previous_steps.drain(0..extra_count);
    }
    self.previous_steps.push(current_grid);

    in_endless_loop
  }

  pub fn clear_previous_steps(&mut self) {
    self.previous_steps = vec![];
  }
}
