use crate::feature::kanji::KanjiStatisticsRepository;
use crate::feature::family_name::FamilyNameRepository;
use crate::feature::functional as F;

pub struct SimpleFeatures{
    pub family_order_score: f64,
    pub family_length_score: f64,
    pub given_order_score: f64,
    pub given_length_score: f64
}

pub struct FamilyRankingFeatures{
    pub rank: f64,
    pub fullname_length: f64,
    pub family_length: f64,
    pub given_length: f64,
    pub family_order_score: f64,
    pub given_order_score: f64,
    pub family_length_score: f64,
    pub given_length_score: f64,
    pub given_startswith_specific_kanji: f64
}

impl FamilyRankingFeatures{
    pub fn to_vec(&self) -> Vec<f64>{
        vec![self.rank,
                    self.fullname_length,
                    self.family_length,
                    self.given_length,
                    self.family_order_score,
                    self.given_order_score,
                    self.family_length_score,
                    self.given_length_score,
                    self.given_startswith_specific_kanji
        ]
    }
}

pub struct SimpleFeatureExtractor{
    pub kanji_statistics_repository: KanjiStatisticsRepository
}

impl SimpleFeatureExtractor{
    pub fn get_features(&self, family: &String, given: &String) -> SimpleFeatures{
        let fullname_length = family.chars().count() + given.chars().count();
        let family_order_score = F::calc_order_score(
            &self.kanji_statistics_repository,
            family,
            fullname_length,
            0
        );
        let family_length_score = F::calc_length_score(
            &self.kanji_statistics_repository,
            family,
            fullname_length,
            0
        );
        let given_order_score = F::calc_order_score(
            &self.kanji_statistics_repository,
            given,
            fullname_length,
            family.chars().count()
        );
        let given_length_score = F::calc_length_score(
            &self.kanji_statistics_repository,
            given,
            fullname_length,
            family.chars().count()
        );
        SimpleFeatures{ family_order_score, family_length_score, given_order_score, given_length_score }
    }
}

pub struct FamilyRankingFeatureExtractor{
    pub kanji_statistics_repository: KanjiStatisticsRepository,
    pub family_name_repository: FamilyNameRepository
}

impl FamilyRankingFeatureExtractor{
    pub fn get_features(&self, family: &String, given: &String) -> FamilyRankingFeatures{
        let rank = self.family_name_repository.get_rank(family);
        let family_length = family.chars().count();
        let family_length_f64 = family_length as f64;
        let given_length = given.chars().count();
        let given_length_f64 = given_length as f64;
        let fullname_length = family_length + given_length;
        let fullname_length_f64 = family_length_f64 + given_length_f64;
        let family_order_score = F::calc_order_score(
            &self.kanji_statistics_repository,
            family,
            fullname_length,
            0
        );
        let family_length_score = F::calc_length_score(
            &self.kanji_statistics_repository,
            family,
            fullname_length,
            0
        );
        let given_order_score = F::calc_order_score(
            &self.kanji_statistics_repository,
            given,
            fullname_length,
            family.chars().count()
        );
        let given_length_score = F::calc_length_score(
            &self.kanji_statistics_repository,
            given,
            fullname_length,
            family.chars().count()
        );

        let mut given_startswith_specific_kanji= false;
        let specific_kanjis = ["田", "谷", "川", "島", "原", "村", "塚", "森", "井", "子"];
        for _specific_kanji in specific_kanjis{
            if given.starts_with(_specific_kanji){
                given_startswith_specific_kanji = true;
                break;
            }
        }
        let given_startswith_specific_kanji_f64 = given_startswith_specific_kanji as i32 as f64;

        FamilyRankingFeatures{
            rank,
            fullname_length: fullname_length_f64,
            family_length: family_length_f64,
            given_length: given_length_f64,
            family_order_score,
            given_order_score,
            family_length_score,
            given_length_score,
            given_startswith_specific_kanji: given_startswith_specific_kanji_f64
        }
    }
}