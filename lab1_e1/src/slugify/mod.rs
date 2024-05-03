use std::iter::zip;

// a str cannot be indexed in Rust because Strings are encoded in UTF-8 internally, so the concept
// of indexing itself would be ambiguous -> byte indexing could be used, is fast, but almost always incorrect

pub fn slugify(s: &str) -> String {
    let mut slug = String::new();

    for c in s.chars() {
        slug.push(conv(c));
    }

    if (slug.len() > 1 && slug.chars().all(|c| c == '-')) || (slug.len() == 1 && slug.contains('-'))
    {
        return String::from('-');
    }

    let mut prev_ch = ' ';
    slug.retain(|c| {
        let keep = c != '-' || prev_ch != '-';
        prev_ch = c;
        keep
    });

    if slug.chars().last().unwrap() == '-' {
        let mut chars = slug.chars();
        chars.next_back();
        slug = String::from(chars.as_str())
    }
    slug
}

pub fn conv(c: char) -> char {
    const SUBS_I: &str =
        "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str =
        "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    return if c.is_ascii_alphanumeric() {
        c.to_ascii_lowercase()
    } else {
        for (ci, co) in zip(SUBS_I.chars(), SUBS_O.chars()) {
            if c == ci {
                return co;
            }
        }
        '-'
    };
}
