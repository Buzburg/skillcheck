use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedSkill {
    pub frontmatter: SkillFrontmatter,
    pub has_frontmatter: bool,
    pub body: String,
    #[allow(dead_code)]
    pub total_lines: usize,
}

pub struct SkillParser;

impl SkillParser {
    pub fn parse(content: &str) -> ParsedSkill {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        if content.starts_with("---") {
            // Find closing ---
            if let Some(close_idx) = lines[1..].iter().position(|l| l.trim() == "---") {
                let fm_raw = lines[1..=close_idx].join("\n");
                let body = lines[close_idx + 2..].join("\n");

                let frontmatter: SkillFrontmatter = serde_yaml::from_str(&fm_raw).unwrap_or_default();
                return ParsedSkill {
                    frontmatter,
                    has_frontmatter: true,
                    body,
                    total_lines,
                };
            }
        }

        ParsedSkill {
            frontmatter: SkillFrontmatter::default(),
            has_frontmatter: false,
            body: content.to_string(),
            total_lines,
        }
    }
}
