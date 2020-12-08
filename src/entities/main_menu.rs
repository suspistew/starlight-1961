use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct MenuBackground {
    pub parallax_index: usize,
}

impl Component for MenuBackground {
    type Storage = DenseVecStorage<Self>;
}

pub struct PushEnter;
impl Component for PushEnter {
    type Storage = DenseVecStorage<Self>;
}
