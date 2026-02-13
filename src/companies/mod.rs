use crate::jobs::{Company, CompanyTier, Job, SkillRequirement};
use crate::skills::Proficiency;

pub fn get_all_companies() -> Vec<Company> {
    vec![
        Company {
            name: "DataStartup AI".to_string(),
            description: "Fast-growing AI startup focused on NLP solutions".to_string(),
            tier: CompanyTier::Startup,
            open_positions: vec![
                Job {
                    id: 1,
                    title: "Junior ML Engineer".to_string(),
                    company: "DataStartup AI".to_string(),
                    salary_min: 80000,
                    salary_max: 110000,
                    requirements: vec![
                        SkillRequirement {
                            skill_name: "Python".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "PyTorch".to_string(),
                            min_proficiency: Proficiency::Basic,
                            mandatory: true,
                            weight: 0.8,
                        },
                    ],
                    min_experience_days: 0,
                    description: "Build ML models for NLP tasks".to_string(),
                    difficulty: 1,
                },
            ],
        },
        Company {
            name: "TechCorp Inc".to_string(),
            description: "Established tech company with ML division".to_string(),
            tier: CompanyTier::MidSize,
            open_positions: vec![
                Job {
                    id: 2,
                    title: "ML Engineer".to_string(),
                    company: "TechCorp Inc".to_string(),
                    salary_min: 120000,
                    salary_max: 160000,
                    requirements: vec![
                        SkillRequirement {
                            skill_name: "Python".to_string(),
                            min_proficiency: Proficiency::Advanced,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "PyTorch".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "SQL".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: false,
                            weight: 0.5,
                        },
                        SkillRequirement {
                            skill_name: "Statistics".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: true,
                            weight: 0.7,
                        },
                    ],
                    min_experience_days: 90,
                    description: "Develop and deploy ML models at scale".to_string(),
                    difficulty: 2,
                },
            ],
        },
        Company {
            name: "MegaTech".to_string(),
            description: "Large tech company with massive ML infrastructure".to_string(),
            tier: CompanyTier::BigTech,
            open_positions: vec![
                Job {
                    id: 3,
                    title: "Senior AI Engineer".to_string(),
                    company: "MegaTech".to_string(),
                    salary_min: 180000,
                    salary_max: 250000,
                    requirements: vec![
                        SkillRequirement {
                            skill_name: "Python".to_string(),
                            min_proficiency: Proficiency::Expert,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "Transformers".to_string(),
                            min_proficiency: Proficiency::Advanced,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "LLM Fine-tuning".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: true,
                            weight: 0.8,
                        },
                        SkillRequirement {
                            skill_name: "System Design".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: true,
                            weight: 0.7,
                        },
                        SkillRequirement {
                            skill_name: "Communication".to_string(),
                            min_proficiency: Proficiency::Intermediate,
                            mandatory: false,
                            weight: 0.5,
                        },
                    ],
                    min_experience_days: 180,
                    description: "Lead AI projects and mentor junior engineers".to_string(),
                    difficulty: 3,
                },
            ],
        },
        Company {
            name: "SearchGiant".to_string(),
            description: "World's largest search and AI company".to_string(),
            tier: CompanyTier::Faang,
            open_positions: vec![
                Job {
                    id: 4,
                    title: "Staff LLM Engineer".to_string(),
                    company: "SearchGiant".to_string(),
                    salary_min: 280000,
                    salary_max: 400000,
                    requirements: vec![
                        SkillRequirement {
                            skill_name: "Python".to_string(),
                            min_proficiency: Proficiency::Expert,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "Transformers".to_string(),
                            min_proficiency: Proficiency::Expert,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "LLM Fine-tuning".to_string(),
                            min_proficiency: Proficiency::Advanced,
                            mandatory: true,
                            weight: 1.0,
                        },
                        SkillRequirement {
                            skill_name: "RAG".to_string(),
                            min_proficiency: Proficiency::Advanced,
                            mandatory: true,
                            weight: 0.8,
                        },
                        SkillRequirement {
                            skill_name: "System Design".to_string(),
                            min_proficiency: Proficiency::Advanced,
                            mandatory: true,
                            weight: 0.9,
                        },
                    ],
                    min_experience_days: 365,
                    description: "Architect next-generation LLM systems".to_string(),
                    difficulty: 4,
                },
            ],
        },
    ]
}
