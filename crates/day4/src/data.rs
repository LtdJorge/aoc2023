#[derive(Debug, Clone)]
pub(crate) struct Card {
    pub(crate) id: i32,
    pub(crate) winning: Vec<i32>,
    pub(crate) owned: Vec<i32>,
}

impl Card {
    pub(crate) fn calculate_points(&self) -> i32 {
        self.winning
            .iter()
            .filter_map(|winner_value| self.owned.iter().find(|&&val| val == *winner_value))
            .fold(0, |acc, val| if acc == 0 { 1 } else { acc * 2 })
    }
}
