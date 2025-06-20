use crate::divider::divided_name::DividedName;
use crate::divider::name_divider::NameDivider;
use crate::divider::name_divider_base::NameDividerBase;
use crate::divider::two_char_score_calculator::TwoCharScoreCalculator;
use regex::Regex;

pub struct TwoCharNameDivider {
    pub two_char_score_calculator: TwoCharScoreCalculator,
    pub name_divider_base: NameDividerBase,
}

impl NameDivider for TwoCharNameDivider {
    fn divide_name(&self, undivided_name: &String) -> DividedName {
        self.name_divider_base
            .divide_name(undivided_name, &self.two_char_score_calculator)
    }
}

pub fn get_two_char_name_divider(
    separator: String,
    normalize_name: bool,
    algorithm_name: String,
) -> TwoCharNameDivider {
    let two_char_score_calculator = TwoCharScoreCalculator {};
    let name_divider_base = NameDividerBase {
        separator,
        normalize_name,
        algorithm_name,
        compiled_regex_kanji: Regex::new(r"\p{Script=Han}+").unwrap(),
    };
    TwoCharNameDivider {
        two_char_score_calculator,
        name_divider_base,
    }
}
