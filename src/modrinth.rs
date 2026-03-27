pub fn is_valid_modrinth_slug(slug: &str) -> bool {
    let len_ok = (3..=64).contains(&slug.len());
    let chars_ok = slug
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || "!@$()`.+,\"-'".contains(c));

    len_ok && chars_ok
}
