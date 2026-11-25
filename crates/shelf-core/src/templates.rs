pub fn fill(template: &str, values: &[(&str, &str)]) -> String {
    values
        .iter()
        .fold(template.to_string(), |text, (name, value)| {
            text.replace(&format!("{{{name}}}"), value)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_every_named_placeholder() {
        let url = fill("x://{kind}/{id}?t={id}", &[("kind", "play"), ("id", "7")]);
        assert_eq!(url, "x://play/7?t=7");
    }
}
