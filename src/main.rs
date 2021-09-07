use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
use std::time::Instant;
use std::vec::Vec;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Record {
    number: i128,
    steps: i32,
    historical: Vec<i128>
}

fn recursive(mut number: i128, mut steps: i32, mut historical: Vec<i128>) -> (i32, Vec<i128>) {
    //Base case
    if number == 1 {
        return (steps.clone(),historical.clone())
    }
    match number%2 {
        0 => {
            number = number/2;
            steps +=1;
            historical.push(number);
            recursive(number, steps,historical)
        }
        _ => {
            number = number*3+1;
            steps +=1;
            historical.push(number);
            recursive(number, steps,historical)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let iterations:i128 = 10;
    let base: i128 = 10000;
    for _n in 1..iterations {
        let mut before = Instant::now();
        let sl: i128 = base*_n;
        let il: i128 = sl-base+1;
        let vec: Vec<i128> = (il..sl).collect();
        let mut res: Vec<Record> = Vec::new();
        for i in vec {
            let row = recursive(i,0, vec![i]);
            res.push(Record {number: i, steps: row.0, historical: row.1});
        }
				let _computed = before.elapsed();
				before = Instant::now();
        let f = File::create(format!("./data/iter_{:?}_output.json", _n)).expect("Unable to create file");
        let bw = BufWriter::new(f);
        serde_json::to_writer(bw, &res)?;
        println!("Iteration {:?} Execution time: {:.2?}  Writing File: {:.2?}", _n, _computed, before.elapsed());     
    }
    Ok(())
}
