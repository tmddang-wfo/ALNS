#![allow(non_snake_case)]
use std::env::current_exe;
use crate::utils::*;
use crate::structs::*;
use std::cmp;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::rng;

pub fn check_coverage(partial: &Vec<Vec<usize>>, removed_staff: usize,
                      shifts: &Vec<Shift>, days: &Vec<Day>,
                      current_day: usize) -> usize {
    let mut morning_cov: i32 = 0;
    let mut afternoon_cov: i32 = 0;
    let mut morning_shortage: i32 = 0;
    let mut afternoon_shortage: i32 = 0;
    let mut shortage_shift: Vec<usize> = vec![];
    let mut shortage_status: bool = Default::default(); //Default shortage value is false
    let mut assign_dayoff: bool = Default::default(); //Default day off is false
    let mut target_shift: usize = 0; //Default target shift is 0: unassigned shift

    for i in 0..STAFF_NUM {
        if MORNING_SHIFT.contains(&(partial[i][current_day] as i32)) {
            morning_cov += 1;
        } else if AFTERNOON_SHIFT.contains(&(partial[i][current_day] as i32)) {
            afternoon_cov += 1;
        }
    }
    morning_shortage = cmp::max(0, days[current_day].morning_cov as i32 - morning_cov);
    afternoon_shortage = cmp::max(0, days[current_day].afternoon_cov as i32 - afternoon_cov);

    //Check if there is shortage occurs
    if morning_shortage == 0 && afternoon_shortage == 0 {
        shortage_status = false;
    } else {shortage_status = true;}

    //Assign shift if shortage occurs
    if shortage_status == true {

        //Case require morning shift
        if morning_shortage > afternoon_shortage {
            for l in 0..MORNING_SHIFT.len() {
                shortage_shift.push(MORNING_SHIFT[l] as usize)
            }
        //Case requires afternoon shift
        } else if afternoon_shortage > morning_shortage {
            for l in 0..AFTERNOON_SHIFT.len() {
                shortage_shift.push(AFTERNOON_SHIFT[l] as usize)
            }
        //Case where both morning and afternoon are in shortage the same amount
        } else {
            for l in 0..ALL_SHIFT.len() {
                shortage_shift.push(ALL_SHIFT[l] as usize);
            }
        }

    //Check if assign DO already
    } else {
        if partial[removed_staff].contains(&DAY_OFF) {
            for l in 0..ALL_SHIFT.len() {
                shortage_shift.push(ALL_SHIFT[l] as usize);
            }
        } else {
            assign_dayoff = true;
        }
    }

    //Final check if target shift is already assigned (to DO)
    if assign_dayoff == true {
        target_shift = DAY_OFF;
    } else {
        target_shift = check_worktime(&shortage_shift, shifts, current_day);
    }

    //Return target shift
    target_shift
    }

pub fn check_worktime(shift_list: &Vec<usize>,
                      shifts: &Vec<Shift>,
                      current_day: usize) -> usize {

    //let mut target_shift: Vec<usize> = vec![];
    let mut expected_worktime: Vec<usize> = vec![0; shift_list.len()];
    let mut gap_worktime: Vec<usize> = vec![0; shift_list.len()];

    for l in 0..shift_list.len() {
        expected_worktime[l] += shifts[l].shift_duration;
        for k in 0..DAY_NUM{
            if k == current_day {continue;}
            expected_worktime[l] += shifts[k].shift_duration
        }
        gap_worktime[l] = 44usize.abs_diff(expected_worktime[l]);
    }
    let (shift_idx, &min_gap) = gap_worktime
        .iter()
        .enumerate()
        .min_by_key(|&(_, val)| val)
        .unwrap();

    //Return the target shift
    let target_shift = shift_list[shift_idx];
    target_shift
}

pub fn greedy_repair(partial: &Vec<Vec<usize>>,
                     removed_staff: usize,
                     staffs: &Vec<Staff>,
                     shifts: &Vec<Shift>,
                     days: &Vec<Day>) -> Solution {

    let mut current_sol = partial.clone();
    let mut repair_sol = partial[removed_staff].clone();


    for k in 0..DAY_NUM{
        //Check PH assignment first
        if days[k].day_type == 2 && staffs[removed_staff].group == 2 {
            repair_sol[k] = PH_SHIFT
        } else {
            let target_shift = check_coverage(&current_sol, removed_staff, shifts, days, k);
            repair_sol[k] = target_shift;
        }
        //Update current solution
        current_sol[removed_staff][k] = repair_sol[k]
    }

    //current_sol
    Solution::new(current_sol, shifts, days)
}


pub fn reverse_greedy_repair(partial: &Vec<Vec<usize>>,
                     removed_staff: usize,
                     staffs: &Vec<Staff>,
                     shifts: &Vec<Shift>,
                     days: &Vec<Day>) -> Solution {

    let mut current_sol = partial.clone();
    let mut repair_sol = partial[removed_staff].clone();


    for k in (0..DAY_NUM).rev() {
        //Check PH assignment first
        if days[k].day_type == 2 && staffs[removed_staff].group == 2 {
            repair_sol[k] = PH_SHIFT
        } else {
            let target_shift = check_coverage(&current_sol, removed_staff, shifts, days, k);
            repair_sol[k] = target_shift;
        }
        //Update current solution
        current_sol[removed_staff][k] = repair_sol[k]
    }

    //current_sol
    Solution::new(current_sol, shifts, days)
}

pub fn random_greedy_repair(partial: &Vec<Vec<usize>>,
                             removed_staff: usize,
                             staffs: &Vec<Staff>,
                             shifts: &Vec<Shift>,
                             days: &Vec<Day>) -> Solution {

    let mut current_sol = partial.clone();
    let mut repair_sol = partial[removed_staff].clone();

    let mut shuffle_days: Vec<usize> = (0..DAY_NUM).collect();
    let mut rng = rng();
    shuffle_days.shuffle(&mut rng);


    for &k in shuffle_days.iter() {
        //Check PH assignment first
        if days[k].day_type == 2 && staffs[removed_staff].group == 2 {
            repair_sol[k] = PH_SHIFT
        } else {
            let target_shift = check_coverage(&current_sol, removed_staff, shifts, days, k);
            repair_sol[k] = target_shift;
        }
        //Update current solution
        current_sol[removed_staff][k] = repair_sol[k]
    }

    //current_sol
    Solution::new(current_sol, shifts, days)
}