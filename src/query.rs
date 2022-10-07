use crate::data::{UnicodeData, UnicodeFile};
use regex::Regex;

const MAX_GAP: (u32, u32) = (u32::MAX, u32::MAX);

fn get_unicode_items<'a>(
    unicode_file: &'a UnicodeFile,
) -> Box<dyn Iterator<Item = &'a UnicodeData> + 'a> {
    let mut gaps_iter = unicode_file.gaps.clone().into_iter();
    let mut next_gap = gaps_iter.next().unwrap_or(MAX_GAP);

    let mut hash_index: u32 = 0;

    let iter = (0..(unicode_file.map.len() - 1)).map(move |_| {
        hash_index += 1;
        if hash_index > next_gap.0 {
            hash_index = next_gap.1;
            next_gap = gaps_iter.next().unwrap_or(MAX_GAP);
        }

        unicode_file.map.get(&hash_index).unwrap()
    });

    Box::new(iter)
}

pub fn query_name<'a>(
    name: String,
    unicode_file: &'a UnicodeFile,
) -> Box<dyn Iterator<Item = &'a UnicodeData> + 'a> {
    let is_regex_re = Regex::new(r"^/(.+?)/[a-z]*$").unwrap();
    if let Some(captures) = is_regex_re.captures(&name.to_uppercase()) {
        let user_regex_str = captures.get(1).unwrap().as_str();
        if let Ok(user_regex) = Regex::new(user_regex_str) {
            return Box::new(
                get_unicode_items(unicode_file)
                    .filter(move |character_data| user_regex.is_match(&character_data.name)),
            );
        }
    }

    Box::new(
        get_unicode_items(unicode_file)
            .filter(move |character_data| (*character_data).name.contains(&name.to_uppercase())),
    )
}
