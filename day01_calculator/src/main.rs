use std::{env, io::Error};

fn main() -> Result<(),Error>{
    // Taking arguments as input,
    let args:Vec<String> = env::args().collect();

    let (part1,part2) = (args[2].trim().parse::<i32>().unwrap(),args[3].trim().parse::<i32>().unwrap());
    match args[1].as_str() {
        "sum" => {
            let sum = part1 + part2;
            println!("The Sum is : {}", sum);
            Ok(())
        },

        "sub" => {
            let sub = part1 + part2;
            println!("The Difference is : {}", sub);
            Ok(())
        },

        "multi" => {
            let multi = part1 * part2;
            println!("The Product is : {}",multi);
            Ok(())
        },
        
        "div" => {
            if part2 != 0 {
               let q= part1 as f64 / part2 as f64;
                println!("The Quotient is : {}",q);
                Ok(())
            } else {
                Err(Error::new(std::io::ErrorKind::InvalidInput, "Maths Error can't divide by Zero"))
            }
        },

        _ => {
            Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid Operator"))
        }
    }
}
