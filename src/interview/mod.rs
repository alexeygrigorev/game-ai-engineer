use rand::Rng;

use crate::jobs::Job;
use crate::player::Player;
use crate::skills::Proficiency;

#[derive(Debug, Clone)]
pub enum QuestionType {
    Technical,
    Behavioral,
    SystemDesign,
    Coding,
}

#[derive(Debug, Clone)]
pub struct InterviewQuestion {
    pub question: String,
    pub question_type: QuestionType,
    pub related_skill: String,
    pub difficulty: u8,
}

#[derive(Debug, Clone)]
pub struct InterviewRound {
    pub name: String,
    pub questions: Vec<InterviewQuestion>,
    pub pass_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct InterviewResult {
    pub round_name: String,
    pub score: f32,
    pub passed: bool,
    pub feedback: Vec<String>,
}

pub struct Interview;

impl Interview {
    pub fn generate_rounds(job: &Job) -> Vec<InterviewRound> {
        let mut rounds = vec![Self::screening_round()];
        
        for req in &job.requirements {
            if req.mandatory && req.min_proficiency >= Proficiency::Intermediate {
                rounds.push(Self::technical_round(&req.skill_name, req.min_proficiency));
            }
        }

        if job.difficulty >= 2 {
            rounds.push(Self::system_design_round());
        }

        if job.difficulty >= 3 {
            rounds.push(Self::behavioral_round());
        }

        rounds
    }

    fn screening_round() -> InterviewRound {
        InterviewRound {
            name: "HR Screening".to_string(),
            questions: vec![
                InterviewQuestion {
                    question: "Tell me about yourself and your experience with AI/ML".to_string(),
                    question_type: QuestionType::Behavioral,
                    related_skill: "Communication".to_string(),
                    difficulty: 1,
                },
                InterviewQuestion {
                    question: "Why are you interested in this role?".to_string(),
                    question_type: QuestionType::Behavioral,
                    related_skill: "Communication".to_string(),
                    difficulty: 1,
                },
            ],
            pass_threshold: 0.5,
        }
    }

    fn technical_round(skill: &str, proficiency: Proficiency) -> InterviewRound {
        let questions = match skill {
            "Python" => vec![
                InterviewQuestion {
                    question: "Explain the difference between list and tuple in Python".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 1,
                },
                InterviewQuestion {
                    question: "What are decorators? Give an example of when you'd use one".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "Explain Python's GIL and its implications for ML workloads".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 3,
                },
            ],
            "PyTorch" | "TensorFlow" => vec![
                InterviewQuestion {
                    question: "Explain the difference between eager and graph execution".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "How would you debug a model that's not learning?".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "Describe the backpropagation algorithm".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
            ],
            "Transformers" => vec![
                InterviewQuestion {
                    question: "Explain the attention mechanism in transformers".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "What is positional encoding and why is it needed?".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "Compare encoder-only, decoder-only, and encoder-decoder architectures".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 3,
                },
            ],
            "LLM Fine-tuning" => vec![
                InterviewQuestion {
                    question: "Explain the difference between fine-tuning and few-shot learning".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "What is LoRA and when would you use it?".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 3,
                },
                InterviewQuestion {
                    question: "How do you prevent catastrophic forgetting during fine-tuning?".to_string(),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 3,
                },
            ],
            _ => vec![
                InterviewQuestion {
                    question: format!("Explain your experience with {}", skill),
                    question_type: QuestionType::Technical,
                    related_skill: skill.to_string(),
                    difficulty: 2,
                },
            ],
        };

        InterviewRound {
            name: format!("Technical: {}", skill),
            questions,
            pass_threshold: match proficiency {
                Proficiency::Basic => 0.5,
                Proficiency::Intermediate => 0.6,
                Proficiency::Advanced => 0.7,
                Proficiency::Expert => 0.8,
                Proficiency::None => 0.5,
            },
        }
    }

    fn system_design_round() -> InterviewRound {
        InterviewRound {
            name: "System Design".to_string(),
            questions: vec![
                InterviewQuestion {
                    question: "Design a real-time recommendation system for an e-commerce platform".to_string(),
                    question_type: QuestionType::SystemDesign,
                    related_skill: "System Design".to_string(),
                    difficulty: 3,
                },
                InterviewQuestion {
                    question: "How would you design an ML pipeline for continuous model training?".to_string(),
                    question_type: QuestionType::SystemDesign,
                    related_skill: "MLOps".to_string(),
                    difficulty: 3,
                },
            ],
            pass_threshold: 0.6,
        }
    }

    fn behavioral_round() -> InterviewRound {
        InterviewRound {
            name: "Behavioral".to_string(),
            questions: vec![
                InterviewQuestion {
                    question: "Tell me about a time you had to explain complex ML concepts to non-technical stakeholders".to_string(),
                    question_type: QuestionType::Behavioral,
                    related_skill: "Communication".to_string(),
                    difficulty: 2,
                },
                InterviewQuestion {
                    question: "Describe a project where you had to balance technical debt with delivering features".to_string(),
                    question_type: QuestionType::Behavioral,
                    related_skill: "Communication".to_string(),
                    difficulty: 2,
                },
            ],
            pass_threshold: 0.6,
        }
    }

    pub fn answer_question(player: &Player, question: &InterviewQuestion) -> f32 {
        let proficiency = player.get_skill_proficiency(&question.related_skill);
        let base_score = match proficiency {
            Proficiency::None => 0.2,
            Proficiency::Basic => 0.4,
            Proficiency::Intermediate => 0.6,
            Proficiency::Advanced => 0.8,
            Proficiency::Expert => 0.95,
        };

        let mut rng = rand::thread_rng();
        let variance = 0.15;
        let adjustment: f32 = rng.gen_range(-variance..variance);
        
        (base_score + adjustment).clamp(0.0, 1.0)
    }

    pub fn conduct_round(player: &Player, round: &InterviewRound) -> InterviewResult {
        let mut total_score = 0.0;
        let mut feedback = Vec::new();

        for question in &round.questions {
            let score = Self::answer_question(player, question);
            total_score += score;
            
            feedback.push(format!(
                "Q: {}\nYour score: {:.0}%",
                question.question,
                score * 100.0
            ));
        }

        let avg_score = total_score / round.questions.len() as f32;
        let passed = avg_score >= round.pass_threshold;

        InterviewResult {
            round_name: round.name.clone(),
            score: avg_score,
            passed,
            feedback,
        }
    }
}
