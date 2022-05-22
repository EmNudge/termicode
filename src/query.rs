use crate::data::{UnicodeData, UnicodeFile};
use regex::Regex;

pub fn query_name<'a>(
    name: String,
    unicode_file: &'a UnicodeFile,
) -> Box<dyn Iterator<Item = &'a UnicodeData> + 'a> {
    let is_regex_re = Regex::new(r"^/(.+?)/[a-z]*$").unwrap();
    if let Some(captures) = is_regex_re.captures(&name.to_uppercase()) {
        let user_regex_str = captures.get(1).unwrap().as_str();
        if let Ok(user_regex) = Regex::new(user_regex_str) {
            return Box::new(
                unicode_file
                    .map
                    .iter()
                    .filter(move |(_codepoint, character_data)| {
                        user_regex.is_match(&character_data.name)
                    })
                    .map(|(_codepoint, character_data)| character_data),
            );
        }
    }

    Box::new(
        unicode_file
            .map
            .iter()
            .filter(move |(_codepoint, character_data)| {
                (*character_data).name.contains(&name.to_uppercase())
            })
            .map(|(_codepoint, character_data)| character_data),
    )
}
