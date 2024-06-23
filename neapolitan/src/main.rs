use std::env::args;
use std::fs::{read_to_string, write};
use std::process;
use serde_json::{from_str, to_string_pretty};
use neapolitan::NodalAnalysisStudyBuilder;

fn main()
{
    let args: Vec<String> = args().collect();
    let mut precision: Option<f64> = None;
    let mut iteration_limit: Option<usize> = None;

    let model_json = match read_to_string(&args[1]) 
    {
        Ok(o) => o,
        Err(e) => 
        {
            println!("[neapolitan].....ERR: could not find the specified filepath!");
            println!("[neapolitan].....ERR: {e}");
            process::exit(1);
        }
    };

    let mut i = 1;
    while i < args.len()
    {
        let arg = &args[i];

        if arg == "--precision" ||
           arg == "-p"
        {
            precision = match args[i + 1].parse()
            {
                Ok(o) => 
                {
                    println!("[neapolitan]......... solver precision is: {o}");
                    Some(o)
                },
                Err(e) =>
                {
                    println!("[neapolitan].....ERR: failed to parse precision argument!");
                    println!("[neapolitan].....ERR: {e}");
                    process::exit(1);
                }
            };

            i += 1;
        }

        else if arg == "--iterations" ||
                arg == "-i"
        {
            iteration_limit = match args[i + 1].parse()
            {
                Ok(o) => 
                {
                    println!("[neapolitan]......... solver iteration limit is: {o}");
                    Some(o)
                },
                Err(e) =>
                {
                    println!("[neapolitan].....ERR: failed to parse iteration limit argument!");
                    println!("[neapolitan].....ERR: {e}");
                    process::exit(1);
                }
            };
            
            i += 1;
        }

        i += 1;
    }

    let model = match from_str(&model_json)
    {
        Ok(o) => o,
        Err(e) =>
        {
            println!("[neapolitan].....ERR: failed to read model from json file!");
            println!("[neapolitan].....ERR: {e}");
            process::exit(1);
        }
    };

    let solution = match NodalAnalysisStudyBuilder::from_model_with_default_config(model)
        .run_study(precision.unwrap_or(0.0001), iteration_limit.unwrap_or(100))
    {
        Ok(o) => o,
        Err(e) => 
        {
            println!("[neapolitan].....ERR: failed to solve the given model!");
            println!("[neapolitan].....ERR: {e}");
            process::exit(1);
        }
    };

    let solution_json = match to_string_pretty(&solution)
    {
        Ok(o) => o,
        Err(e) => 
        {
            println!("[neapolitan].....ERR: failed to format solution file!");
            println!("[neapolitan].....ERR: {e}");
            process::exit(1);
        }
    };

    let solution_file = args[1].replace(".json", ".soln.json");
    match write(solution_file, solution_json) 
    {
        Ok(_) => process::exit(0),
        Err(e) => {
            println!("[neapolitan].....ERR: neapolitan could not write to the output file!");
            println!("[neapolitan].....ERR: {e}");
            process::exit(1);
        }
    }
}