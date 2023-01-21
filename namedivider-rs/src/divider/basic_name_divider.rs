use regex::Regex;
use crate::feature::extractor::SimpleFeatureExtractor;
use crate::feature::kanji::KanjiStatisticsRepository;
use crate::divider::divided_name::DividedName;
use crate::divider::name_divider_base::NameDividerBase;
use crate::divider::basic_score_calculator::BasicScoreCalculator;
use crate::divider::name_divider::NameDivider;

pub struct BasicNameDivider{
    pub basic_score_calculator: BasicScoreCalculator,
    pub name_divider_base: NameDividerBase
}

impl NameDivider for BasicNameDivider{
    fn divide_name(&self, undivided_name: &String) -> DividedName{
        self.name_divider_base.divide_name(undivided_name, &self.basic_score_calculator)
    }
}

pub fn get_basic_name_divider(
    separator: String,
    normalize_name: bool,
    algorithm_name: String,
    only_order_score_when_4: bool) -> BasicNameDivider{
    let repo = KanjiStatisticsRepository::new();
    let feature_extractor = SimpleFeatureExtractor{ kanji_statistics_repository: repo};
    let basic_score_calculator = BasicScoreCalculator{
        feature_extractor,
        only_order_score_when_4
    };
    let name_divider_base = NameDividerBase{
        separator,
        normalize_name,
        algorithm_name,
        compiled_regex_kanji: Regex::new(r"\p{Script=Han}+").unwrap()
    };
    BasicNameDivider{ basic_score_calculator, name_divider_base }
}