//! Companies Module
//!
//! Loads company and job data from config/companies.toml.
//! Provides access to all companies and their open positions.

use serde::Deserialize;
use std::str::FromStr;

use crate::jobs::{Company, CompanyTier, Job, SkillRequirement};
use crate::skills::Proficiency;

/// Job requirement configuration from TOML
#[derive(Debug, Clone, Deserialize)]
struct JobRequirementConfig {
    skill_name: String,
    min_proficiency: String,
    mandatory: bool,
    weight: f32,
}

/// Job configuration from TOML
#[derive(Debug, Clone, Deserialize)]
struct JobConfig {
    id: u32,
    title: String,
    salary_min: u32,
    salary_max: u32,
    min_experience_days: u32,
    description: String,
    difficulty: u8,
    requirements: Vec<JobRequirementConfig>,
}

/// Company configuration from TOML
#[derive(Debug, Clone, Deserialize)]
struct CompanyConfig {
    name: String,
    description: String,
    tier: String,
    jobs: Vec<JobConfig>,
}

/// Root config structure
#[derive(Debug, Clone, Deserialize)]
struct CompaniesConfig {
    companies: Vec<CompanyConfig>,
}

fn parse_proficiency(s: &str) -> Proficiency {
    Proficiency::from_str(s).unwrap_or(Proficiency::None)
}

fn parse_tier(s: &str) -> CompanyTier {
    match s {
        "Startup" => CompanyTier::Startup,
        "MidSize" => CompanyTier::MidSize,
        "BigTech" => CompanyTier::BigTech,
        "Faang" => CompanyTier::Faang,
        _ => CompanyTier::Startup,
    }
}

fn convert_job_config(job: JobConfig, company_name: &str) -> Job {
    Job {
        id: job.id,
        title: job.title,
        company: company_name.to_string(),
        salary_min: job.salary_min,
        salary_max: job.salary_max,
        requirements: job
            .requirements
            .into_iter()
            .map(|r| SkillRequirement {
                skill_name: r.skill_name,
                min_proficiency: parse_proficiency(&r.min_proficiency),
                mandatory: r.mandatory,
                weight: r.weight,
            })
            .collect(),
        min_experience_days: job.min_experience_days,
        description: job.description,
        difficulty: job.difficulty,
    }
}

/// Load all companies from config file
pub fn get_all_companies() -> Vec<Company> {
    const CONFIG: &str = include_str!("../config/companies.toml");
    let config: CompaniesConfig = toml::from_str(CONFIG).expect("Failed to parse companies.toml");

    config
        .companies
        .into_iter()
        .map(|c| Company {
            name: c.name.clone(),
            description: c.description,
            tier: parse_tier(&c.tier),
            open_positions: c
                .jobs
                .into_iter()
                .map(|j| convert_job_config(j, &c.name))
                .collect(),
        })
        .collect()
}
