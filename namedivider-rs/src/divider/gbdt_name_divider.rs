use crate::divider::divided_name::DividedName;
use crate::divider::gbdt_score_calculator::GBDTScoreCalculator;
use crate::divider::name_divider::NameDivider;
use crate::divider::name_divider_base::NameDividerBase;
use crate::feature::assets::Asset;
use crate::feature::extractor::FamilyRankingFeatureExtractor;
use crate::feature::family_name::FamilyNameRepository;
use crate::feature::kanji::KanjiStatisticsRepository;
use lightgbm::Booster;
use regex::Regex;

pub struct GBDTNameDivider {
    pub gbdt_score_calculator: GBDTScoreCalculator,
    pub name_divider_base: NameDividerBase,
}

impl NameDivider for GBDTNameDivider {
    fn divide_name(&self, undivided_name: &String) -> DividedName {
        self.name_divider_base
            .divide_name(undivided_name, &self.gbdt_score_calculator)
    }
}

pub fn get_gbdt_name_divider(
    separator: String,
    normalize_name: bool,
    algorithm_name: String,
) -> GBDTNameDivider {
    let kanji_statistics_repository = KanjiStatisticsRepository::new();
    let family_name_repository = FamilyNameRepository::new();
    let feature_extractor = FamilyRankingFeatureExtractor {
        kanji_statistics_repository,
        family_name_repository,
    };

    let contents = Asset::get("gbdt_model_v1.txt")
        .unwrap()
        .data
        .as_ref()
        .to_owned();
    let model_str = std::str::from_utf8(&contents).unwrap();
    let model = Booster::from_string(model_str).unwrap();

    let gbdt_score_calculator = GBDTScoreCalculator {
        feature_extractor,
        model,
    };
    let name_divider_base = NameDividerBase {
        separator,
        normalize_name,
        algorithm_name,
        compiled_regex_kanji: Regex::new(r"\p{Script=Han}+").unwrap(),
    };
    GBDTNameDivider {
        gbdt_score_calculator,
        name_divider_base,
    }
}
