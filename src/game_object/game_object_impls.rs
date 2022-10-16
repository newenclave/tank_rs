use crate::{game_object_area::GameObjectAreaImpl, animated::Animated, sprite::Sprite};

pub type GameObjectAnimated = GameObjectAreaImpl<Animated>;
pub type GameObjectStatic = GameObjectAreaImpl<Sprite>;
