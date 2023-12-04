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

    pub fn get_run_minimum_needed(self) -> anyhow::Result<PlayedSet> {
        let red = self
            .sets
            .set_list
            .iter()
            .map(|played_set| played_set.red)
            .max()
            .ok_or(anyhow::Error::msg("Error getting max from red"))?;
        let green = self
            .sets
            .set_list
            .iter()
            .map(|played_set| played_set.green)
            .max()
            .ok_or(anyhow::Error::msg("Error getting max from green"))?;
        let blue = self
            .sets
            .set_list
            .iter()
            .map(|played_set| played_set.blue)
            .max()
            .ok_or(anyhow::Error::msg("Error getting max from blue"))?;
        Ok(PlayedSet { red, green, blue })
    }
}

impl PlayedSet {
    pub fn calculate_power(self) -> i32 {
        self.red * self.green * self.blue
    }
}

pub fn verify_set(truth_set: &PlayedSet, played_set: &PlayedSet) -> bool {
    truth_set.red >= played_set.red
        && truth_set.green >= played_set.green
        && truth_set.blue >= played_set.blue
}
