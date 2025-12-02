use std::collections::HashMap;
use rand::prelude::*;
mod operators;
mod structs;
mod utils;
mod alns;

use structs::{Solution, Shift, Staff, Day};
use utils::*;
use alns::*;

use crate::operators::destroy_ops;
use crate::operators::repair_ops;

fn main() {
    let days_info: Vec<Vec<usize>> = vec![
        vec![0, 0, 4, 3],
        vec![1, 0, 4, 3],
        vec![2, 2, 3, 3],
        vec![3, 2, 3, 3],
        vec![4, 0, 4, 3],
        vec![5, 1, 3, 3],
        vec![6, 1, 3, 3]
    ];

    //Define days struct
    let days: Vec<Day> = (0..DAY_NUM).map(|i| Day{
        id: i,
        day_type: days_info[i][1],
        morning_cov: days_info[i][2],
        afternoon_cov: days_info[i][3],
    }).collect();


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

    //Define staffs struct
    let staffs: Vec<Staff> = (0..staffs_info.len()).map(|i| Staff{
        id: i,
        agency: staffs_info[i][1],
        group: staffs_info[i][2],
    }).collect();

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

    //Define shifts struct
    let shifts: Vec<Shift> = (0..shifts_info.len()).map(|i| Shift{
        id: i,
        shift_type: shifts_info[i][1],
        shift_duration: shifts_info[i][2],
    }).collect();


    //Coverage requirements
    //println!("Schedule: {:?}", staffs_schedule);
    //println!("{:?}", shifts_schedule);


    //Debug
    //let current_sol = Solution::new(staffs_schedule, shifts, days);
    //println!("Fitness Value: {:?}", current_sol.fitness_val);
    //println!("Worktime Penalty: {:?}", current_sol.WorktimePenalty);
    //println!("Afternoon Penalty: {:?}", current_sol.AfternoonPenalty);
    //println!("Coverage Penalty: {:?}", current_sol.CoveragePenalty);
    //println!("Consecutive Penalty: {:?}", current_sol.ConsecutivePenalty);

    let staff_schedule = generate_init_sched(&staffs, &days);
    let init_solution = Solution::new(staff_schedule, &shifts, &days);

    let destroy_sol = destroy_ops::worst_destroy(&init_solution);
    let removed_staff = destroy_sol.removed_staff;
    let partial_schedule = destroy_sol.partial_schedule;
    println!("Partial Sol: {:?}", partial_schedule);

    let current_sol = repair_ops::greedy_repair(&partial_schedule,
                                                removed_staff, &staffs,
                                               &shifts, &days);
    println!("Repair Sol: {:?}", current_sol.staffs_schedule);

    //println!("init_solution: {:?}", partial_schedule);

}