mod auditor;
mod badge;
mod optimizer;
mod parser;

use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process;

use auditor::Auditor;
use badge::BadgeGenerator;
use optimizer::SkillOptimizer;
use parser::SkillParser;

#[derive(Parser)]
#[command(name = "skillcheck")]
#[command(about = "Quality linter for AI agent skills and prompt manifests.")]
#[command(version = "0.1.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit a SKILL.md file
    Scan {
        /// Target path to SKILL.md
        #[arg(default_value = "SKILL.md")]
        path: PathBuf,
    },

    /// Auto-optimize skill prompt to compress token bloat
    Fix {
        /// Target path to SKILL.md
        #[arg(default_value = "SKILL.md")]
        path: PathBuf,

        /// Overwrite file in place
        #[arg(short, long)]
        write: bool,
    },

    /// Generate Markdown / SVG badge for repository README
    Badge {
        /// Target path to SKILL.md
        #[arg(default_value = "SKILL.md")]
        path: PathBuf,
    },

    /// Print version information
    Version,
}

fn main() {
    let cli = Cli::parse();
    let auditor = Auditor::new();

    match cli.command {
        Commands::Scan { path } => {
            if !path.exists() {
                eprintln!("{}", format!("Error: File '{}' not found.", path.display()).red());
                process::exit(1);
            }

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", format!("Error reading '{}': {}", path.display(), e).red());
                    process::exit(1);
                }
            };

            let parsed = SkillParser::parse(&content);
            let report = auditor.audit(&parsed);

            println!("{}", "╔═════════════════════════════════════════════════════════════════╗".cyan());
            println!("{}", "║      skillcheck v0.1.1 — Agent Skill & Prompt Quality Audit     ║".cyan().bold());
            println!("{}", "╚═════════════════════════════════════════════════════════════════╝".cyan());
            println!(" File: {}", path.display().to_string().bold());
            println!(" Estimated Tokens: {}", report.token_count.to_string().yellow());
            println!(" Projected Cost (1k calls): ${:.4}", report.estimated_cost_per_1k);
            println!();

            if report.findings.is_empty() {
                println!("{}", "  ✔ No findings from the configured heuristic checks.".green().bold());
            } else {
                println!(" Findings ({}):", report.findings.len());
                for f in &report.findings {
                    let sev_colored = match f.severity {
                        "ERROR" | "HIGH" => f.severity.red().bold(),
                        _ => f.severity.yellow().bold(),
                    };
                    println!("  [{}] [{}] {} (-{} pts)", sev_colored, f.category.cyan(), f.message, f.deduction);
                }
            }

            println!();
            let score_colored = if report.total_score >= 80 {
                report.total_score.to_string().green().bold()
            } else if report.total_score >= 60 {
                report.total_score.to_string().yellow().bold()
            } else {
                report.total_score.to_string().red().bold()
            };

            println!(" Overall Score: {} / 100 [Grade: {}]", score_colored, report.letter_grade.bold());
            let badge = BadgeGenerator::markdown_badge(report.total_score, report.letter_grade);
            println!(" README Badge:  {}", badge.dimmed());

            if report.total_score < 70 {
                process::exit(1);
            }
        }

        Commands::Fix { path, write } => {
            if !path.exists() {
                eprintln!("{}", format!("Error: File '{}' not found.", path.display()).red());
                process::exit(1);
            }

            let content = fs::read_to_string(&path).unwrap_or_else(|error| {
                eprintln!("Error reading '{}': {}", path.display(), error);
                process::exit(1);
            });
            let optimizer = SkillOptimizer::new();
            let (optimized, saved_words) = optimizer.optimize(&content);

            println!(
                "{}",
                format!("✔ Pruned ~{} redundant conversational words (~{} tokens saved)", saved_words, (saved_words as f64 * 1.3) as usize).green().bold()
            );

            if write {
                fs::write(&path, &optimized).unwrap_or_else(|error| {
                    eprintln!("Error writing '{}': {}", path.display(), error);
                    process::exit(1);
                });
                println!("✔ Wrote optimized content in-place to '{}'", path.display());
            } else {
                println!("\nPreview of changes (pass --write to apply):\n");
                println!("{}", optimized);
            }
        }

        Commands::Badge { path } => {
            let content = fs::read_to_string(&path).unwrap_or_else(|error| {
                eprintln!("Error reading '{}': {}", path.display(), error);
                process::exit(1);
            });
            let parsed = SkillParser::parse(&content);
            let report = auditor.audit(&parsed);
            println!("{}", BadgeGenerator::markdown_badge(report.total_score, report.letter_grade));
        }

        Commands::Version => {
            println!("skillcheck v0.1.1 (Lighthouse for AI Agent Skills)");
        }
    }
}
