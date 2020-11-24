pub mod level_state;
pub mod main_menu_state;
pub mod next_level;
pub mod end_state;

#[derive(PartialEq)]
pub enum CurrentState {
    MainMenu,
    Level,
    NextLevel,
    End
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::MainMenu
    }
}