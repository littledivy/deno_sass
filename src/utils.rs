pub fn str_to_style(style: &str) -> sass_rs::OutputStyle {
    match style {
        "nested" => sass_rs::OutputStyle::Nested,
        "expanded" => sass_rs::OutputStyle::Expanded,
        "compact" => sass_rs::OutputStyle::Compact,
        "compressed" => sass_rs::OutputStyle::Compressed,
        _ => sass_rs::OutputStyle::Nested,
    }
}