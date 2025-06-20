use crate::divider::score_calculator::ScoreCalculator;

pub struct TwoCharScoreCalculator {}
impl ScoreCalculator for TwoCharScoreCalculator {
    fn calc_score(&self, family: &String, _given: &String) -> f64 {
        return if family.chars().count() == 2 {
            1.0
        } else {
            0.0
        };
    }
}
