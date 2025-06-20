use crate::divider::divided_name::DividedName;
use crate::divider::score_calculator::ScoreCalculator;
use regex::Regex;

fn slice(undivided_name: &String, idx: usize) -> (String, String) {
    let mut family = "".to_string();
    let mut given = "".to_string();
    for (i, c) in undivided_name.chars().enumerate() {
        if i < idx {
            family.push(c);
        } else {
            given.push(c);
        }
    }
    (family, given)
}

pub struct UndividedNameHolder {
    pub original_name: String,
    pub normalized_name: String,
}

impl UndividedNameHolder {
    pub fn new(original_name: String) -> Self {
        let mut normalized_name = original_name.clone();
        let old_new_pairs = [("髙", "高"), ("𠮷", "吉")];
        for (old, new) in old_new_pairs {
            normalized_name = normalized_name.replace(old, new);
        }
        Self {
            original_name,
            normalized_name,
        }
    }

    pub fn get_divided_original_name(&self, divided_normalized_name: DividedName) -> DividedName {
        let family_length = divided_normalized_name.family.chars().count();
        let (family, given) = slice(&self.original_name, family_length);
        DividedName {
            family,
            given,
            separator: divided_normalized_name.separator,
            score: divided_normalized_name.score,
            algorithm: divided_normalized_name.algorithm,
        }
    }
}

pub struct NameDividerBase {
    pub separator: String,
    pub normalize_name: bool,
    pub algorithm_name: String,
    pub compiled_regex_kanji: Regex,
}

impl NameDividerBase {
    pub fn new(separator: String, normalize_name: bool, algorithm_name: String) -> Self {
        Self {
            separator,
            normalize_name,
            algorithm_name,
            compiled_regex_kanji: Regex::new(r"\p{Script=Han}+").unwrap(),
        }
    }

    fn create_divided_name(
        &self,
        family: String,
        given: String,
        score: f64,
        algorithm: String,
    ) -> DividedName {
        DividedName {
            family,
            given,
            separator: self.separator.clone(),
            score,
            algorithm,
        }
    }

    fn validate(&self, undivided_name: &String) -> Result<(), &'static str> {
        if undivided_name.chars().count() < 2 {
            return Err("Name length needs at least 2 chars.");
        }
        Ok(())
    }

    fn divide_by_rule_base(&self, undivided_name: &String) -> Option<DividedName> {
        let fullname_length = undivided_name.chars().count();
        if fullname_length == 2 {
            return Some(self.create_divided_name(
                undivided_name.chars().next().unwrap().to_string(),
                undivided_name.chars().nth(1).unwrap().to_string(),
                1.0,
                "rule".to_string(),
            ));
        }

        let mut is_kanji_list: Vec<bool> = Vec::new();
        for (i, c) in undivided_name.chars().enumerate() {
            let is_kanji: bool = self.compiled_regex_kanji.is_match(&c.to_string());
            is_kanji_list.push(is_kanji);
            if i >= 2
                && is_kanji_list[0] != is_kanji
                && is_kanji_list[is_kanji_list.len() - 2] == is_kanji
            {
                let (family, given) = slice(undivided_name, i - 1);
                return Some(self.create_divided_name(family, given, 1.0, "rule".to_string()));
            }
        }

        None
    }

    fn divide_by_algorithm(
        &self,
        undivided_name: &String,
        score_calculator: &impl ScoreCalculator,
    ) -> DividedName {
        let mut scores: Vec<f64> = Vec::new();
        let fullname_length = undivided_name.chars().count();
        for idx in 1..fullname_length {
            let (family, given) = slice(undivided_name, idx);
            let score = score_calculator.calc_score(&family, &given);
            scores.push(score);
        }

        let mut sum = 0.0;
        let mut max_idx = 0;
        let mut max_val = 0.0;
        for (i, _score) in scores.into_iter().enumerate() {
            let _exp = _score.exp();
            if max_val < _exp {
                max_val = _exp;
                max_idx = i;
            }
            sum += _score.exp();
        }
        let score = max_val / sum;
        let (pred_family, pred_given) = slice(undivided_name, max_idx + 1);

        self.create_divided_name(pred_family, pred_given, score, self.algorithm_name.clone())
    }

    fn _divide_name(
        &self,
        undivided_name: &String,
        score_calculator: &impl ScoreCalculator,
    ) -> DividedName {
        let divided_name_by_rule_base = self.divide_by_rule_base(undivided_name);
        if divided_name_by_rule_base.is_some() {
            return divided_name_by_rule_base.unwrap();
        }
        self.divide_by_algorithm(undivided_name, score_calculator)
    }

    pub fn divide_name(
        &self,
        undivided_name: &String,
        score_calculator: &impl ScoreCalculator,
    ) -> DividedName {
        self.validate(undivided_name).unwrap();
        if self.normalize_name {
            let holder = UndividedNameHolder::new(undivided_name.clone());
            let divided_name = self._divide_name(&holder.normalized_name, score_calculator);
            holder.get_divided_original_name(divided_name)
        } else {
            self._divide_name(undivided_name, score_calculator)
        }
    }
}
