use self_update::cargo_crate_version;

pub fn is_valid_modrinth_slug(slug: &str) -> bool {
    let len_ok = (3..=64).contains(&slug.len());
    let chars_ok = slug
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || "!@$()`.+,\"-'".contains(c));

    len_ok && chars_ok
}

pub fn create_ferinth() -> ferinth::Ferinth<()> {
    ferinth::Ferinth::<()>::new(
        env!("CARGO_CRATE_NAME"),
        Some(cargo_crate_version!()),
        Some("hello@gurkz.me / https://github.com/thatgurkangurk/are-we-there-yet"),
    )
}
