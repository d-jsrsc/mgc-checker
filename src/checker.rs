#![allow(dead_code)]
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::str::Chars;
use std::sync::RwLock;

use lazy_static::lazy_static;

pub enum MatchType {
    MinMatchType, //最小匹配规则
    MaxMatchType, //最大匹配规则
}

#[derive(Debug)]
pub struct SensitiveWordMap {
    is_end: char,
    word: char,
    word_map: Option<HashMap<char, Box<SensitiveWordMap>>>,
}

lazy_static! {
    static ref SENSITIVE_WORD_MAP: RwLock<HashMap<char, SensitiveWordMap>> =
        RwLock::new(HashMap::<char, SensitiveWordMap>::new());
}

pub fn is_contains_sensitive_word(txt: &str, match_type: &MatchType) -> bool {
    let mut is_contains = false;
    let len = txt.chars().count();
    let mut i = 0;
    while i < len {
        let length = check_sensitive_word(txt, i, match_type);
        if length > 0 {
            is_contains = true;
            break;
        }
        i += 1;
    }
    is_contains
}
fn check_sensitive_word(txt: &str, begin_index: usize, match_type: &MatchType) -> usize {
    let mut match_flag = 0;
    let mut last_match_length = 0;
    let txt_vec: Vec<char> = txt.chars().collect();
    if let Some(word) = &txt_vec.get(begin_index) {
        if let Some(swm) = SENSITIVE_WORD_MAP.read().unwrap().get(word) {
            match_flag += 1;
            if (*swm).is_end == '1' {
                last_match_length = match_flag;

                match match_type {
                    MatchType::MinMatchType => {
                        return last_match_length;
                    }
                    MatchType::MaxMatchType => (),
                }
            }

            //递归查找
            let mut j = begin_index + 1;
            recursive_find_map(
                swm,
                &txt_vec,
                &mut j,
                &mut match_flag,
                &mut last_match_length,
                match_type,
            );
        }
    }
    last_match_length
}

fn recursive_find_map(
    swm: &SensitiveWordMap,
    txt_vec: &[char],
    i: &mut usize,
    match_flag: &mut usize,
    last_match_length: &mut usize,
    match_type: &MatchType,
) {
    if let Some(word) = txt_vec.get(*i) {
        if let Some(wm) = &swm.word_map {
            if let Some(next_swm) = wm.get(word) {
                *match_flag += 1;

                if swm.is_end == '1' {
                    *last_match_length = *match_flag;
                    match match_type {
                        MatchType::MinMatchType => {
                            return;
                        }
                        MatchType::MaxMatchType => (),
                    }
                }

                if next_swm.is_end == '1' {
                    *last_match_length = *match_flag;
                    match match_type {
                        MatchType::MinMatchType => {
                            return;
                        }
                        MatchType::MaxMatchType => (),
                    }
                }

                if let Some(nwm) = &next_swm.word_map {
                    if nwm.is_empty() {
                        *last_match_length = *match_flag;
                        match match_type {
                            MatchType::MinMatchType => {
                                return;
                            }
                            MatchType::MaxMatchType => (),
                        }
                    }
                }

                *i += 1;
                recursive_find_map(
                    next_swm,
                    txt_vec,
                    i,
                    match_flag,
                    last_match_length,
                    match_type,
                );
            }
        }
    }
}

fn recursive_build_map(map: &mut SensitiveWordMap, chars: &mut Chars, count: &mut usize) {
    if let Some(ch) = chars.next() {
        *count -= 1;
        if let Some(now_map) = map.word_map.as_mut() {
            // let contains_key = now_map.contains_key(&ch);

            if let std::collections::hash_map::Entry::Vacant(e) = now_map.entry(ch) {
                let is_end = if *count == 0 { '1' } else { '0' };
                let swm = SensitiveWordMap {
                    word: ch,
                    is_end,
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                e.insert(Box::new(swm));
                if let Some(m) = now_map.get_mut(&ch) {
                    recursive_build_map(&mut *m, &mut *chars, count);
                }
            } else if let Some(m) = now_map.get_mut(&ch) {
                recursive_build_map(&mut *m, &mut *chars, count);
            }
        }
    }
}

pub fn build_sensitive_word_map(set: BTreeSet<String>) {
    let mut sensitive_word_map = HashMap::<char, SensitiveWordMap>::new();

    let iterator = set.iter();
    for key in iterator {
        let len = key.chars().count();
        let mut count = len;
        let mut key_chars = key.chars();
        //读取每行的首个字符
        if let Some(first_char) = key_chars.next() {
            count -= 1;
            if let Some(word_map) = sensitive_word_map.get_mut(&first_char) {
                //读取下一个字符
                recursive_build_map(&mut *word_map, &mut key_chars, &mut count);
            } else {
                let is_end = if len == 1 { '1' } else { '0' };

                let now_map = SensitiveWordMap {
                    word: first_char,
                    is_end,
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                sensitive_word_map.insert(first_char, now_map);

                if let Some(now_map) = sensitive_word_map.get_mut(&first_char) {
                    recursive_build_map(&mut *now_map, &mut key_chars, &mut count);
                }
            }
        }
    }
    let mut new_settings = SENSITIVE_WORD_MAP.write().unwrap();
    *new_settings = sensitive_word_map;
}
