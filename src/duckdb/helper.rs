pub fn quote(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}
