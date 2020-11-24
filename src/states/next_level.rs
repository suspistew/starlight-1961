use amethyst::{SimpleState, StateData, GameData, SimpleTrans, Trans};
use amethyst::core::ecs::WorldExt;
use crate::states::CurrentState;
use crate::states::level_state::LevelState;

pub struct NextLevelState {
    pub next_level_nb: usize
}

impl SimpleState for NextLevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<CurrentState>() = CurrentState::NextLevel;
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        return Trans::Switch(Box::new(LevelState{ level_nb: self.next_level_nb }));
    }
}