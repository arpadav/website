// --------------------------------------------------
// external
// --------------------------------------------------
use std::path::PathBuf;

#[derive(Clone, Debug)]
/// A small CSV file wrapper for site-generation data
pub struct Csv<'a> {
    /// Path to the CSV file on disk
    path: PathBuf,
    /// Contents
    contents: String,
    /// Expected header row
    headers: Vec<&'a str>,
}
/// [`Csv`] implementation
impl<'a> Csv<'a> {
    /// Creates a CSV wrapper with an expected header row
    pub fn new(
        path: impl Into<PathBuf>,
        headers: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, String> {
        let path = path.into();
        let contents = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to open `{}`: {}", path.display(), e))?;
        Ok(Self {
            path,
            contents,
            headers: headers.into_iter().collect()
        })
    }

    /// Reads all non-header rows, validating the header and row widths
    pub fn read_rows(&self) -> Result<Vec<CsvRow>, String> {
        // --------------------------------------------------
        // parse lines
        // --------------------------------------------------
        let mut rows = self.contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(Self::parse_line)
            .collect::<Vec<_>>();
        // --------------------------------------------------
        // empty csv
        // --------------------------------------------------
        if rows.is_empty() {
            return Ok(Vec::new());
        }
        // --------------------------------------------------
        // remove headers, check if match?
        // --------------------------------------------------
        let headers = rows.remove(0);
        if headers != self.headers {
            return Err(format!(
                "`{}` has headers {:?}, expected {:?}",
                self.path.display(),
                headers,
                self.headers
            ));
        }
        // --------------------------------------------------
        // verify row len matches header len, make CsvRow of
        // each row
        // --------------------------------------------------
        rows.into_iter()
            .enumerate()
            .map(|(i, row)| {
                if row.len() != self.headers.len() {
                    return Err(format!(
                        "`{}` row {} has {} fields, expected {}",
                        self.path.display(),
                        i + 2,
                        row.len(),
                        self.headers.len()
                    ));
                }
                Ok(CsvRow::new(row))
            })
            .collect()
    }

    /// Parses one CSV line, tolerating malformed quoting
    fn parse_line(line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut field = String::new();
        let mut chars = line.chars().peekable();
        let mut in_quotes = false;
        while let Some(c) = chars.next() {
            match (in_quotes, c) {
                // --------------------------------------------------
                // "" inside a quoted field -> a literal quote
                // --------------------------------------------------
                (true, '"') if chars.peek() == Some(&'"') => {
                    field.push('"');
                    chars.next();
                }
                // --------------------------------------------------
                // closing quote
                // --------------------------------------------------
                (true, '"') => in_quotes = false,
                // --------------------------------------------------
                // literal inside quotes
                // --------------------------------------------------
                (true, _) => field.push(c),
                // --------------------------------------------------
                // opening quote
                // --------------------------------------------------
                (false, '"') => in_quotes = true,
                // --------------------------------------------------
                // consume/reset field, add to fields
                // --------------------------------------------------
                (false, ',') => fields.push(std::mem::take(&mut field)),
                (false, _) => field.push(c),
            }
        }
        fields.push(field);
        fields
    }
}

#[derive(Clone, Debug)]
/// One parsed CSV row
pub struct CsvRow {
    /// Parsed fields
    fields: Vec<String>,
}
/// [`CsvRow`] implementation
impl CsvRow {
    /// Creates a CSV row from already-validated fields
    fn new(fields: Vec<String>) -> Self {
        Self { fields }
    }

    /// Consumes the row and returns exactly `N` fields
    pub fn into_array<const N: usize>(self, name: &str) -> Result<[String; N], String> {
        self.fields.try_into().map_err(|fields: Vec<String>| {
            format!("{name} has {} fields, expected {N}", fields.len())
        })
    }
}
