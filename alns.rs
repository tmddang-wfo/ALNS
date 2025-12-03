#![allow(non_snake_case)]
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::prelude::*;
use crate::operators::destroy_ops::{random_destroy, worst_destroy};
use crate::operators::repair_ops::greedy_repair;
use crate::structs::*;
use crate::utils::*;
use std::f64::consts::E;

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

struct ALNSConfig {
    iterations: usize,
    reaction_factor: f64,
    start_temp: f64,
    cooling_rate: f64,
}

struct OperatorWeights {
    destroy_random: f64,
    destroy_worst: f64,
    repair_greedy: f64,
}

impl OperatorWeights {
    fn new() -> Self{
        Self{
            destroy_random: 1.0,
            destroy_worst: 1.0,
            repair_greedy: 1.0,
        }
    }

    fn select_destroy(&self, rng: &mut ThreadRng) -> DestroyOp {
        let total = self.destroy_worst + self.destroy_random;
        let val = rng.random::<f64>()*total;
        if val <self.destroy_random {
            DestroyOp::Random
        } else {
            DestroyOp::Worst
        }
    }

    fn select_repair(&self, rng: &mut ThreadRng) -> RepairOp {
        RepairOp::Greedy
    }
}

pub fn solve_alns(staffs: &Vec<Staff>, shifts: &Vec<Shift>, days: &Vec<Day>, config: ALNSConfig) -> Solution {
    let mut rng = rand::rng();

    //Create initial solution
    let initial_sol = generate_init_sched(&staffs, &days);
    let mut current_sol = Solution::new(initial_sol, shifts, days);
    let mut best_sol = current_sol.clone();

    //Initiate ALNS state
    let mut temperature = config.cooling_rate;
    let mut weight = OperatorWeights::new();
    let mut scores = OperatorWeights{destroy_random: 0.0, destroy_worst: 0.0, repair_greedy: 0.0};
    let mut counts = OperatorWeights{destroy_random: 0.0, destroy_worst: 0.0, repair_greedy: 0.0};
    println!("Initial cost: {:.2}", current_sol.fitness_val);

    for i in 0..config.iterations {
        //Select operators
        let destroy_op = weight.select_destroy(&mut rng);
        let repair_op = weight.select_repair(&mut rng);

        //Apply operators
        let destroy_res = match destroy_op {
            DestroyOp::Random => random_destroy(&current_sol),
            DestroyOp::Worst => worst_destroy(&current_sol),
        };

        let candidate_sol = match repair_op{
            RepairOp::Greedy => greedy_repair(&destroy_res.partial_schedule, destroy_res.removed_staff, staffs, shifts, days),
        };

        //Evaluate new solution
        let delta = candidate_sol.fitness_val - current_sol.fitness_val;
        let mut op_score = 0.0;

        //Case 1 Find the new global best solution
        if candidate_sol.fitness_val < best_sol.fitness_val {
            best_sol = candidate_sol.clone();
            current_sol = candidate_sol;
            op_score = GLOBAL_BEST;
            println!("Iter {}: New best found! Cost: {:2}", i, best_sol.fitness_val);
        }

        //Case 2 Find the new solution better than current one
        else if delta < 0.0 {
            current_sol = candidate_sol;
            op_score = LOCAL_BEST;
        }

        //Case Find the new feasible solution
        else {
            let probabiltity = E.powf(-delta/temperature);
            if rng.random::<f64>() < probabiltity {
                current_sol = candidate_sol;
                op_score = NEW_SOLUTION;
            }
        }

        match destroy_op {
            DestroyOp::Random => {
                scores.destroy_random += op_score;
                counts.destroy_random += 1.0;
            }
            DestroyOp::Worst => {
                scores.destroy_worst += op_score;
                counts.destroy_worst += 1.0;
            }
        }

        match repair_op {
            RepairOp::Greedy => {
                scores.repair_greedy += op_score;
                counts.repair_greedy += 1.0;
            }
        }

        //Update adaptive weights
        if (i+1) % UPDATE_FREQUENCY == 0 {
            //Update random destroy weight
            if counts.destroy_random > 0.0 {
                let performance = scores.destroy_random / counts.destroy_random;
                weight.destroy_random = (1.0 - config.reaction_factor) * weight.destroy_random
                + config.reaction_factor*performance;
            }

            //Update worst destroy weight
            if counts.destroy_worst > 0.0 {
                let performance = scores.destroy_worst / counts.destroy_worst;
                weight.destroy_worst = (1.0 - config.reaction_factor) * weight.destroy_worst
                    + config.reaction_factor*performance;
            }

            //Reset scores/counts
            scores = OperatorWeights{destroy_random: 0.0, destroy_worst: 0.0, repair_greedy: 0.0};
            counts = OperatorWeights{destroy_random: 0.0, destroy_worst: 0.0, repair_greedy: 0.0};
        }

        //Cooldown temperatures
        temperature = config.cooling_rate;
    }
    //Return best global solution
    best_sol

}



