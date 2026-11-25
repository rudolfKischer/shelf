#[derive(Debug, Clone, PartialEq)]
pub enum Vdf {
    Text(String),
    Map(Vec<(String, Vdf)>),
}

#[derive(Debug, PartialEq)]
pub enum VdfError {
    UnexpectedEnd,
    ExpectedQuote(usize),
}

const QUOTE: u8 = b'"';
const OPEN_BRACE: u8 = b'{';
const CLOSE_BRACE: u8 = b'}';
const BACKSLASH: u8 = b'\\';
const SLASH: u8 = b'/';

impl Vdf {
    pub fn get(&self, key: &str) -> Option<&Vdf> {
        self.entries()
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| value)
    }

    pub fn path(&self, keys: &[&str]) -> Option<&Vdf> {
        keys.iter().try_fold(self, |node, key| node.get(key))
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            Vdf::Text(value) => Some(value),
            Vdf::Map(_) => None,
        }
    }

    pub fn text_at(&self, keys: &[&str]) -> Option<&str> {
        self.path(keys).and_then(Vdf::text)
    }

    pub fn number_at(&self, keys: &[&str]) -> Option<u64> {
        self.text_at(keys).and_then(|value| value.parse().ok())
    }

    pub fn entries(&self) -> &[(String, Vdf)] {
        match self {
            Vdf::Map(entries) => entries,
            Vdf::Text(_) => &[],
        }
    }
}

pub fn parse(source: &str) -> Result<Vdf, VdfError> {
    let mut cursor = Cursor {
        bytes: source.as_bytes(),
        position: 0,
    };
    cursor.read_entries(false).map(Vdf::Map)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl Cursor<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn skip_blank(&mut self) {
        while let Some(byte) = self.peek() {
            if byte.is_ascii_whitespace() {
                self.position += 1;
            } else if self.at_line_comment() {
                self.skip_line();
            } else {
                return;
            }
        }
    }

    fn at_line_comment(&self) -> bool {
        self.peek() == Some(SLASH) && self.bytes.get(self.position + 1) == Some(&SLASH)
    }

    fn skip_line(&mut self) {
        while let Some(byte) = self.peek() {
            self.position += 1;
            if byte == b'\n' {
                return;
            }
        }
    }

    fn read_entries(&mut self, inside_block: bool) -> Result<Vec<(String, Vdf)>, VdfError> {
        let mut entries = Vec::new();
        loop {
            self.skip_blank();
            match self.peek() {
                None if inside_block => return Err(VdfError::UnexpectedEnd),
                None => return Ok(entries),
                Some(CLOSE_BRACE) => {
                    self.position += 1;
                    return Ok(entries);
                }
                Some(_) => entries.push(self.read_entry()?),
            }
        }
    }

    fn read_entry(&mut self) -> Result<(String, Vdf), VdfError> {
        let key = self.read_quoted()?;
        self.skip_blank();
        if self.peek() != Some(OPEN_BRACE) {
            return Ok((key, Vdf::Text(self.read_quoted()?)));
        }
        self.position += 1;
        Ok((key, Vdf::Map(self.read_entries(true)?)))
    }

    fn read_quoted(&mut self) -> Result<String, VdfError> {
        if self.peek() != Some(QUOTE) {
            return Err(VdfError::ExpectedQuote(self.position));
        }
        self.position += 1;
        let mut collected = Vec::new();
        loop {
            let byte = self.peek().ok_or(VdfError::UnexpectedEnd)?;
            self.position += 1;
            match byte {
                QUOTE => return Ok(String::from_utf8_lossy(&collected).into_owned()),
                BACKSLASH => collected.push(self.read_escaped()?),
                other => collected.push(other),
            }
        }
    }

    fn read_escaped(&mut self) -> Result<u8, VdfError> {
        let escaped = self.peek().ok_or(VdfError::UnexpectedEnd)?;
        self.position += 1;
        Ok(match escaped {
            b'n' => b'\n',
            b't' => b'\t',
            other => other,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../tests/fixtures/sample_manifest.acf");

    #[test]
    fn reads_nested_values() {
        let root = parse(SAMPLE).unwrap();
        assert_eq!(
            root.text_at(&["AppState", "name"]),
            Some("Hollow Knight: Silksong")
        );
        assert_eq!(
            root.number_at(&["AppState", "SizeOnDisk"]),
            Some(8260628780)
        );
        assert_eq!(
            root.text_at(&["AppState", "UserConfig", "language"]),
            Some("english")
        );
    }

    #[test]
    fn reports_unterminated_block() {
        assert_eq!(parse("\"a\" {"), Err(VdfError::UnexpectedEnd));
    }
}
