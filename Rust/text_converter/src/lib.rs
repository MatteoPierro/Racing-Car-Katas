// src/lib.rs
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
pub struct HtmlTextConverter<Reader: LinesReader> {
    reader: Reader,
}

impl<Reader: LinesReader> HtmlTextConverter<Reader> {
    pub fn new(lines_reader: Reader) -> Self {
        Self {
            reader: lines_reader,
        }
    }

    pub fn convert_to_html(&self) -> io::Result<String> {
        let mut html = String::new();

        for line in self.reader.lines()? {
            let line = line?;
            html.push_str(&escape_html(&line));
            html.push_str("<br />");
        }

        Ok(html)
    }
}

struct FileLinesReader {
    path: String,
}

impl LinesReader for FileLinesReader {
    fn lines<'a>(&'a self) -> io::Result<impl Iterator<Item = io::Result<String>> + 'a> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Ok(lines)
    }
}

pub trait LinesReader {
    fn lines<'a>(&'a self) -> io::Result<impl Iterator<Item = io::Result<String>> + 'a>;
}

fn escape_html(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use uuid::Uuid;

    #[test]
    fn when_using_a_real_file_lines_reader() {
        let file_path = &format!("/tmp/{}.txt", Uuid::new_v4());

        fs::write(file_path, "first line\ninter>barcelona").unwrap();

        let converter = HtmlTextConverter::new(FileLinesReader {
            path: file_path.to_string(),
        });
        let converted = converter.convert_to_html().unwrap();

        assert_eq!("first line<br />inter&gt;barcelona<br />", converted);
    }

    #[test]
    fn convert_a_single_line() {
        let reader = FakeLinesReader(vec!["single line".to_string()]);

        let converter = HtmlTextConverter::new(reader);
        let converted = converter.convert_to_html().unwrap();

        assert_eq!("single line<br />", converted);
    }

    struct FakeLinesReader(Vec<String>);
    impl LinesReader for FakeLinesReader {
        fn lines<'a>(&'a self) -> io::Result<impl Iterator<Item = io::Result<String>> + 'a> {
            Ok(self.0.iter().map(|s| Ok(s.clone())))
        }
    }

    #[test]
    #[ignore]
    fn test_html_pages_converter() {
        let converter = HtmlPagesConverter::new("foo.txt").unwrap();
        assert_eq!("foo.txt", converter.get_filename());
    }
}

pub struct HtmlPagesConverter {
    filename: String,
    breaks: Vec<u64>,
}

impl HtmlPagesConverter {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut breaks = vec![0];
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut cumulative_char_count = 0;
        for line in reader.lines() {
            let line = line?;
            cumulative_char_count += line.len() + 1;
            if line.contains("PAGE_BREAK") {
                breaks.push(cumulative_char_count as u64);
            }
        }

        Ok(Self {
            filename: filename.to_string(),
            breaks,
        })
    }

    pub fn get_html_page(&self, page: usize) -> io::Result<String> {
        let file = File::open(&self.filename)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(self.breaks[page]))?;
        let mut html_page = String::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains("PAGE_BREAK") {
                break;
            }
            html_page.push_str(&escape_html(&line));
            html_page.push_str("<br />");
        }

        Ok(html_page)
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
}
