use crate::feature::assets::Asset;
use std::collections::HashMap;

pub struct FamilyNameRepository {
    family_names: HashMap<String, f64>,
}

impl FamilyNameRepository {
    pub fn new() -> Self {
        let contents = Asset::get("family_names.txt")
            .unwrap()
            .data
            .as_ref()
            .to_owned();
        let contents_str = std::str::from_utf8(&contents).unwrap();
        let mut family_names = HashMap::new();

        for (rank, family_name) in contents_str.lines().enumerate() {
            family_names.insert(family_name.to_string(), rank as f64);
        }

        Self { family_names }
    }

    pub fn get_rank(&self, family: &String) -> f64 {
        let rank = self.family_names.get(family);
        if rank.is_none() {
            f64::NAN
        } else {
            *rank.unwrap()
        }
    }
}
