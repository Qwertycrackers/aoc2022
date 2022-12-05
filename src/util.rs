
pub fn case_path(case: &str) -> String {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/{}/{}", cargo_dir, "cases", case)
}
