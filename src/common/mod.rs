pub mod time;

use fixed::types::I32F32;
use nalgebra::{Vector2, Vector3};

pub type Tick = u64;
pub type Fixed = I32F32;
pub type Vec2F = Vector2<Fixed>;
pub type Vec3F = Vector3<Fixed>;

static TICKS_PER_VIEW: Tick = 50;
