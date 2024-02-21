use regex::Regex;
use std::collections::HashMap;

const DYNAMIC_PATH_STARTS_WITH: &str = ":";

/// Path To Regex
///
///
///
/// TODO: include ':' means, dynamic
/// TODO: implement '$' starts with
pub fn path_to_regex(sanitized_path: &str) -> String {
    let mut regex = String::new();

    let mut paths = sanitized_path.split(DYNAMIC_PATH_STARTS_WITH);

    // 1st item
    regex.push_str(paths.next().unwrap_or_default());

    // rest items
    paths.for_each(|path_item| {
        if let Some((name, rest)) = path_item.split_once('/') {
            regex.push_str(&naming_regex_maker(name));
            regex.push('/');
            regex.push_str(rest);
        } else {
            regex.push_str(&naming_regex_maker(path_item));
        }
    });

    // end the regex
    regex.push('$');

    regex
}

fn naming_regex_maker(name: &str) -> String {
    format!(r"(?<{}>\w+)", name)
}

pub type Params = HashMap<String, String>;

/// Path Regex Matcher
///
/// - `target_pathname` must be provided from `request.pathname`.
pub fn path_regex_matcher<'a>(regex_path: &str, target_pathname: &'a str) -> Option<Params> {
    let regex_path = Regex::new(regex_path).expect("ERROR: Regex::new()");

    if let Some(captures) = regex_path.captures(target_pathname) {
        let params = regex_path
            .capture_names()
            .into_iter()
            .filter_map(|name| name)
            .fold(Params::new(), |mut acc, name| {
                let value = captures.name(name).map_or("", |i| i.as_str());

                acc.insert(name.to_string(), value.to_string());

                acc
            });

        return Some(params);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_regex() {
        assert_eq!(
            path_to_regex("/users/:userId"),
            r"/users/(?<userId>\w+)$",
            "Single dynamic regex part."
        );
        assert_eq!(
            path_to_regex("/users/:userId/messages"),
            r"/users/(?<userId>\w+)/messages$"
        );
        assert_eq!(
            path_to_regex("/users/:userId/:messageId"),
            r"/users/(?<userId>\w+)/(?<messageId>\w+)$",
            "Multiple dynamic regex parts."
        );
        assert_eq!(path_to_regex("/users/list"), r"/users/list$");
        assert_eq!(path_to_regex("/"), r"/$");
    }

    #[test]
    fn test_naming_regex_maker() {
        assert_eq!(naming_regex_maker("userId"), r"(?<userId>\w+)");
    }
}
