use crate::divider::score_calculator::ScoreCalculator;
use crate::divider::lightgbm_text_model::LightGbmTextModel;
use crate::feature::extractor::FamilyRankingFeatureExtractor;
use std::sync::Arc;

pub struct GBDTScoreCalculator {
    pub feature_extractor: FamilyRankingFeatureExtractor,
    pub model: Arc<LightGbmTextModel>,
}

impl ScoreCalculator for GBDTScoreCalculator {
    fn calc_score(&self, family: &String, given: &String) -> f64 {
        let features = self.feature_extractor.get_features(family, given);
        self.model.predict(&features.to_vec())
    }
}
