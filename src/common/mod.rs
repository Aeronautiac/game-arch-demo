pub mod time;

use fixed::types::I32F32;

pub type Tick = u64;
pub type Fixed = I32F32;

static TICKS_PER_VIEW: Tick = 50;
