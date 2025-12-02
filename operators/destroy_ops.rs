use rand::Rng;
use crate::utils::{DAY_NUM, STAFF_NUM};
use crate::structs::Solution;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum DestroyOp {Random, Worst}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct DestroyResult {
    pub partial_schedule: Vec<Vec<usize>>,
    pub removed_staff: usize,
}

pub fn random_destroy (sol: &Solution) -> DestroyResult {
    let mut partial_schedule = sol.staffs_schedule.clone();

    let mut rng = rand::rng();
    let removed_staff = rng.random_range(0..STAFF_NUM+1);

    for k in 0..DAY_NUM {
        partial_schedule[removed_staff][k] = 0;
    }

    DestroyResult {partial_schedule, removed_staff}
}

pub fn worst_destroy (sol: &Solution) -> DestroyResult {
    let mut partial_schedule = sol.staffs_schedule.clone();
    let mut worst_penalty = 0;
    let mut removed_staff: Option<usize> = None;

    for i in 0..STAFF_NUM {
        let mut staff_penalty = 0;
        staff_penalty = sol.WorktimePenalty[i] + sol.AfternoonPenalty[i] + sol.WorktimePenalty[i];
        if staff_penalty > worst_penalty {
            worst_penalty = staff_penalty;
            removed_staff = Some(i);
        }
    }

    if let Some(staff) = removed_staff {
        for k in 0..DAY_NUM {
            partial_schedule[staff][k] = 0;
        }
    }

    DestroyResult {partial_schedule, removed_staff: removed_staff.unwrap()}
}