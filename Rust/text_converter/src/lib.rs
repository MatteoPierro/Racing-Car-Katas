// src/lib.rs
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};

pub struct HtmlTextConverter {
    full_filename_with_path: String,
}

impl HtmlTextConverter {
    pub fn new(full_filename_with_path: &str) -> Self {
        Self {
            full_filename_with_path: full_filename_with_path.to_string(),
        }
    }

    pub fn convert_to_html(&self) -> io::Result<String> {
        let lines = self.lines()?;

        let mut html = String::new();
        for line in lines {
            let line = line?;
            html.push_str(&escape_html(&line));
            html.push_str("<br />");
        }

        Ok(html)
    }

    fn lines<'a>(&'a self) -> io::Result<impl Iterator<Item=io::Result<String>> + 'a> {
        let file = File::open(&self.full_filename_with_path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Ok(lines)
    }

    pub fn get_filename(&self) -> &str {
        &self.full_filename_with_path
    }
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
    fn when_there_is_a_content() {
        let file_path = &format!("/tmp/{}.txt", Uuid::new_v4());

        fs::write(file_path, "first line\ninter>barcelona").unwrap();

        let converter = HtmlTextConverter::new(file_path);
        let converted = converter.convert_to_html().unwrap();

        assert_eq!("first line<br />inter&gt;barcelona<br />", converted);
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
