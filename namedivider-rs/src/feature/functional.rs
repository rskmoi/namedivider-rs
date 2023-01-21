use crate::feature::kanji;

fn _create_order_mask(full_name_length: &usize, char_idx: &usize) -> Result<Vec<i32>, &'static str>{
    if char_idx == &0 || char_idx == &(full_name_length - 1) {
        return Err("First character and last character must not be created order mask.");
    }
    if full_name_length == &3{
        return Ok(vec![0, 0, 1, 1, 0, 0]);
    }
    if char_idx == &1{
        return Ok(vec![0, 1, 1, 1, 0, 0]);
    }
    if char_idx == &(full_name_length - 2){
        return Ok(vec![0, 0, 1, 1, 1, 0]);
    }
    Ok(vec![0, 1, 1, 1, 1, 0])
}

fn _create_length_mask(full_name_length: &usize, char_idx: &usize) -> Vec<i32>{
    let min_family = char_idx + 1;
    let min_family_idx = if min_family > 4 { 4 } else { min_family };
    let max_family = full_name_length - 1;
    let max_family_idx = if max_family > 4 { 4 } else { max_family };
    let min_given = full_name_length - char_idx;
    let min_given_idx = if min_given > 4 { 4 } else { min_given };
    let max_given = full_name_length - 1;
    let max_given_idx = if max_given > 4 { 4 } else { max_given };

    let mut lc_family = vec![0, 0, 0, 0];
    if min_family <= max_family{
        for i in min_family_idx - 1..max_family_idx{
            lc_family[i] = 1
        }
    }
    let mut lc_given = vec![0, 0, 0, 0];
    if min_given <= max_given{
        for i in min_given_idx - 1..max_given_idx{
            lc_given[i] = 1
        }
    }
    lc_family.append(&mut lc_given);
    lc_family
}

fn _calc_current_order_status(piece_of_divided_name: &String, idx_in_piece_of_divided_name: &usize, is_family: bool)  -> usize{
    return if idx_in_piece_of_divided_name == &0 {
        if is_family { 0 } else { 3 }
    } else if idx_in_piece_of_divided_name == &(piece_of_divided_name.chars().count() - 1) {
        if is_family { 2 } else { 5 }
    } else if is_family { 1 } else { 4 }
}

fn _calc_current_length_status(piece_of_divided_name: &String, is_family: bool) -> usize{
    let piece_of_divided_name_length = piece_of_divided_name.chars().count();
    let length_for_get_status = if piece_of_divided_name_length <= 4 {piece_of_divided_name_length} else {4};
    if is_family {length_for_get_status - 1} else {length_for_get_status - 1 + 4}
}

pub fn calc_order_score(
    kanji_statistics_repository: &kanji::KanjiStatisticsRepository,
    piece_of_divided_name: &String,
    full_name_length: usize,
    start_index: usize) -> f64{
    let is_family = start_index == 0;
    let mut scores = 0.0;
    for (idx_in_piece_of_divided_name, _kanji) in piece_of_divided_name.chars().into_iter().enumerate(){
        let current_index = start_index + idx_in_piece_of_divided_name;
        if current_index == 0 || current_index == (full_name_length - 1) {
            continue;
        }
        let mask = _create_order_mask(&full_name_length, &current_index).unwrap();
        let current_order_status_idx = _calc_current_order_status(
            piece_of_divided_name,
            &idx_in_piece_of_divided_name,
            is_family
        );
        let _order_counts = &kanji_statistics_repository.get(&_kanji.to_string()).order_counts;
        let mut masked_order_scores = vec![0, 0, 0, 0, 0, 0];
        for i in 0..6{
            masked_order_scores[i] = &_order_counts[i] * mask[i];
        }
        let _sum: f64 = f64::from(masked_order_scores.iter().sum::<i32>());
        if _sum == 0.0{
            continue;
        }
        let _score = f64::from(masked_order_scores[current_order_status_idx]) / _sum;
        scores += _score;
    }
    scores
}

pub fn calc_length_score(
    kanji_statistics_repository: &kanji::KanjiStatisticsRepository,
    piece_of_divided_name: &String,
    full_name_length: usize,
    start_index: usize) -> f64{
    let is_family = start_index == 0;
    let mut scores = 0.0;
    for (idx_in_piece_of_divided_name, _kanji) in piece_of_divided_name.chars().into_iter().enumerate(){
        let current_idx = start_index + idx_in_piece_of_divided_name;
        let mask = _create_length_mask(&full_name_length, &current_idx);
        let current_length_status_idx = _calc_current_length_status(piece_of_divided_name, is_family);
        let _length_counts = &kanji_statistics_repository.get(&_kanji.to_string()).length_counts;
        let mut masked_length_scores = vec![0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..8{
            masked_length_scores[i] = &_length_counts[i] * mask[i];
        }
        let _sum: f64 = f64::from(masked_length_scores.iter().sum::<i32>());
        if _sum == 0.0{
            continue;
        }
        let _score = f64::from(masked_length_scores[current_length_status_idx]) / _sum;
        scores += _score;
    }
    scores
}