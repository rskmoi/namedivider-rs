use crate::divider::score_calculator::ScoreCalculator;
use crate::feature::extractor::FamilyRankingFeatureExtractor;
use lightgbm::Booster;

pub struct GBDTScoreCalculator{
    pub feature_extractor:  FamilyRankingFeatureExtractor,
    pub model: Booster
}

impl ScoreCalculator for GBDTScoreCalculator{
    fn calc_score(&self, family: &String, given: &String) -> f64{
        let features = self.feature_extractor.get_features(family, given);
        let score = self.model.predict(vec![features.to_vec()]);
        score.unwrap()[0][0]
    }
}
