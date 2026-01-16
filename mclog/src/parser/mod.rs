#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Custom,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LogEntry<'a> {
    pub log_level: Option<LogLevel>,
    pub prefix: Option<&'a str>,
    pub message: &'a str,
}
pub struct Parser<'a> {
    lines: &'a [&'a str],
}

impl<'a> Parser<'a> {
    pub fn new(lines: &'a [&'a str]) -> Self {
        Self { lines }
    }

    pub fn parse(&self) -> Vec<LogEntry<'a>> {
        let lines = self.lines;

        let mut entries = Vec::with_capacity(lines.len());
        
        let mut current_state: Option<LogLevel> = None;
    
        for &line_str in lines {
            let line = Line(line_str);
            
            if let Some(level) = line.log_level() {
                current_state = Some(level);
            }
    
            entries.push(LogEntry {
                log_level: current_state,
                prefix: line.prefix(),
                message: line.message(),
            });
        }
    
        entries
    }

    pub fn html(&self) -> Vec<String> {
        let mut html: Vec<String> = vec![];

        let entries = self.parse();

        let mut id: usize = 1;

        for entry in entries {
            if entry.prefix.is_none_or(|p| p.is_empty()) && entry.message.is_empty() { continue; }

            let log_level = match entry.log_level {
                Some(LogLevel::Info) => "i",
                Some(LogLevel::Warn) => "w",
                Some(LogLevel::Error) => "e",
                Some(LogLevel::Custom) => "c",
                None => "",
            };

            let html_part = match entry.log_level {
                Some(LogLevel::Info) => format!(
                    r#"<span class="p" id="L{}"><span class={}>{}</span>{}</span>"#,
                    id,
                    log_level,
                    html_escape::encode_text(entry.prefix.unwrap_or_default()),
                    html_escape::encode_text(entry.message),
                ),
                Some(LogLevel::Warn) | Some(LogLevel::Error) |  Some(LogLevel::Custom) | None => format!(
                    r#"<span class="p" id="L{}"><span class="{}">{}{}</span></span>"#,
                    id,
                    log_level,
                    html_escape::encode_text(entry.prefix.unwrap_or_default()),
                    html_escape::encode_text(entry.message),
                ),
            };
            html.push(html_part);
            id += 1;
        }

        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LogLevel::*;

    fn get_entry_levels(log: &[&'static str]) -> Vec<Option<LogLevel>> {
        let levels: Vec<Option<LogLevel>> = Parser::new(&log)
            .parse()
            .into_iter()
            .map(|entry| entry.log_level)
            .collect();

        levels
    }

    #[test]
    fn empty_log() {
        assert!(get_entry_levels(&[]).is_empty());
    }

    #[test]
    fn log_starts_with_unknown_level() {
        let log = ["a", "b", "INFO] c", "d", "ERROR] e"];

        let expected_levels = vec![None, None, Some(Info), Some(Info), Some(Error)];

        assert_eq!(get_entry_levels(&log), expected_levels);
    }

    #[test]
    fn log_starts_with_known_level() {
        let log = ["INFO] a", "b", "c", "ERROR] d"];

        let expected_levels = vec![Some(Info), Some(Info), Some(Info), Some(Error)];

        assert_eq!(get_entry_levels(&log), expected_levels);
    }

    #[test]
    fn log_ends_with_unknown_level() {
        let log = ["INFO] a", "b", "ERROR] c", "d"];

        let expected_levels = vec![Some(Info), Some(Info), Some(Error), Some(Error)];

        assert_eq!(get_entry_levels(&log), expected_levels);
    }

    #[test]
    fn log_ends_with_known_level() {
        let log = ["INFO] a", "b", "ERROR] c", "WARN] d"];

        let expected_levels = vec![Some(Info), Some(Info), Some(Error), Some(Warn)];

        assert_eq!(get_entry_levels(&log), expected_levels);
    }
}

pub struct Line<'a>(pub &'a str);

impl<'a> Line<'a> {
    fn log_level(&self) -> Option<LogLevel> {
        if self.0.contains("INFO]") {
            return Some(LogLevel::Info);
        } else if self.0.contains("WARN]") {
            return Some(LogLevel::Warn);
        } else if self.0.contains("ERROR]") {
            return Some(LogLevel::Error);
        }
        None
    }

    fn is_stacktrace_line(&self) -> bool {
        let trimmed = self.0.trim_start();

        !trimmed.is_empty() && trimmed.len() != self.0.len() && trimmed.starts_with("at ")
    }

    fn prefix(&self) -> Option<&'a str> {
        let index = self.0.find("INFO]:")? + "INFO]:".len();
        Some(&self.0[..index])
    }

    fn message(&self) -> &'a str {
        self.0
            .split_once("INFO]:")
            .map(|(_, msg)| msg)
            .unwrap_or(self.0)
    }
}

/// Is Stacktrace if `Line::log_level` returns `LogLevel::Error` ∧
/// >= 3 following lines starts with `\s+at `
/// Stacktrace ends with line starting with non-whitespace character
/// Returns Vec of start and end indexes of stacktraces, starting from index 0
pub fn get_stacktraces(lines: &[Line]) -> Vec<(usize, usize)> {
    let mut stacktraces = vec![];

    for (index, line) in lines.iter().enumerate() {
        if line.log_level() == Some(LogLevel::Error) {
            let mut i = 0;

            loop {
                let next_line = lines.get(index + i + 1);

                if next_line.is_none() || next_line.is_some_and(|line| !line.is_stacktrace_line()) {
                    break;
                }
                i += 1;
            }

            if i >= 3 {
                let start_idx = index;
                let end_idx = index + i;
                stacktraces.push((start_idx, end_idx));
            }
        }
    }
    stacktraces
}

pub fn get_jars(line: String) -> Vec<(usize, usize)> {
    let mut jars = vec![];
    let mut in_brackets = false;

    for (i, ch) in line.char_indices() {
        if ch == '~' && line.get(i + 1..i + 2) == Some("[") {
            in_brackets = true;
            continue;
        }

        if in_brackets {
            if ch == ']' {
                in_brackets = false;
            }
            continue;
        }

        if ch == '.' && line.get(i..i + 4) == Some(".jar") {
            let start = line[..i]
                .rfind(|sep: char| {
                    sep == ' ' || sep == '[' || sep == '/' || sep == '(' || sep == ':'
                })
                .map(|idx| {
                    let sep_char = line[idx..].chars().next().unwrap();
                    idx + sep_char.len_utf8()
                })
                .unwrap_or(0);

            let end = i + 4;
            jars.push((start, end));
        }
    }
    jars
}
