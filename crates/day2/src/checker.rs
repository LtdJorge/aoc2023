use crate::parser::{Color, GameRun, GameSets, PlayedSet};

impl GameRun {
    pub fn verify_run(self, truth_set: &PlayedSet) -> Option<i32> {
        match self
            .sets
            .set_list
            .iter()
            .all(|played_set| verify_set(truth_set, played_set))
        {
            true => Some(self.id),
            false => None,
        }
    }
}

pub fn verify_set(truth_set: &PlayedSet, played_set: &PlayedSet) -> bool {
    truth_set.red >= played_set.red
        && truth_set.green >= played_set.green
        && truth_set.blue >= played_set.blue
}
