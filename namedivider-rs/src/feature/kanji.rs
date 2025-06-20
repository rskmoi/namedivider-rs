use crate::feature::assets::Asset;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct KanjiStatistics {
    pub kanji: String,
    pub order_counts: Vec<i32>,
    pub length_counts: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct KanjiStatisticsVec {
    kanji_statistics_vec: Vec<KanjiStatistics>,
}

pub struct KanjiStatisticsRepository {
    kanji_dict: HashMap<String, KanjiStatistics>,
    default_kanji: KanjiStatistics,
}

impl KanjiStatisticsRepository {
    pub fn new() -> Self {
        let contents = Asset::get("kanji.json").unwrap().data.as_ref().to_owned();
        let contents_str = std::str::from_utf8(&contents).unwrap();
        let deserialized: KanjiStatisticsVec = serde_json::from_str(contents_str).unwrap();
        let mut kanji_dict = HashMap::new();
        for _d in deserialized.kanji_statistics_vec {
            kanji_dict.insert(
                _d.kanji.clone(),
                KanjiStatistics {
                    kanji: _d.kanji.clone(),
                    order_counts: _d.order_counts.clone(),
                    length_counts: _d.length_counts.clone(),
                },
            );
        }

        Self {
            kanji_dict,
            default_kanji: KanjiStatistics {
                kanji: "default".to_string(),
                order_counts: vec![0, 0, 0, 0, 0, 0],
                length_counts: vec![0, 0, 0, 0, 0, 0, 0, 0],
            },
        }
    }

    pub fn get(&self, key: &String) -> &KanjiStatistics {
        let kanji = self.kanji_dict.get(key);
        if kanji.is_none() {
            return &self.default_kanji;
        }
        kanji.unwrap()
    }
}
