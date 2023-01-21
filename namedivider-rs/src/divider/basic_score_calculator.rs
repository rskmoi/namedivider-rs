use crate::divider::score_calculator::ScoreCalculator;
use crate::feature::extractor::SimpleFeatureExtractor;


pub struct BasicScoreCalculator{
    pub feature_extractor:  SimpleFeatureExtractor,
    pub only_order_score_when_4: bool
}

impl ScoreCalculator for BasicScoreCalculator{
    fn calc_score(&self, family: &String, given: &String) -> f64{
        let fullname_length  = family.chars().count() + given.chars().count();
        let features = self.feature_extractor.get_features(family, given);
        let order_score = (features.family_order_score + features.given_order_score) / ((fullname_length - 2) as f64);
        if self.only_order_score_when_4 && fullname_length == 4{
            return order_score;
        }
        let length_score = (features.family_length_score + features.given_length_score) / (fullname_length as f64);
        (order_score + length_score) / 2.0
    }
}