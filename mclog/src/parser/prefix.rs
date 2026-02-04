#[derive(Debug, PartialEq)]
pub enum State {
    MatchingLevel,
    LevelFound,
    CollectingData,
    SkippingWhitespace,
    End,
}

pub struct LevelSuffixParser {
    pub state: State,
    pub target_level: Vec<char>,
    pub match_index: usize,
    pub buffer: String,
    pub found: bool,
}

impl LevelSuffixParser {
    pub fn new(level: &str) -> Self {
        Self {
            state: State::MatchingLevel,
            target_level: level.chars().collect(),
            match_index: 0,
            buffer: String::new(),
            found: false,
        }
    }

    pub fn next_char(&mut self, ch: char) {
        match self.state {
            State::MatchingLevel => {
                if self.target_level.get(self.match_index) == Some(&ch) {
                    self.match_index += 1;
                    if self.match_index == self.target_level.len() {
                        self.state = State::LevelFound;
                        self.found = true;
                    }
                } else {
                    self.match_index = 0;
                    if self.target_level.get(0) == Some(&ch) {
                        self.match_index = 1;
                    }
                }
            }
            State::LevelFound => {
                if ch.is_whitespace() {
                    self.state = State::SkippingWhitespace;
                } else {
                    self.buffer.push(ch);
                    self.state = State::CollectingData;
                }
            }
            State::CollectingData => {
                if ch.is_whitespace() {
                    self.state = State::SkippingWhitespace;
                } else {
                    self.buffer.push(ch);
                }
            }
            State::SkippingWhitespace => {
                if !ch.is_whitespace() {
                    self.state = State::End;
                }
            }
            State::End => {}
        }
    }

    pub fn get_result(&self) -> Option<String> {
        if self.found {
            Some(self.buffer.clone())
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.state = State::MatchingLevel;
        self.match_index = 0;
        self.buffer.clear();
        self.found = false;
    }
}
