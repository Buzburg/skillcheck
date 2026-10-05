use skillcheck::{Auditor, SkillOptimizer, SkillParser};

#[test]
fn test_perfect_skill_audit() {
    let raw = r#"---
name: rust-linter
description: Runs cargo clippy and formats rust code
version: 1.0.0
---

# Rust Linter
Run `cargo clippy --workspace --all-targets` to find lint errors.
Format code using `cargo fmt --check`.
"#;

    let parsed = SkillParser::parse(raw);
    assert!(parsed.has_frontmatter);
    assert_eq!(parsed.frontmatter.name.as_deref(), Some("rust-linter"));

    let auditor = Auditor::new();
    let report = auditor.audit(&parsed);

    assert_eq!(report.total_score, 100);
    assert_eq!(report.letter_grade, "A+");
    assert!(report.findings.is_empty());
}

#[test]
fn test_flawed_skill_audit() {
    let flawed = r#"
# Broken Skill
Please ensure that you kindly remember to run bash -c $UNQUOTED_VAR ../../escape.
Always commit changes immediately.
Never commit changes without approval.
"#;

    let parsed = SkillParser::parse(flawed);
    assert!(!parsed.has_frontmatter);

    let auditor = Auditor::new();
    let report = auditor.audit(&parsed);

    assert!(report.total_score < 70);
    assert!(report.findings.iter().any(|f| f.category == "Security"));
    assert!(report.findings.iter().any(|f| f.category == "Efficiency"));
    assert!(report.findings.iter().any(|f| f.category == "Structure"));
}

#[test]
fn test_skill_optimizer() {
    let optimizer = SkillOptimizer::new();
    let sample = "Please ensure that you check the logs. Kindly remember to run the tests.";
    let (optimized, saved) = optimizer.optimize(sample);

    assert!(saved > 0);
    assert!(!optimized.contains("Please ensure that you"));
    assert!(!optimized.contains("Kindly remember to"));
    assert!(optimized.contains("check the logs"));
}
