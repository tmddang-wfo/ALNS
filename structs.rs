#![allow(non_snake_case)]
use crate::utils::*;

#[derive(Debug)]
pub struct Staff {
    pub id: usize,
    pub agency: usize,
    pub group: usize,
}

#[derive(Debug)]
pub struct Day {
    pub id: usize,
    pub day_type: usize,
    pub morning_cov: usize,
    pub afternoon_cov: usize,
}

#[derive(Debug)]
pub struct Shift {
    pub id: usize,
    pub shift_type : usize,
    pub shift_duration: usize,
}

#[derive(Clone, Debug)]
pub struct Solution {
    pub staffs_schedule: Vec<Vec<usize>>,
    pub fitness_val: f64,
    pub CoveragePenalty: i32,
    pub WorktimePenalty: Vec<usize>,
    pub ConsecutivePenalty: Vec<usize>,
    pub AfternoonPenalty: Vec<usize>,
    pub CoverageList: Vec<usize>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DestroyOp {Random, Worst}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RepairOp {Greedy}


impl Solution {
    pub fn new(staffs_schedule: Vec<Vec<usize>>, shifts: &Vec<Shift>, days: &Vec<Day>) -> Self {
        let CoveragePenalty: i32 = Self::calculate_CoveragePenalty(&staffs_schedule, &days);

        let CoverageList: Vec<usize> = Self::check_CoveragePenalty(&staffs_schedule, &days);

        let WorktimePenalty: Vec<usize> = Self::calculate_WorktimePenalty(&staffs_schedule, &shifts);
        let mut Global_WorktimePenalty: usize = 0;
        for n in &WorktimePenalty {
            Global_WorktimePenalty += *n;
        }

        let ConsecutivePenalty: Vec<usize> = Self::calculate_ConsecutivePenalty(&staffs_schedule);
        let mut Global_ConsecutivePenalty: usize = 0;
        for n in &ConsecutivePenalty {
            Global_ConsecutivePenalty += *n;
        }

        let AfternoonPenalty: Vec<usize> = Self::calculate_AfternoonPenalty(&staffs_schedule);
        let mut Global_AfternoonPenalty: usize = 0;
        for n in &AfternoonPenalty {
            Global_AfternoonPenalty += *n;
        }

        let fitness_val = W1*CoveragePenalty as f64
            + W2*Global_WorktimePenalty as f64
            + W3*Global_ConsecutivePenalty as f64
            + W4*Global_AfternoonPenalty as f64;

        Self {staffs_schedule,
            fitness_val,
            CoveragePenalty,
            WorktimePenalty,
            ConsecutivePenalty,
            AfternoonPenalty,
            CoverageList,}
    }

    fn calculate_WorktimePenalty(staffs_schedule: &Vec<Vec<usize>>, shifts: &Vec<Shift>) -> Vec<usize> {
        let mut WorktimePenalty: Vec<usize> = vec![0; STAFF_NUM];
        for i in 0..STAFF_NUM {
            let mut ActualWorktime: i32 = 0;
            for k in 0..DAY_NUM {
                let shift_idx = staffs_schedule[i][k];
                ActualWorktime += shifts[shift_idx].shift_duration as i32;
            }
            WorktimePenalty[i] = (44 - ActualWorktime).abs() as usize;
        }
        //let Global_WorktimePenalty: i32 = WorktimePenalty.iter().sum();
        WorktimePenalty
        //Global_WorktimePenalty
    }

    fn calculate_CoveragePenalty(staffs_schedule: &Vec<Vec<usize>>, days: &Vec<Day>) -> i32 {
        let mut CoveragePenalty: i32 = 0;
        for k in 0..DAY_NUM {
            let mut MorningActualCov: i32 = 0;
            let mut AfternoonActualCov: i32 = 0;
            let mut MorningPenaltyCov:i32 = 0;
            let mut AfternoonPenaltyCov: i32 = 0;
            for i in 0..STAFF_NUM {
                if MORNING_SHIFT.contains(&(staffs_schedule[i][k] as i32)) {
                    MorningActualCov += 1;
                } else if AFTERNOON_SHIFT.contains(&(staffs_schedule[i][k] as i32)) {
                    AfternoonActualCov += 1;
                }
            }
            MorningPenaltyCov = ((days[k].morning_cov as i32) - MorningActualCov).abs();
            AfternoonPenaltyCov = ((days[k].afternoon_cov as i32) - AfternoonActualCov).abs();
            CoveragePenalty += MorningPenaltyCov + AfternoonPenaltyCov;
        }
        CoveragePenalty
    }

    fn check_CoveragePenalty(staffs_schedule: &Vec<Vec<usize>>, days: &Vec<Day>) -> Vec<usize> {
        let mut CoverageList: Vec<usize> = vec![0; DAY_NUM];
        for k in 0..DAY_NUM {
            let mut MorningActualCov: i32 = 0;
            let mut AfternoonActualCov: i32 = 0;
            let mut MorningPenaltyCov:i32 = 0;
            let mut AfternoonPenaltyCov: i32 = 0;
            let mut PenaltyValue: i32 = 0;
            for i in 0..STAFF_NUM {
                if MORNING_SHIFT.contains(&(staffs_schedule[i][k] as i32)) {
                    MorningActualCov += 1;
                } else if AFTERNOON_SHIFT.contains(&(staffs_schedule[i][k] as i32)) {
                    AfternoonActualCov += 1;
                }
            }
            MorningPenaltyCov = ((days[k].morning_cov as i32) - MorningActualCov).abs();
            AfternoonPenaltyCov = ((days[k].afternoon_cov as i32) - AfternoonActualCov).abs();
            PenaltyValue += MorningPenaltyCov + AfternoonPenaltyCov;
            CoverageList[k] = PenaltyValue as usize;
        }
        CoverageList
    }

    fn calculate_ConsecutivePenalty(staffs_schedule: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut ConsecutivePenalty = vec![0; STAFF_NUM];
        for i in 0..STAFF_NUM {
            for k in 0..DAY_NUM - 2 {
                if AFTERNOON_SHIFT.contains(&(staffs_schedule[i][k] as i32))
                    && AFTERNOON_SHIFT.contains(&(staffs_schedule[i][k+1] as i32))
                    && AFTERNOON_SHIFT.contains(&(staffs_schedule[i][k+2] as i32)) {
                    ConsecutivePenalty[i] += ALPHA_PENALTY
                }
            }
        }
        ConsecutivePenalty
    }

    fn calculate_AfternoonPenalty(staffs_schedule: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut AfternoonPenalty = vec![0; STAFF_NUM];
        for i in 0..STAFF_NUM {
            for k in 1..DAY_NUM {
                if OTHER_SHIFT.contains(&(staffs_schedule[i][k-1] as i32))
                    && AFTERNOON_SHIFT.contains(&(staffs_schedule[i][k] as i32)) {
                    AfternoonPenalty[i] += BETA_PENALTY
                }
            }
        }
        AfternoonPenalty
    }
}

