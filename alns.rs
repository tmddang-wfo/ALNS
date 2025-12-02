use rand::Rng;
use rand::prelude::*;
use crate::structs::{Day, Shift, Staff};
use crate::utils::{DAY_NUM, DAY_OFF, PH_SHIFT, SHIFT_NUM, STAFF_NUM, WORKING_SHIFT};

pub fn generate_init_sched(staffs: &Vec<Staff>, days: &Vec<Day>) -> Vec<Vec<usize>> {
    //Initiate staff schedule
    let mut staffs_schedule: Vec<Vec<usize>> = vec![vec![0; DAY_NUM]; STAFF_NUM];

    //Logic to assign PH shift
    for i in 0..STAFF_NUM {
        for k in 0..DAY_NUM {
            if (days[k].day_type == 2) && (staffs[i].group == 2) {
                staffs_schedule[i][k] = PH_SHIFT;
            }
        }
    }

    //Logic to randomly assign DO
    let mut rng = rand::rng();
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
                    //&WORKING_SHIFT.shuffle(&mut rng);
                    let random_idx = rng.random_range(0..WORKING_SHIFT.len());
                    let gen_shift = WORKING_SHIFT[random_idx] as usize;
                    staffs_schedule[i][k] = gen_shift;
                }
            }
        }
    }
    staffs_schedule
}

pub fn update_shift_sched(staffs_schedule: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    //Initiate shift schedule
    let mut shifts_schedule: Vec<Vec<usize>> = vec![vec![0; DAY_NUM]; SHIFT_NUM];

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
    shifts_schedule
}



