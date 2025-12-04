#![allow(non_snake_case)]
mod operators;
mod structs;
mod utils;
mod alns;

use structs::{Solution, Shift, Staff, Day};
use utils::*;
use alns::*;

use calamine::{open_workbook, Data, Reader, Xlsx};
use std::error::Error;

fn extract_row(row: &[Data]) -> Vec<i32> {
    row.iter()
        .filter_map(|cell| match cell {
            Data::Int(i) => Some(*i as i32),
            Data::Float(f) => Some(*f as i32),
            _ => None,
        })
        .collect()
}

fn read_excel(path: &str, sheet_name: &str) -> Result<(Vec<Vec<usize>>), Box<dyn Error>> {

    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let range = workbook
        .worksheet_range(sheet_name)?;

        // Extract 4 rows at once using helper function
    let mut iter = range.rows();

    let idx_vec= extract_row(iter.next().unwrap_or(&[]));
    let type_vec= extract_row(iter.next().unwrap_or(&[]));
    let morning_vec= extract_row(iter.next().unwrap_or(&[]));
    let afternoon_vec= extract_row(iter.next().unwrap_or(&[]));

    // Construct days_info
    let mut days_info: Vec<Vec<usize>> = Vec::new();

    for k in 0..28 {
        days_info.push(vec![
            idx_vec[k] as usize,
            type_vec[k] as usize,
            morning_vec[k] as usize,
            afternoon_vec[k] as usize,
        ]);
        }
    Ok((days_info))
}


fn main() {
    let path = "C:/Users/trinh/RustroverProjects/ExcelReader/monthly_data.xlsx";
    let sheet_name = "input_data";
    let monthly_data = read_excel(&path, &sheet_name).unwrap();

    let mut weeks: Vec<Vec<Day>> = Vec::new();

    let mut first_day: usize = 0;
    let mut last_day: usize = DAY_NUM;

    for w in 0..WEEK_NUM {
        let mut days_info: Vec<Vec<usize>> = Vec::new();

        for k in first_day..last_day {
            days_info.push(monthly_data[k].to_vec());
        }

        //Define days struct
        let days: Vec<Day> = (0..DAY_NUM).map(|i| Day{
            id: i,
            order: days_info[i][0],
            day_type: days_info[i][1],
            morning_cov: days_info[i][2],
            afternoon_cov: days_info[i][3],
        }).collect();

        weeks.push(days);

        first_day += DAY_NUM;
        last_day += DAY_NUM;
    }

    let days = &weeks[0];

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



    //Run heuristics
    let final_sol = solve_alns(&staffs, &shifts, &days);

    //Output log
    println!("-----------------------Solving completed-----------------------------------");
    println!("Final solution");
    for row in &final_sol.staffs_schedule {
        println!("{:?}", row);}
    println!("Final best cost: {:?}", final_sol.fitness_val);
}


