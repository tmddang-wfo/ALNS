use std::env::current_exe;
use crate::utils::*;
use crate::structs::*;
use std::cmp;

pub fn check_coverage(partial: &Vec<Vec<usize>>, days: &Vec<Day>, current_day: usize) -> Vec<usize> {
    let mut morning_cov: i32 = 0;
    let mut afternoon_cov: i32 = 0;
    let mut morning_shortage: i32 = 0;
    let mut afternoon_shortage: i32 = 0;
    let mut shortage_shift: Vec<usize> = vec![];

    for i in 0..STAFF_NUM {
        if MORNING_SHIFT.contains(&(partial[i][current_day] as i32)) {
            morning_cov += 1;
        } else if AFTERNOON_SHIFT.contains(&(partial[i][current_day] as i32)) {
            afternoon_cov += 1;
        }
    }
    morning_shortage = cmp::max(0, days[current_day].morning_cov as i32 - morning_cov);
    afternoon_shortage = cmp::max(0, days[current_day].afternoon_cov as i32 - afternoon_cov);

    if morning_shortage > afternoon_shortage {
        for l in 0..MORNING_SHIFT.len() {
            shortage_shift.push(MORNING_SHIFT[l] as usize)
        }
    } else if afternoon_shortage > morning_shortage {
        for l in 0..AFTERNOON_SHIFT.len() {
            shortage_shift.push(AFTERNOON_SHIFT[l] as usize)
        }
    } else {
            shortage_shift.push(DAY_OFF);
        }
    shortage_shift
    }

pub fn check_worktime(repair_sol: &Vec<usize>,
                      shortage_list: &Vec<usize>,
                      shifts: &Vec<Shift>,
                      current_day: usize) -> usize {

    //let mut target_shift: Vec<usize> = vec![];
    let mut expected_worktime: Vec<usize> = vec![0; shortage_list.len()];
    let mut gap_worktime: Vec<usize> = vec![0; shortage_list.len()];

    for l in 0..shortage_list.len() {
        expected_worktime[l] += shifts[l].shift_duration;
        for k in 0..DAY_NUM{
            if k == current_day {continue;}
            expected_worktime[l] += shifts[k].shift_duration
        }
        gap_worktime[l] = 44usize.abs_diff(expected_worktime[l]);
    }
    let (target_shift, &min_gap) = gap_worktime
        .iter()
        .enumerate()
        .min_by_key(|&(_, val)| val)
        .unwrap();

    //Return the target shift
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
        if days[k].id == 2 && staffs[removed_staff].group == 2 {
            repair_sol[k] = PH_SHIFT
        } else {
            let shortage_shift = check_coverage(&current_sol, days, k);
            let target_shift = check_worktime(&repair_sol, &shortage_shift, shifts, k);
            repair_sol[k] = target_shift;
        }
        //Update current solution
        current_sol[removed_staff][k] = repair_sol[k]
    }

    //current_sol
    Solution::new(current_sol, shifts, days)
}
