use crate::player::Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameScreen {
    Title,
    World,
    Dialog,
    Menu,
    Skills,
    JobBoard,
    Interview,
    Study,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub screen: GameScreen,
    pub player: Player,
    pub day: u32,
    pub time_of_day: f32,
    pub paused: bool,
}

impl GameState {
    pub fn new(player_name: &str) -> Self {
        Self {
            screen: GameScreen::Title,
            player: Player::new(player_name),
            day: 1,
            time_of_day: 8.0,
            paused: false,
        }
    }

    pub fn advance_time(&mut self, hours: f32) {
        self.time_of_day += hours;
        if self.time_of_day >= 24.0 {
            self.time_of_day -= 24.0;
            self.day += 1;
            self.player.rest();
        }
    }

    pub fn time_string(&self) -> String {
        let hour = self.time_of_day.floor() as u32;
        let minute = ((self.time_of_day % 1.0) * 60.0) as u32;
        format!("{:02}:{:02}", hour, minute)
    }

    pub fn is_night(&self) -> bool {
        self.time_of_day < 6.0 || self.time_of_day >= 20.0
    }
}
