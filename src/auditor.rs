use regex::Regex;
use crate::parser::ParsedSkill;

#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub category: &'static str,
    pub severity: &'static str,
    pub message: String,
    pub deduction: u32,
}

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub total_score: u32,
    pub letter_grade: &'static str,
    pub token_count: usize,
    pub estimated_cost_per_1k: f64,
    pub findings: Vec<AuditFinding>,
}

pub struct Auditor {
    filler_regex: Regex,
    unquoted_var_regex: Regex,
    path_traversal_regex: Regex,
}

impl Auditor {
    pub fn new() -> Self {
        Self {
            filler_regex: Regex::new(r"(?i)\b(please ensure that you|kindly remember to|as an ai assistant|it is important to note that|feel free to|in order to achieve this)\b").unwrap(),
            unquoted_var_regex: Regex::new(r#"(bash|sh|run_command).*?(\$[a-zA-Z0-9_]+[^\s"'])"#).unwrap(),
            path_traversal_regex: Regex::new(r"\.\./\.\./").unwrap(),
        }
    }

    pub fn audit(&self, skill: &ParsedSkill) -> AuditReport {
        let mut score = 100u32;
        let mut findings = Vec::new();

        // 1. Structure Verification (25 points max)
        if !skill.has_frontmatter {
            findings.push(AuditFinding {
                category: "Structure",
                severity: "ERROR",
                message: "Missing YAML frontmatter (---) metadata.".to_string(),
                deduction: 15,
            });
            score = score.saturating_sub(15);
        } else {
            if skill.frontmatter.name.is_none() {
                findings.push(AuditFinding {
                    category: "Structure",
                    severity: "ERROR",
                    message: "Missing 'name' in YAML frontmatter.".to_string(),
                    deduction: 10,
                });
                score = score.saturating_sub(10);
            }
            if skill.frontmatter.description.is_none() {
                findings.push(AuditFinding {
                    category: "Structure",
                    severity: "WARN",
                    message: "Missing 'description' in YAML frontmatter.".to_string(),
                    deduction: 5,
                });
                score = score.saturating_sub(5);
            }
        }

        // 2. Security & Injection Checks (35 points max)
        if self.path_traversal_regex.is_match(&skill.body) {
            findings.push(AuditFinding {
                category: "Security",
                severity: "HIGH",
                message: "Potential path traversal sequence ('../../') detected.".to_string(),
                deduction: 20,
            });
            score = score.saturating_sub(20);
        }

        if self.unquoted_var_regex.is_match(&skill.body) {
            findings.push(AuditFinding {
                category: "Security",
                severity: "WARN",
                message: "Unquoted shell variable expansion detected in commands (risk of argument injection).".to_string(),
                deduction: 10,
            });
            score = score.saturating_sub(10);
        }

        // 3. Token Efficiency & Conversational Bloat (20 points max)
        let words = skill.body.split_whitespace().count();
        let est_tokens = (words as f64 * 1.3) as usize;

        let filler_matches = self.filler_regex.find_iter(&skill.body).count();
        if filler_matches > 0 {
            let ded = (filler_matches as u32 * 3).min(15);
            findings.push(AuditFinding {
                category: "Efficiency",
                severity: "WARN",
                message: format!("Found {} conversational filler phrases. Run `skillcheck fix` to prune.", filler_matches),
                deduction: ded,
            });
            score = score.saturating_sub(ded);
        }

        if est_tokens > 2500 {
            findings.push(AuditFinding {
                category: "Efficiency",
                severity: "WARN",
                message: format!("Skill body is unusually large ({} estimated tokens). Consider splitting.", est_tokens),
                deduction: 10,
            });
            score = score.saturating_sub(10);
        }

        // 4. Clarity & Contradiction (20 points max)
        let lower = skill.body.to_lowercase();
        if lower.contains("always") && lower.contains("never") && lower.contains("commit") {
            if lower.contains("always commit") && lower.contains("never commit") {
                findings.push(AuditFinding {
                    category: "Clarity",
                    severity: "HIGH",
                    message: "Contradictory directives detected ('always commit' vs 'never commit').".to_string(),
                    deduction: 15,
                });
                score = score.saturating_sub(15);
            }
        }

        let letter_grade = match score {
            95..=100 => "A+",
            90..=94 => "A",
            80..=89 => "B",
            70..=79 => "C",
            60..=69 => "D",
            _ => "F",
        };

        // Estimated cost per 1,000 invocations ($3.00 / 1M tokens)
        let est_cost_1k = (est_tokens as f64 * 1000.0) / 1_000_000.0 * 3.00;

        AuditReport {
            total_score: score,
            letter_grade,
            token_count: est_tokens,
            estimated_cost_per_1k: est_cost_1k,
            findings,
        }
    }
}
