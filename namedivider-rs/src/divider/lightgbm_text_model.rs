const CATEGORICAL_MASK: i8 = 1;
const DEFAULT_LEFT_MASK: i8 = 2;
const MISSING_TYPE_NONE: i8 = 0;
const MISSING_TYPE_ZERO: i8 = 1;
const MISSING_TYPE_NAN: i8 = 2;
const ZERO_THRESHOLD: f64 = 1e-35;

#[derive(Debug)]
pub struct LightGbmTextModel {
    trees: Vec<LightGbmTree>,
    sigmoid: f64,
}

#[derive(Debug)]
struct LightGbmTree {
    split_feature: Vec<usize>,
    threshold: Vec<f64>,
    decision_type: Vec<i8>,
    left_child: Vec<i32>,
    right_child: Vec<i32>,
    leaf_value: Vec<f64>,
}

impl LightGbmTextModel {
    pub fn parse(model_str: &str) -> Self {
        let mut sigmoid = 1.0;
        let mut trees = Vec::new();
        let mut current_tree_lines = Vec::new();

        for line in model_str.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(objective) = line.strip_prefix("objective=") {
                sigmoid = parse_sigmoid(objective);
                continue;
            }

            if line.starts_with("Tree=") || line == "end of trees" {
                if !current_tree_lines.is_empty() {
                    trees.push(LightGbmTree::parse(&current_tree_lines));
                    current_tree_lines.clear();
                }
            }

            if line == "end of trees" {
                break;
            }

            if !trees.is_empty() || line.starts_with("Tree=") || !current_tree_lines.is_empty() {
                current_tree_lines.push(line);
            }
        }

        if !current_tree_lines.is_empty() {
            trees.push(LightGbmTree::parse(&current_tree_lines));
        }

        assert!(!trees.is_empty(), "LightGBM model contains no trees");

        Self { trees, sigmoid }
    }

    pub fn predict(&self, features: &[f64]) -> f64 {
        let raw_score = self
            .trees
            .iter()
            .map(|tree| tree.predict(features))
            .sum::<f64>();
        1.0 / (1.0 + (-self.sigmoid * raw_score).exp())
    }
}

impl LightGbmTree {
    fn parse(lines: &[&str]) -> Self {
        let mut num_leaves = None;
        let mut num_cat = None;
        let mut split_feature = None;
        let mut threshold = None;
        let mut decision_type = None;
        let mut left_child = None;
        let mut right_child = None;
        let mut leaf_value = None;
        let mut is_linear = None;

        for line in lines {
            if let Some(value) = line.strip_prefix("num_leaves=") {
                num_leaves = Some(parse_scalar::<usize>(value));
            } else if let Some(value) = line.strip_prefix("num_cat=") {
                num_cat = Some(parse_scalar::<usize>(value));
            } else if let Some(value) = line.strip_prefix("split_feature=") {
                split_feature = Some(parse_vec::<usize>(value));
            } else if let Some(value) = line.strip_prefix("threshold=") {
                threshold = Some(parse_vec::<f64>(value));
            } else if let Some(value) = line.strip_prefix("decision_type=") {
                decision_type = Some(parse_vec::<i8>(value));
            } else if let Some(value) = line.strip_prefix("left_child=") {
                left_child = Some(parse_vec::<i32>(value));
            } else if let Some(value) = line.strip_prefix("right_child=") {
                right_child = Some(parse_vec::<i32>(value));
            } else if let Some(value) = line.strip_prefix("leaf_value=") {
                leaf_value = Some(parse_vec::<f64>(value));
            } else if let Some(value) = line.strip_prefix("is_linear=") {
                is_linear = Some(parse_scalar::<u8>(value));
            }
        }

        let num_leaves = num_leaves.expect("LightGBM tree is missing num_leaves");
        let num_cat = num_cat.expect("LightGBM tree is missing num_cat");
        let split_feature = split_feature.expect("LightGBM tree is missing split_feature");
        let threshold = threshold.expect("LightGBM tree is missing threshold");
        let decision_type = decision_type.expect("LightGBM tree is missing decision_type");
        let left_child = left_child.expect("LightGBM tree is missing left_child");
        let right_child = right_child.expect("LightGBM tree is missing right_child");
        let leaf_value = leaf_value.expect("LightGBM tree is missing leaf_value");

        assert_eq!(num_cat, 0, "categorical LightGBM splits are not supported");
        assert_eq!(is_linear.unwrap_or(0), 0, "linear LightGBM trees are not supported");
        assert_eq!(leaf_value.len(), num_leaves, "invalid LightGBM leaf count");

        let num_internal_nodes = num_leaves - 1;
        assert_eq!(split_feature.len(), num_internal_nodes, "invalid split_feature length");
        assert_eq!(threshold.len(), num_internal_nodes, "invalid threshold length");
        assert_eq!(decision_type.len(), num_internal_nodes, "invalid decision_type length");
        assert_eq!(left_child.len(), num_internal_nodes, "invalid left_child length");
        assert_eq!(right_child.len(), num_internal_nodes, "invalid right_child length");

        for decision in &decision_type {
            assert!(
                decision & CATEGORICAL_MASK == 0,
                "categorical LightGBM splits are not supported"
            );
            let missing_type = get_missing_type(*decision);
            assert!(
                matches!(missing_type, MISSING_TYPE_NONE | MISSING_TYPE_ZERO | MISSING_TYPE_NAN),
                "unsupported LightGBM missing type: {missing_type}"
            );
        }

        Self {
            split_feature,
            threshold,
            decision_type,
            left_child,
            right_child,
            leaf_value,
        }
    }

    fn predict(&self, features: &[f64]) -> f64 {
        let mut node = 0usize;
        loop {
            let feature_index = self.split_feature[node];
            let feature_value = *features
                .get(feature_index)
                .unwrap_or_else(|| panic!("missing feature index {feature_index}"));
            let child = self.next_child(node, feature_value);
            if child < 0 {
                return self.leaf_value[(-child - 1) as usize];
            }
            node = child as usize;
        }
    }

    fn next_child(&self, node: usize, mut feature_value: f64) -> i32 {
        let decision_type = self.decision_type[node];
        let missing_type = get_missing_type(decision_type);

        if feature_value.is_nan() && missing_type != MISSING_TYPE_NAN {
            feature_value = 0.0;
        }

        let is_missing = (missing_type == MISSING_TYPE_ZERO && is_zero(feature_value))
            || (missing_type == MISSING_TYPE_NAN && feature_value.is_nan());
        if is_missing {
            return if decision_type & DEFAULT_LEFT_MASK != 0 {
                self.left_child[node]
            } else {
                self.right_child[node]
            };
        }

        if feature_value <= self.threshold[node] {
            self.left_child[node]
        } else {
            self.right_child[node]
        }
    }
}

fn parse_sigmoid(objective: &str) -> f64 {
    let Some(sigmoid) = objective.strip_prefix("binary sigmoid:") else {
        panic!("unsupported LightGBM objective: {objective}");
    };
    parse_scalar(sigmoid)
}

fn get_missing_type(decision_type: i8) -> i8 {
    (decision_type >> 2) & 3
}

fn is_zero(value: f64) -> bool {
    (-ZERO_THRESHOLD..=ZERO_THRESHOLD).contains(&value)
}

fn parse_vec<T>(value: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    value.split_whitespace().map(parse_scalar).collect()
}

fn parse_scalar<T>(value: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    value.parse::<T>().unwrap_or_else(|err| {
        panic!("failed to parse LightGBM scalar '{value}': {err:?}");
    })
}

#[cfg(test)]
mod tests {
    use super::LightGbmTextModel;

    #[test]
    fn parses_and_predicts_tiny_binary_model() {
        let model = LightGbmTextModel::parse(
            "tree
version=v3
num_class=1
num_tree_per_iteration=1
objective=binary sigmoid:1

Tree=0
num_leaves=2
num_cat=0
split_feature=0
split_gain=1
threshold=0.5
decision_type=2
left_child=-1
right_child=-2
leaf_value=-1 1
is_linear=0
shrinkage=1

end of trees
",
        );

        assert!((model.predict(&[0.25]) - 0.2689414213699951).abs() < 1e-15);
        assert!((model.predict(&[0.75]) - 0.7310585786300049).abs() < 1e-15);
    }

    #[test]
    fn parses_bundled_gbdt_model() {
        let model = LightGbmTextModel::parse(include_str!("../assets/gbdt_model_v1.txt"));
        let prediction = model.predict(&[1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.0]);

        assert_eq!(model.trees.len(), 268);
        assert!(prediction > 0.0);
        assert!(prediction < 1.0);
    }
}
