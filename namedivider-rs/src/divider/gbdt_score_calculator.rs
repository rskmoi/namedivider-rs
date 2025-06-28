use crate::divider::score_calculator::ScoreCalculator;
use crate::feature::extractor::FamilyRankingFeatureExtractor;
use lightgbm::Booster;
use std::sync::Arc;
use std::cell::RefCell;

// Thread-local storage for single LightGBM model (maximum efficiency)
// Each thread gets exactly one Booster instance, created lazily on first use
thread_local! {
    static THREAD_MODEL: RefCell<Option<(u64, Booster)>> = RefCell::new(None);
}

pub struct GBDTScoreCalculator {
    pub feature_extractor: FamilyRankingFeatureExtractor,
    pub model_string: Arc<String>,
    pub model_hash: u64,  // Pre-computed hash for fast lookup
}

impl ScoreCalculator for GBDTScoreCalculator {
    fn calc_score(&self, family: &String, given: &String) -> f64 {
        let features = self.feature_extractor.get_features(family, given);
        
        // Maximum efficiency: single model per thread with zero-cost lookup
        THREAD_MODEL.with(|model_cell| {
            let mut model_ref = model_cell.borrow_mut();
            
            // Check if we need to initialize or update the thread-local model
            let needs_init = match model_ref.as_ref() {
                None => true,  // First access - need to create model
                Some((cached_hash, _)) => *cached_hash != self.model_hash,  // Different model
            };
            
            if needs_init {
                // Lazy initialization: create model only when needed
                let booster = Booster::from_string(&self.model_string)
                    .expect("Failed to create Booster from model string");
                *model_ref = Some((self.model_hash, booster));
            }
            
            // Use the thread-local model for prediction (guaranteed to exist)
            let (_, model) = model_ref.as_ref().unwrap();
            let score = model.predict(vec![features.to_vec()]);
            score.unwrap()[0][0]
        })
    }
}
