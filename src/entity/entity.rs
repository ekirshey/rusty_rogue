use super::attack::Attackable;
use super::display::Drawable;
use std::collections::HashMap;

// Create Entity concept
pub trait Entity : Attackable + Drawable {}
impl<T> Entity for T where T: Attackable + Drawable {}

pub type EntityMap = HashMap<u32, Box<Entity>>;