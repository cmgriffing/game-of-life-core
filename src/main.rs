mod core;

use crate::core::game::{
  deserialize_string_into_cellules, game_grids_are_identical, serialize_cellules_to_string,
  GameState, SerializedGameState,
};
use crate::core::history::History;
use crate::core::seeds::{get_seeds, seed_middle_line_starter};
use log::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
  /// The pattern to look for
  submission: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Submission {
  game_state: SerializedGameState,
  step_count: i32,
  active_count: i32,
  modifications: Vec<GridModification>,
  seed_label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SavedSubmission {
  _id: String,
  submission: Submission,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct GridModification {
  step_index: i32,
  grid_index: i32,
}

fn main() {
  let args = Cli::from_args();

  // println!("{:?}", args);

  let json: SavedSubmission = if args.submission != "" {
    serde_json::from_str(&args.submission).unwrap()
  } else {
    serde_json::from_str("{{}}").unwrap()
  };

  let starter_seed = get_seeds()
    .into_iter
    .find(|&seed| seed.label == json.submission.seed_label)
    .unwrap();

  // run the game
  let seeded_grid = starter_seed.cellules.clone();

  let mut game_state = GameState {
    cellules: seeded_grid,
    active: false,
    cellules_height: json.submission.game_state.cellules_height,
    cellules_width: json.submission.game_state.cellules_width,
  };

  // currently assumes step 0 modifications
  json
    .submission
    .modifications
    .iter()
    .for_each(|modification| game_state.toggle_cellule(modification.grid_index as usize));

  let mut history = History {
    previous_steps: vec![],
  };

  let mut steps = 0;

  while history.is_in_endless_loop(game_state.cellules.clone()) == false && steps <= 4000 {
    game_state.step();
    steps += 1;
  }

  info!("steps {}", steps);

  let submission_grid = deserialize_string_into_cellules(json.submission.game_state.cellules);

  // verify results
  let game_is_valid =
    game_grids_are_identical(submission_grid.clone(), game_state.cellules.clone());

  print!(
    "{}",
    json!({
      "valid": game_is_valid,
      "submission_id": json._id,
      // "serialized_submission": serialize_cellules_to_string(submission_grid.clone()),
      // "serialized_game_state": serialize_cellules_to_string(game_state.cellules.clone())
    })
  )
}
