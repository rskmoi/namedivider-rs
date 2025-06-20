pub trait ScoreCalculator {
    fn calc_score(&self, family: &String, given: &String) -> f64;
}
