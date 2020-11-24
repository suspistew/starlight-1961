use amethyst::{SimpleState, StateData, GameData};
use amethyst::core::ecs::WorldExt;
use crate::states::CurrentState;

pub struct EndLevelState;

impl SimpleState for EndLevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<CurrentState>() = CurrentState::End;
    }
}