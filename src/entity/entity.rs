use entity::Attackable;
use entity::Drawable;
use std::collections::HashMap;

// Create Entity concept
pub trait Entity : Attackable + Drawable {}
impl<T> Entity for T where T: Attackable + Drawable {}

pub type EntityMap = HashMap<usize, Box<Entity>>;