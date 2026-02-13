mod companies;
mod engine;
mod game;
mod graphics;
mod interview;
mod jobs;
mod llm;
mod player;
mod skills;
mod ui;
mod world;

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use game::{GameScreen, GameState};
use world::{WorldPlayer, Camera, GameMap, BuildingType, Npc, get_npcs};
use ui::{draw_hud, draw_interaction_hint, draw_controls_hint};
use jobs::Job;
use graphics::{init_fonts, draw_text_crisp, use_custom_font, is_custom_font_enabled};

fn window_conf() -> Conf {
    Conf {
        window_title: "AI Engineer Career RPG".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct Dialog {
    pub speaker: String,
    pub text: String,
    pub choices: Vec<String>,
}

#[derive(Debug, Clone)]
struct QuizQuestion {
    question: String,
    options: Vec<String>,
    correct_idx: usize,
}

struct InterviewState {
    job: Job,
    questions: Vec<QuizQuestion>,
    current_question: usize,
    score: u32,
    selected_answer: usize,
}

struct Game {
    state: GameState,
    world_player: WorldPlayer,
    camera: Camera,
    map: GameMap,
    npcs: Vec<Npc>,
    current_dialog: Option<Dialog>,
    current_npc: Option<usize>,
    selected_choice: usize,
    player_name_input: String,
    input_active: bool,
    interview: Option<InterviewState>,
    scroll_offset: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::new(""),
            world_player: WorldPlayer::new(5.0 * 32.0, (world::MAP_HEIGHT as f32 - 5.0) * 32.0),
            camera: Camera::new(),
            map: GameMap::new(),
            npcs: get_npcs(),
            current_dialog: None,
            current_npc: None,
            selected_choice: 0,
            player_name_input: String::new(),
            input_active: true,
            interview: None,
            scroll_offset: 0,
        }
    }

    async fn update(&mut self) {
        let dt = get_frame_time();

        match self.state.screen {
            GameScreen::Title => {
                if self.input_active {
                    if is_key_pressed(KeyCode::Enter) && !self.player_name_input.is_empty() {
                        self.state = GameState::new(&self.player_name_input);
                        self.state.screen = GameScreen::World;
                        self.input_active = false;
                    }
                    
                    while let Some(c) = get_char_pressed() {
                        if c.is_alphanumeric() || c == ' ' {
                            if self.player_name_input.len() < 20 {
                                self.player_name_input.push(c);
                            }
                        }
                    }
                    if is_key_pressed(KeyCode::Backspace) && !self.player_name_input.is_empty() {
                        self.player_name_input.pop();
                    }
                }
            }
            GameScreen::World => {
                self.world_player.update(dt, &self.map);

                self.camera.follow(self.world_player.x, self.world_player.y);

                if is_key_pressed(KeyCode::E) {
                    let mut interacted = false;

                    for (i, npc) in self.npcs.iter().enumerate() {
                        if npc.distance_to(self.world_player.x, self.world_player.y) < 50.0 {
                            self.current_npc = Some(i);
                            let (name, text) = npc.get_dialog();
                            self.current_dialog = Some(Dialog {
                                speaker: name.to_string(),
                                text: text.to_string(),
                                choices: vec![],
                            });
                            self.state.screen = GameScreen::Dialog;
                            interacted = true;
                            break;
                        }
                    }

                    if !interacted {
                        if let Some(building) = self.map.get_building_at(self.world_player.x, self.world_player.y) {
                            let building = building.clone();
                            self.interact_with_building(&building);
                        }
                    }
                }

                if is_key_pressed(KeyCode::I) {
                    self.state.screen = GameScreen::Skills;
                }

                if is_key_pressed(KeyCode::J) {
                    self.state.screen = GameScreen::JobBoard;
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.state.screen = GameScreen::Menu;
                }

                if is_key_pressed(KeyCode::F) {
                    use_custom_font(!is_custom_font_enabled());
                }
            }
            GameScreen::Dialog => {
                if let Some(dialog) = &self.current_dialog {
                    if dialog.choices.is_empty() {
                        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
                            if let Some(npc_idx) = self.current_npc {
                                if !self.npcs[npc_idx].advance_dialog() {
                                    self.npcs[npc_idx].reset_dialog();
                                    self.current_npc = None;
                                } else {
                                    let (name, text) = self.npcs[npc_idx].get_dialog();
                                    self.current_dialog = Some(Dialog {
                                        speaker: name.to_string(),
                                        text: text.to_string(),
                                        choices: vec![],
                                    });
                                    return;
                                }
                            }
                            self.current_dialog = None;
                            self.state.screen = GameScreen::World;
                        }
                    } else {
                        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                            if self.selected_choice > 0 {
                                self.selected_choice -= 1;
                            }
                        }
                        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                            if self.selected_choice < dialog.choices.len() - 1 {
                                self.selected_choice += 1;
                            }
                        }
                        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
                            self.handle_dialog_choice();
                        }
                    }
                }
            }
            GameScreen::Skills => {
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::I) {
                    self.state.screen = GameScreen::World;
                }
            }
            GameScreen::Study => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state.screen = GameScreen::World;
                }
                if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                    if self.selected_choice > 0 {
                        self.selected_choice -= 1;
                    }
                }
                if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                    if self.selected_choice < self.state.player.skills.len() - 1 {
                        self.selected_choice += 1;
                    }
                }
                if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
                    self.handle_study();
                }
            }
            GameScreen::JobBoard => {
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::J) {
                    self.state.screen = GameScreen::World;
                }
                if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                    if self.selected_choice > 0 {
                        self.selected_choice -= 1;
                    }
                }
                if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                    let total_jobs: usize = companies::get_all_companies().iter().map(|c| c.open_positions.len()).sum();
                    if self.selected_choice < total_jobs - 1 {
                        self.selected_choice += 1;
                    }
                }
                if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
                    self.start_interview();
                }
            }
            GameScreen::Interview => {
                if let Some(ref interview) = self.interview {
                    if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                        if self.selected_choice > 0 {
                            self.selected_choice -= 1;
                        }
                    }
                    if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                        if self.selected_choice < 3 {
                            self.selected_choice += 1;
                        }
                    }
                    if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
                        self.answer_interview_question();
                    }
                }
            }
            GameScreen::Menu => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state.screen = GameScreen::World;
                }
            }
            _ => {}
        }
    }

    fn interact_with_building(&mut self, building: &world::Building) {
        match building.building_type {
            BuildingType::Apartment => {
                self.current_dialog = Some(Dialog {
                    speaker: "Home".to_string(),
                    text: "Welcome home! Would you like to rest?".to_string(),
                    choices: vec!["Rest (restore energy)".to_string(), "Leave".to_string()],
                });
                self.selected_choice = 0;
                self.state.screen = GameScreen::Dialog;
            }
            BuildingType::Library => {
                self.state.screen = GameScreen::Study;
            }
            BuildingType::CoffeeShop => {
                self.current_dialog = Some(Dialog {
                    speaker: "Barista".to_string(),
                    text: "Welcome! Care for some coffee? Great for networking!".to_string(),
                    choices: vec!["Buy coffee ($5)".to_string(), "Network with people".to_string(), "Leave".to_string()],
                });
                self.selected_choice = 0;
                self.state.screen = GameScreen::Dialog;
            }
            BuildingType::Company { tier: _ } => {
                self.current_dialog = Some(Dialog {
                    speaker: building.name.clone(),
                    text: format!("Welcome to {}! What would you like to do?", building.name),
                    choices: vec!["View open positions".to_string(), "Talk to recruiter".to_string(), "Leave".to_string()],
                });
                self.selected_choice = 0;
                self.state.screen = GameScreen::Dialog;
            }
            BuildingType::JobCenter => {
                self.state.screen = GameScreen::JobBoard;
            }
            BuildingType::Park => {
                self.current_dialog = Some(Dialog {
                    speaker: "Park".to_string(),
                    text: "A peaceful park. Great for clearing your mind.".to_string(),
                    choices: vec!["Relax (+energy)".to_string(), "Leave".to_string()],
                });
                self.selected_choice = 0;
                self.state.screen = GameScreen::Dialog;
            }
        }
    }

    fn handle_dialog_choice(&mut self) {
        if let Some(dialog) = &self.current_dialog {
            let choice_idx = self.selected_choice;
            let choice = dialog.choices.get(choice_idx).cloned().unwrap_or_default();

            if choice.contains("Rest") || choice.contains("Relax") {
                self.state.player.energy = self.state.player.max_energy;
                self.state.advance_time(8.0);
                self.state.screen = GameScreen::World;
                self.current_dialog = None;
                return;
            }
            if choice.contains("Buy coffee") {
                if self.state.player.money >= 5 {
                    self.state.player.money -= 5;
                    self.state.player.energy = (self.state.player.energy + 20).min(self.state.player.max_energy);
                }
                self.state.screen = GameScreen::World;
                self.current_dialog = None;
                return;
            }
            if choice.contains("View open positions") || choice == "Network with people" {
                self.state.screen = GameScreen::JobBoard;
                self.current_dialog = None;
                return;
            }
            if choice.contains("Leave") {
                self.state.screen = GameScreen::World;
                self.current_dialog = None;
                return;
            }
            if choice.contains("Awesome!") || choice.contains("OK") {
                self.state.screen = GameScreen::World;
                self.current_dialog = None;
                return;
            }
        }
        self.current_dialog = None;
        self.state.screen = GameScreen::World;
    }

    fn handle_study(&mut self) {
        let skills: Vec<_> = self.state.player.skills.iter().collect();
        if self.selected_choice < skills.len() {
            let skill_name = skills[self.selected_choice].0.clone();
            let energy_cost = 30;
            
            if self.state.player.energy >= energy_cost {
                if let Some(skill) = self.state.player.skills.get_mut(&skill_name) {
                    self.state.player.energy -= energy_cost;
                    let xp_gained = 50;
                    skill.add_experience(xp_gained);
                    self.state.advance_time(2.0);
                }
            }
        }
    }

    fn start_interview(&mut self) {
        let mut idx = 0;
        let mut target_job: Option<Job> = None;
        
        'outer: for company in companies::get_all_companies() {
            for job in &company.open_positions {
                if idx == self.selected_choice {
                    target_job = Some(job.clone());
                    break 'outer;
                }
                idx += 1;
            }
        }
        
        if let Some(job) = target_job {
            let questions = self.generate_interview_questions(&job);
            self.interview = Some(InterviewState {
                job,
                questions,
                current_question: 0,
                score: 0,
                selected_answer: 0,
            });
            self.selected_choice = 0;
            self.state.screen = GameScreen::Interview;
        }
    }

    fn generate_interview_questions(&self, job: &Job) -> Vec<QuizQuestion> {
        let mut questions = Vec::new();
        
        for req in &job.requirements {
            if req.mandatory {
                let q = self.create_question_for_skill(&req.skill_name);
                questions.push(q);
            }
        }
        
        if questions.len() > 5 {
            questions.shuffle();
            questions.truncate(5);
        }
        
        if questions.is_empty() {
            questions.push(QuizQuestion {
                question: "Why do you want to work here?".to_string(),
                options: vec![
                    "I'm passionate about AI and want to learn".to_string(),
                    "For the money".to_string(),
                    "My friend works here".to_string(),
                    "I don't know".to_string(),
                ],
                correct_idx: 0,
            });
        }
        
        questions
    }

    fn create_question_for_skill(&self, skill_name: &str) -> QuizQuestion {
        match skill_name {
            "Python" => QuizQuestion {
                question: "What is the difference between a list and a tuple in Python?".to_string(),
                options: vec![
                    "Lists are mutable, tuples are immutable".to_string(),
                    "Lists are faster than tuples".to_string(),
                    "Tuples can hold more items".to_string(),
                    "There is no difference".to_string(),
                ],
                correct_idx: 0,
            },
            "PyTorch" | "TensorFlow" => QuizQuestion {
                question: "What is backpropagation?".to_string(),
                options: vec![
                    "Algorithm to compute gradients by chain rule".to_string(),
                    "A type of neural network layer".to_string(),
                    "Data preprocessing technique".to_string(),
                    "A loss function".to_string(),
                ],
                correct_idx: 0,
            },
            "Transformers" => QuizQuestion {
                question: "What is the key innovation in Transformer architecture?".to_string(),
                options: vec![
                    "Self-attention mechanism".to_string(),
                    "Convolutional layers".to_string(),
                    "Recurrent connections".to_string(),
                    "Dropout regularization".to_string(),
                ],
                correct_idx: 0,
            },
            "LLM Fine-tuning" => QuizQuestion {
                question: "What is LoRA?".to_string(),
                options: vec![
                    "Low-Rank Adaptation for efficient fine-tuning".to_string(),
                    "A type of language model".to_string(),
                    "A tokenization method".to_string(),
                    "A training loss function".to_string(),
                ],
                correct_idx: 0,
            },
            "SQL" => QuizQuestion {
                question: "Which SQL clause is used to filter results?".to_string(),
                options: vec![
                    "WHERE".to_string(),
                    "ORDER BY".to_string(),
                    "GROUP BY".to_string(),
                    "SELECT".to_string(),
                ],
                correct_idx: 0,
            },
            "Statistics" => QuizQuestion {
                question: "What is the mean of [2, 4, 6, 8]?".to_string(),
                options: vec![
                    "5".to_string(),
                    "4".to_string(),
                    "6".to_string(),
                    "4.5".to_string(),
                ],
                correct_idx: 0,
            },
            _ => QuizQuestion {
                question: format!("Explain your experience with {}", skill_name),
                options: vec![
                    "I have strong practical experience".to_string(),
                    "I've studied it but need practice".to_string(),
                    "I've heard of it".to_string(),
                    "I don't know this".to_string(),
                ],
                correct_idx: 0,
            },
        }
    }

    fn answer_interview_question(&mut self) {
        if let Some(ref mut interview) = self.interview {
            let current = interview.current_question;
            if current < interview.questions.len() {
                if interview.selected_answer == interview.questions[current].correct_idx {
                    interview.score += 1;
                }
                interview.current_question += 1;
                interview.selected_answer = 0;
                
                if interview.current_question >= interview.questions.len() {
                    let total = interview.questions.len() as u32;
                    let score = interview.score;
                    let job = interview.job.clone();
                    
                    if score >= total / 2 {
                        let salary = (job.salary_min + job.salary_max) / 2;
                        self.state.player.employed = true;
                        self.state.player.current_salary = salary;
                        self.current_dialog = Some(Dialog {
                            speaker: "Interview Complete".to_string(),
                            text: format!("Congratulations! You got the job!\nPosition: {} at {}\nSalary: ${}/year", 
                                job.title, job.company, salary),
                            choices: vec!["Awesome!".to_string()],
                        });
                    } else {
                        self.current_dialog = Some(Dialog {
                            speaker: "Interview Complete".to_string(),
                            text: format!("Unfortunately, you didn't pass. Score: {}/{}\nKeep studying and try again!", 
                                score, total),
                            choices: vec!["OK".to_string()],
                        });
                    }
                    
                    self.interview = None;
                    self.state.screen = GameScreen::Dialog;
                }
            }
        }
    }

    async fn draw(&mut self) {
        clear_background(DARKGRAY);

        match self.state.screen {
            GameScreen::Title => self.draw_title_screen(),
            GameScreen::World => self.draw_world(),
            GameScreen::Dialog => {
                self.draw_world();
                self.draw_dialog();
            }
            GameScreen::Skills => {
                self.draw_world();
                self.draw_skills_screen();
            }
            GameScreen::Study => {
                self.draw_world();
                self.draw_study_screen();
            }
            GameScreen::JobBoard => {
                self.draw_world();
                self.draw_job_board();
            }
            GameScreen::Interview => {
                self.draw_world();
                self.draw_interview_screen();
            }
            GameScreen::Menu => {
                self.draw_world();
                self.draw_menu();
            }
            _ => {}
        }
    }

    fn draw_title_screen(&mut self) {
        let title = "AI ENGINEER CAREER RPG";
        draw_text_crisp(title, screen_width() / 2.0 - 250.0, screen_height() / 3.0, 48.0, WHITE);

        let subtitle = "Level up your skills, ace interviews, land your dream job!";
        draw_text_crisp(subtitle, screen_width() / 2.0 - 280.0, screen_height() / 3.0 + 50.0, 24.0, Color::from_rgba(200, 200, 200, 255));

        draw_text_crisp("Enter your name:", screen_width() / 2.0 - 80.0, screen_height() / 2.0, 24.0, WHITE);

        let input_box_width = 200.0;
        let input_box_x = screen_width() / 2.0 - input_box_width / 2.0;
        draw_rectangle(input_box_x, screen_height() / 2.0 + 10.0, input_box_width, 35.0, Color::from_rgba(50, 50, 70, 255));
        draw_rectangle(input_box_x + 2.0, screen_height() / 2.0 + 12.0, input_box_width - 4.0, 31.0, Color::from_rgba(30, 30, 50, 255));

        let cursor = if (get_time() * 2.0) as i32 % 2 == 0 { "|" } else { "" };
        let display_text = format!("{}{}", self.player_name_input, cursor);
        draw_text_crisp(&display_text, input_box_x + 10.0, screen_height() / 2.0 + 35.0, 24.0, WHITE);

        if !self.player_name_input.is_empty() {
            draw_text_crisp("Press ENTER to start", screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 100.0, 20.0, Color::from_rgba(150, 255, 150, 255));
        }

        draw_text_crisp("WASD to move | E to interact | I for skills | J for jobs", 
            screen_width() / 2.0 - 230.0, screen_height() - 50.0, 18.0, Color::from_rgba(150, 150, 150, 255));
    }

    fn draw_world(&mut self) {
        let sw = screen_width();
        let sh = screen_height();
        
        let cam_x = self.camera.x;
        let cam_y = self.camera.y;
        
        self.map.draw(cam_x, cam_y);
        
        for npc in &self.npcs {
            let (sx, sy) = self.camera.world_to_screen(npc.x, npc.y);
            if sx > -50.0 && sx < sw + 50.0 && sy > -50.0 && sy < sh + 50.0 {
                graphics::draw_npc(sx, sy, npc.npc_type_id());
            }
        }
        
        let (px, py) = self.camera.world_to_screen(self.world_player.x, self.world_player.y);
        graphics::draw_player(
            px,
            py,
            self.world_player.direction,
            self.world_player.walking,
            self.world_player.anim_timer,
        );

        draw_hud(&self.state);
        draw_controls_hint();

        let mut hint_shown = false;

        for npc in &self.npcs {
            if npc.distance_to(self.world_player.x, self.world_player.y) < 50.0 {
                draw_interaction_hint(&format!("Press E to talk to {}", npc.name));
                hint_shown = true;
                break;
            }
        }

        if !hint_shown {
            if let Some(building) = self.map.get_building_at(self.world_player.x, self.world_player.y) {
                draw_interaction_hint(&format!("Press E to enter {}", building.name));
            }
        }
    }

    fn draw_dialog(&mut self) {
        if let Some(dialog) = &self.current_dialog {
            let box_height = 180.0;
            let box_y = screen_height() - box_height - 20.0;
            let box_margin = 50.0;

            draw_rectangle(box_margin, box_y, screen_width() - box_margin * 2.0, box_height, Color::from_rgba(0, 0, 0, 220));
            draw_rectangle_lines(box_margin, box_y, screen_width() - box_margin * 2.0, box_height, 2.0, WHITE);

            draw_text_crisp(&dialog.speaker, box_margin + 15.0, box_y + 25.0, 22.0, Color::from_rgba(255, 215, 0, 255));

            draw_text_crisp(&dialog.text, box_margin + 15.0, box_y + 55.0, 20.0, WHITE);

            for (i, choice) in dialog.choices.iter().enumerate() {
                let choice_y = box_y + 85.0 + (i as f32 * 28.0);
                let prefix = if i == self.selected_choice { "> " } else { "  " };
                let color = if i == self.selected_choice { Color::from_rgba(255, 255, 100, 255) } else { WHITE };
                draw_text_crisp(&format!("{}{}", prefix, choice), box_margin + 15.0, choice_y, 18.0, color);
            }
        }
    }

    fn draw_skills_screen(&mut self) {
        let panel_width = 600.0;
        let panel_height = 500.0;
        let panel_x = (screen_width() - panel_width) / 2.0;
        let panel_y = (screen_height() - panel_height) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(0, 0, 0, 240));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);

        draw_text_crisp("YOUR SKILLS", panel_x + 20.0, panel_y + 30.0, 24.0, Color::from_rgba(255, 215, 0, 255));
        draw_text_crisp("Press ESC or I to close", panel_x + 20.0, panel_y + 55.0, 14.0, Color::from_rgba(150, 150, 150, 255));

        let by_category = self.state.player.get_skills_by_category();
        let categories: [&skills::SkillCategory; 6] = [
            &skills::SkillCategory::Programming,
            &skills::SkillCategory::MlAlgorithms,
            &skills::SkillCategory::Statistics,
            &skills::SkillCategory::Databases,
            &skills::SkillCategory::SoftSkills,
            &skills::SkillCategory::DomainKnowledge,
        ];

        let mut y = panel_y + 85.0;
        for category in &categories {
            if let Some(skills_list) = by_category.get(*category) {
                draw_text_crisp(&format!("{:?}", category), panel_x + 20.0, y, 16.0, Color::from_rgba(100, 200, 255, 255));
                y += 22.0;
                
                for (name, skill) in skills_list {
                    let xp_bar = self.skill_xp_bar(skill.experience_points, skill.points_to_next_level());
                    draw_text_crisp(&format!("{}: {} {}", name, skill.proficiency.as_str(), xp_bar), 
                        panel_x + 40.0, y, 14.0, WHITE);
                    y += 18.0;
                }
                y += 10.0;
            }
        }
    }

    fn draw_study_screen(&mut self) {
        let panel_width = 600.0;
        let panel_height = 550.0;
        let panel_x = (screen_width() - panel_width) / 2.0;
        let panel_y = (screen_height() - panel_height) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(0, 0, 0, 240));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);

        draw_text_crisp("LIBRARY - Study Skills", panel_x + 20.0, panel_y + 30.0, 24.0, Color::from_rgba(255, 215, 0, 255));
        draw_text_crisp(&format!("Energy: {}/100 (30 per study session)", self.state.player.energy), 
            panel_x + 20.0, panel_y + 55.0, 14.0, Color::from_rgba(150, 150, 150, 255));
        draw_text_crisp("Press ESC to leave | WS/Arrows to select | E to study", 
            panel_x + 20.0, panel_y + 75.0, 14.0, Color::from_rgba(150, 150, 150, 255));

        let skills: Vec<_> = self.state.player.skills.iter().collect();
        let mut y = panel_y + 100.0;

        for (i, (name, skill)) in skills.iter().enumerate() {
            let selected = i == self.selected_choice;
            let prefix = if selected { "> " } else { "  " };
            let color = if selected { Color::from_rgba(255, 255, 100, 255) } else { WHITE };
            let xp_bar = self.skill_xp_bar(skill.experience_points, skill.points_to_next_level());
            
            draw_text_crisp(&format!("{}{}: {} {}", prefix, name, skill.proficiency.as_str(), xp_bar), 
                panel_x + 30.0, y, 16.0, color);
            
            if selected {
                draw_text_crisp(&format!("Difficulty: {} | XP to next: {}", 
                    skill.skill.difficulty, skill.points_to_next_level() - skill.experience_points),
                    panel_x + 50.0, y + 18.0, 12.0, Color::from_rgba(150, 150, 150, 255));
                y += 20.0;
            }
            y += 25.0;
        }
    }

    fn skill_xp_bar(&self, current: u32, max: u32) -> String {
        if max == 0 { return String::new(); }
        let filled = (current as f32 / max as f32 * 10.0) as usize;
        format!("[{}{}]", "=".repeat(filled), " ".repeat(10 - filled))
    }

    fn draw_job_board(&mut self) {
        let panel_width = 700.0;
        let panel_height = 550.0;
        let panel_x = (screen_width() - panel_width) / 2.0;
        let panel_y = (screen_height() - panel_height) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(0, 0, 0, 240));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);

        draw_text_crisp("JOB BOARD - Press E to Apply", panel_x + 20.0, panel_y + 30.0, 24.0, Color::from_rgba(255, 215, 0, 255));
        draw_text_crisp("WASD to navigate | ESC or J to close", panel_x + 20.0, panel_y + 55.0, 14.0, Color::from_rgba(150, 150, 150, 255));

        let mut y = panel_y + 90.0;
        let mut idx = 0;
        for company in companies::get_all_companies() {
            draw_text_crisp(&format!("{} ({})", company.name, company.tier.as_str()), 
                panel_x + 20.0, y, 18.0, Color::from_rgba(100, 200, 255, 255));
            y += 22.0;

            for job in &company.open_positions {
                let selected = idx == self.selected_choice;
                let match_score = job.calculate_match(&self.state.player.skills) * 100.0;
                let match_indicator = if match_score >= 70.0 { "[GOOD MATCH]" } 
                    else if match_score >= 40.0 { "[PARTIAL]" } 
                    else { "[SKILLS NEEDED]" };
                let match_color = if match_score >= 70.0 { Color::from_rgba(100, 255, 100, 255) }
                    else if match_score >= 40.0 { Color::from_rgba(255, 255, 100, 255) }
                    else { Color::from_rgba(255, 100, 100, 255) };

                let prefix = if selected { "> " } else { "  " };
                let text_color = if selected { Color::from_rgba(255, 255, 100, 255) } else { WHITE };
                
                draw_text_crisp(&format!("{}{} - {}", prefix, job.title, job.display_salary()), 
                    panel_x + 30.0, y, 14.0, text_color);
                draw_text_crisp(match_indicator, panel_x + 450.0, y, 14.0, match_color);
                y += 20.0;
                idx += 1;
            }
            y += 15.0;
        }
    }

    fn draw_interview_screen(&mut self) {
        if let Some(ref interview) = self.interview {
            let panel_width = 700.0;
            let panel_height = 450.0;
            let panel_x = (screen_width() - panel_width) / 2.0;
            let panel_y = (screen_height() - panel_height) / 2.0;

            draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(0, 0, 0, 240));
            draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);

            draw_text_crisp(&format!("INTERVIEW: {} at {}", interview.job.title, interview.job.company), 
                panel_x + 20.0, panel_y + 30.0, 22.0, Color::from_rgba(255, 215, 0, 255));
            
            draw_text_crisp(&format!("Question {}/{} | Score: {}", 
                interview.current_question + 1, interview.questions.len(), interview.score), 
                panel_x + 20.0, panel_y + 55.0, 14.0, Color::from_rgba(150, 150, 150, 255));

            if interview.current_question < interview.questions.len() {
                let q = &interview.questions[interview.current_question];
                
                draw_text_crisp(&q.question, panel_x + 20.0, panel_y + 100.0, 18.0, WHITE);

                let mut y = panel_y + 150.0;
                for (i, option) in q.options.iter().enumerate() {
                    let selected = i == self.selected_choice;
                    let prefix = if selected { "> " } else { "  " };
                    let color = if selected { Color::from_rgba(255, 255, 100, 255) } else { WHITE };
                    draw_text_crisp(&format!("{}. {}{}", (i + 65) as u8 as char, prefix, option), 
                        panel_x + 30.0, y, 16.0, color);
                    y += 30.0;
                }
                
                draw_text_crisp("WASD to select | E to answer", 
                    panel_x + 20.0, panel_y + panel_height - 30.0, 14.0, Color::from_rgba(150, 150, 150, 255));
            }
        }
    }

    fn draw_menu(&mut self) {
        let panel_width = 300.0;
        let panel_height = 200.0;
        let panel_x = (screen_width() - panel_width) / 2.0;
        let panel_y = (screen_height() - panel_height) / 2.0;

        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(0, 0, 0, 240));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);

        draw_text_crisp("MENU", panel_x + 20.0, panel_y + 30.0, 24.0, WHITE);

        let options = ["Resume", "View Skills (I)", "Job Board (J)", "Quit"];
        for (i, option) in options.iter().enumerate() {
            draw_text_crisp(option, panel_x + 30.0, panel_y + 70.0 + (i as f32 * 30.0), 18.0, WHITE);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    init_fonts();
    let mut game = Game::new();

    loop {
        game.update().await;
        game.draw().await;
        next_frame().await
    }
}
