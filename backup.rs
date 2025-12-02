use std::collections::HashMap;
use rand::prelude::*;

#[derive(Debug)]
struct Staff {
    id: usize,
    agency: usize,
    group: usize,
}

#[derive(Debug)]
struct Day {
    id: usize,
    day_type: usize,
    morning_cov: usize,
    afternoon_cov: usize,
}

#[derive(Debug)]
struct Shift {
    id: usize,
    shift_type : usize,
    shift_duration: usize,
}

#[derive(Clone, Debug)]
struct Solution {
    staffs_schedule: Vec<Vec<usize>>,
    fitness_val: f64,
    CoveragePenalty: i32,
    WorktimePenalty: Vec<usize>,
    ConsecutivePenalty: Vec<usize>,
    AfternoonPenalty: Vec<usize>,
}

impl Solution {
    fn new(staffs_schedule: Vec<Vec<usize>>, shifts: Vec<Shift>, days: Vec<Day>) -> Self {
        let CoveragePenalty: i32 = Self::calculate_CoveragePenalty(&staffs_schedule, &days);

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
            + W3*Global_WorktimePenalty as f64
            + W4*Global_WorktimePenalty as f64;

        Self {staffs_schedule, fitness_val, CoveragePenalty, WorktimePenalty, ConsecutivePenalty, AfternoonPenalty,}
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
                } else {
                    continue;
                }
            }
            MorningPenaltyCov = ((days[k].morning_cov as i32) - MorningActualCov).abs();
            AfternoonPenaltyCov = ((days[k].afternoon_cov as i32) - AfternoonActualCov).abs();
            CoveragePenalty += (MorningPenaltyCov + AfternoonPenaltyCov);
        }
        CoveragePenalty
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



const SHIFT_NUM: usize = 7;
const STAFF_NUM: usize = 9;
const DAY_NUM: usize = 7;
const DAY_OFF:usize = 6;
const PH_SHIFT:usize = 7;
const MORNING_SHIFT: [i32; 3] = [1, 2, 3];
const AFTERNOON_SHIFT: [i32; 2] = [4, 5];
const OTHER_SHIFT: [i32; 3] = [0, 6, 7];

const ALPHA_PENALTY: usize = 10;
const BETA_PENALTY: usize = 10;
const W1: f64 = 0.25;
const W2: f64 = 0.25;
const W3: f64 = 0.25;
const W4: f64 = 0.25;

fn main() {
    let days_info = [0,0,2,2,0,1,1];

    let days_info: Vec<Vec<usize>> = vec![
        vec![0, 0, 4, 3],
        vec![1, 0, 4, 3],
        vec![2, 2, 3, 3],
        vec![3, 2, 3, 3],
        vec![4, 0, 4, 3],
        vec![5, 1, 3, 3],
        vec![6, 1, 3, 3]
    ];

    let mut rng = rand::rng();
    let staffs_info: Vec<Vec<usize>> = vec![
        vec![0, 0, 1], //Staff 1, AG1, Fixed shifts
        vec![1, 0, 2], //Staff 2, AG1, Off on PH
        vec![2, 0, 2], //Staff 3, AG1, Off on PH
        vec![3, 0, 2], //Staff 4, AG1, Off on PH
        vec![4, 2, 0], //Staff 5, AG3, N/A
        vec![5, 2, 1], //Staff 6, AG3, Fixed shifts
        vec![6, 1, 0], //Staff 7, AG2, N/A
        vec![7, 1, 0], //Staff 8, AG2, N/A
        vec![8, 1, 2], //Staff 9, AG2, Off on PH
    ];

    let shifts_info: Vec<Vec<usize>> = vec![
        vec![0, 2, 0], //Unassigned shift (other shift)
        vec![1, 0, 8], //Morning shift M1
        vec![2, 0, 7], //Morning shift M2
        vec![3, 0, 4], //Morning shift M3
        vec![4, 1, 8], //Afternoon shift A1
        vec![5, 1, 7], //Afternoon shift A2
        vec![6, 2, 0], //Day off DO (other shift)
        vec![7, 2, 8], //Public holiday PH (other shift)

    ];

    //let morning_shift: [usize; 3] = [1, 2, 3];
    //let afternoon_shift: [usize; 2] = [4, 5];
    let mut working_shift: [usize; 5] = [1, 2, 3, 4, 5];

    //Define days struct
    let days: Vec<Day> = (0..DAY_NUM).map(|i| Day{
        id: i,
        day_type: days_info[i][1],
        morning_cov: days_info[i][2],
        afternoon_cov: days_info[i][3],
    }).collect();

    //Define staffs struct
    let staffs: Vec<Staff> = (0..staffs_info.len()).map(|i| Staff{
        id: i,
        agency: staffs_info[i][1],
        group: staffs_info[i][2],
    }).collect();

    //Define shifts struct
    let shifts: Vec<Shift> = (0..shifts_info.len()).map(|i| Shift{
        id: i,
        shift_type: shifts_info[i][1],
        shift_duration: shifts_info[i][2],
    }).collect();

    //Initiate staff schedule
    let mut staffs_schedule: Vec<Vec<usize>> = vec![vec![0; DAY_NUM]; STAFF_NUM];

    //Initiate shift schedule
    let mut shifts_schedule: Vec<Vec<usize>> = vec![vec![0; DAY_NUM]; SHIFT_NUM];

    //Logic to assign PH shift
    for i in 0..STAFF_NUM {
        for k in 0..DAY_NUM {
            if (days[k].day_type == 2) && (staffs[i].group == 2) {
                staffs_schedule[i][k] = PH_SHIFT;
            }
        }
    }

    //Logic to randomly assign DO
    for i in 0..STAFF_NUM {
        while !staffs_schedule[i].contains(&DAY_OFF) {
            let rand_idx = rng.random_range(0..7);
            //If the shift schedule is not PH, then assign DO
            if staffs_schedule[i][rand_idx] != PH_SHIFT {
                staffs_schedule[i][rand_idx] = DAY_OFF;}
        }
    }

    //Randomly assign a shift to staffs
    for k in 0..DAY_NUM {
        for i in 0..STAFF_NUM {
            while staffs_schedule[i][k] == 0 {
                if (staffs_schedule[i][k] != PH_SHIFT) && (staffs_schedule[i][k] != DAY_OFF) {
                    &working_shift.shuffle(&mut rng);
                    let gen_shift = working_shift[0];
                    staffs_schedule[i][k] = gen_shift;
                }
            }
        }
    }

    //Update shifts_schedule
    for k in 0..DAY_NUM {
        for j in 0..SHIFT_NUM {
            for i in 0..STAFF_NUM {
                if staffs_schedule[i][k] == j {
                    shifts_schedule[j][k] += 1;
                }
            }
        }
    }

    //Coverage requirements
    println!("Schedule: {:?}", staffs_schedule);
    //println!("{:?}", shifts_schedule);


    //Debug
    let current_sol = Solution::new(staffs_schedule, shifts, days);
    println!("Fitness Value: {:?}", current_sol.fitness_val);
    println!("Worktime Penalty: {:?}", current_sol.WorktimePenalty);
    println!("Afternoon Penalty: {:?}", current_sol.AfternoonPenalty);
    println!("Coverage Penalty: {:?}", current_sol.CoveragePenalty);
    println!("Consecutive Penalty: {:?}", current_sol.ConsecutivePenalty);
}