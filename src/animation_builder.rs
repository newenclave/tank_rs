use std::time::Duration;
use crate::{animated::Animated, sprite::Sprite};

pub struct AnimationBuilder {
    value: Animated,
}

impl AnimationBuilder {
    pub fn new(switch_delay: Duration) -> Self {
        Self {
            value: Animated::new(switch_delay)
        }
    }

    pub fn new_static() -> Self {
        Self {
            value: Animated::new_static(),
        }
    }

    pub fn new_looped(switch_delay: Duration) -> Self {
        Self {
            value: Animated::new_looped(switch_delay),
        }
    }

    pub fn add_sprite(mut self, s: Sprite) -> Self {
        self.value.add_sprite(s);
        self
    }

    pub fn add_sprite_from_string(mut self, s: &str) -> Self {
        self.value.add_sprite(Sprite::new_from_string(s));
        self
    }

    pub fn add_from_string(mut self, s: &str) -> Self {
        self.value.add_from_string(s);
        self
    }

    pub fn modify<F>(mut self, call: F) -> Self 
    where 
        F: Fn(Animated) -> Animated
    {
        self.value = call(self.value);
        self
    }

    pub fn build(self) -> Animated {
        self.value
    }

}
