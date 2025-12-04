//Define all constant values
pub const SHIFT_NUM: usize = 7;
pub const STAFF_NUM: usize = 9;
pub const DAY_NUM: usize = 7;
pub const DAY_OFF:usize = 6;
pub const PH_SHIFT:usize = 7;
pub const MORNING_SHIFT: [i32; 3] = [1, 2, 3];
pub const AFTERNOON_SHIFT: [i32; 2] = [4, 5];
pub const ALL_SHIFT: [i32; 5] = [1, 2, 3, 4, 5];
pub const OTHER_SHIFT: [i32; 2] = [6, 7];
pub const ALPHA_PENALTY: usize = 10;
pub const BETA_PENALTY: usize = 10;
pub const W1: f64 = 0.25;
pub const W2: f64 = 0.25;
pub const W3: f64 = 0.25;
pub const W4: f64 = 0.25;
pub const WORKING_SHIFT: [i32; 5] = [1, 2, 3, 4 ,5];

//ALNS config
pub const GLOBAL_BEST: f64 = 15.0;
pub const LOCAL_BEST: f64 = 10.0;
pub const ACCEPTED: f64 = 5.0;
pub const UPDATE_FREQUENCY: usize = 5;
pub const ITERATIONS: usize = 200;
pub const REACTION_FACTOR: f64 = 0.1;
pub const START_TEMP: f64= 100.0;
pub const COOLING_RATE: f64 = 0.9995;


