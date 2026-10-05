pub struct BadgeGenerator;

impl BadgeGenerator {
    pub fn markdown_badge(score: u32, grade: &str) -> String {
        let color = match score {
            90..=100 => "brightgreen",
            80..=89 => "green",
            70..=79 => "yellow",
            60..=69 => "orange",
            _ => "red",
        };

        format!(
            "[![SkillScore](https://img.shields.io/badge/SkillScore-{}/100_{}_{}.svg)](https://github.com/buzburgai-arch/skillcheck)",
            score, grade, color
        )
    }
}
