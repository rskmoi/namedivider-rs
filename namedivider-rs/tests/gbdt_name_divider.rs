use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::name_divider::NameDivider;

#[test]
fn divide_rule_two_char() {
    let undivided_name = "原敬".to_string();
    let divider = get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string());
    let divided_name = divider.divide_name(&undivided_name);
    assert_eq!(divided_name.family, "原".to_string());
    assert_eq!(divided_name.given, "敬".to_string());
    assert_eq!(divided_name.separator, " ".to_string());
    assert_eq!(divided_name.score, 1.0);
    assert_eq!(divided_name.algorithm, "rule".to_string());
}

#[test]
fn divide_rule_with_kana() {
    let undivided_name = "中山マサ".to_string();
    let divider = get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string());
    let divided_name = divider.divide_name(&undivided_name);
    assert_eq!(divided_name.family, "中山".to_string());
    assert_eq!(divided_name.given, "マサ".to_string());
    assert_eq!(divided_name.separator, " ".to_string());
    assert_eq!(divided_name.score, 1.0);
    assert_eq!(divided_name.algorithm, "rule".to_string());
}

#[test]
fn divide_with_feature() {
    let undivided_name = "菅義偉".to_string();
    let divider = get_gbdt_name_divider(" ".to_string(), true, "gbdt".to_string());
    let divided_name = divider.divide_name(&undivided_name);
    assert_eq!(divided_name.family, "菅".to_string());
    assert_eq!(divided_name.given, "義偉".to_string());
    assert_eq!(divided_name.separator, " ".to_string());
    // assert_eq!(divided_name.score, 1.0);
    assert_eq!(divided_name.algorithm, "gbdt".to_string());
}
