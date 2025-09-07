use serde::{Deserialize, Serialize};

use crate::player::PlayerSync;

#[derive(Clone, Serialize, Deserialize)]
pub enum Sendables {
    PlayerSync(PlayerSync),
}
