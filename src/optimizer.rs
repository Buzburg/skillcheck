use regex::Regex;

pub struct SkillOptimizer {
    replacements: Vec<(Regex, &'static str)>,
}

impl SkillOptimizer {
    pub fn new() -> Self {
        Self {
            replacements: vec![
                (Regex::new(r"(?i)please ensure that you\s+").unwrap(), ""),
                (Regex::new(r"(?i)kindly remember to\s+").unwrap(), ""),
                (Regex::new(r"(?i)it is important to note that\s+").unwrap(), "Note: "),
                (Regex::new(r"(?i)as an ai assistant,\s*").unwrap(), ""),
                (Regex::new(r"(?i)in order to achieve this,\s*").unwrap(), "To do this, "),
                (Regex::new(r"(?i)feel free to\s+").unwrap(), ""),
            ],
        }
    }

    pub fn optimize(&self, content: &str) -> (String, usize) {
        let original_words = content.split_whitespace().count();
        let mut optimized = content.to_string();

        for (re, rep) in &self.replacements {
            optimized = re.replace_all(&optimized, *rep).to_string();
        }

        let new_words = optimized.split_whitespace().count();
        let saved_words = original_words.saturating_sub(new_words);

        (optimized, saved_words)
    }
}
