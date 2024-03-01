/// Different errors specific to Nexsys implementations of algorithms.
pub mod errors;
/// Provides `extern "C"` functions for use in other programming languages. Not 
/// intended for use in other Rust projects.
pub mod ffi;
/// Provides tools for parsing text prior to passing to the equation solving engine.
pub mod parsing;
/// Provides data sets of common units and functions for converting between them.
pub mod units;

use std::collections::HashMap;

use geqslib::solve_equation_with_context;
use geqslib::shunting::{new_context, ContextHashMap, ContextLike, Token};
use geqslib::system::{ConstrainResult, get_equation_unknowns, SystemBuilder};

use parsing::compile;

/// Solves a single equation for a single unknown value, returning a `bool` indicating if the solution attempt was successful 
fn try_solve_single_unknown_eqn(eqn_pool: &mut Vec<String>, ctx: &mut ContextHashMap, declared: &mut HashMap<String, [f64; 3]>, log_step: &mut String, margin: f64, limit: usize) -> anyhow::Result<bool>
{
    for (i, equation) in eqn_pool.iter().enumerate()
    {
        let unknowns: Vec<&str> = get_equation_unknowns(&equation, ctx).collect();
        if unknowns.len() != 1
        {
            return Ok(false);
        }

        let var_info: [f64; 3];
        if declared.contains_key(unknowns[0])
        {
            var_info = declared[unknowns[0]];
        }
        else
        {
            var_info = [1.0, f64::NEG_INFINITY, f64::INFINITY];
        }

        let soln = solve_equation_with_context(equation, ctx, var_info[0], var_info[1], var_info[2], margin, limit)?;
        ctx.add_var_with_domain_to_ctx(&soln.0, soln.1, var_info[1], var_info[2]);
        *log_step = format!(
            "Var: {:#?} \nEquation: {}", 
            soln.0, equation
        );
        eqn_pool.remove(i);
        return Ok(true);
    }
    Ok(false)
}

fn try_solve_subsystem_of_equations(eqn_pool: &mut Vec<String>, ctx: &mut ContextHashMap, declared: &mut HashMap<String, [f64; 3]>, log_step: &mut String, margin: f64, limit: usize) -> anyhow::Result<bool>
{
    for (i, equation) in eqn_pool.iter().enumerate()
    {
        let mut builder = SystemBuilder::new(equation, ctx.clone())?;
        let mut eqn_strings = vec![equation.to_owned()];

        for (j, equation) in eqn_pool.iter().enumerate()
        {
            // Skip the current equation we're building a system with 
            // OR an equation that we've already added to the system
            if j == i || eqn_strings.contains(equation)
            {
                continue;
            }

            // println!("Constraint status: {}", builder.is_fully_constrained());

            match builder.try_constrain_with(equation)?
            {
                ConstrainResult::WillConstrain => {
                    eqn_strings.push(equation.to_owned());
                    // println!("constrained with: {}", equation);
                },
                ConstrainResult::WillOverConstrain => break,
                _ => {}
            }
        }
            
        *log_step = format!("{:#?}", eqn_strings);

        // Solve the found constrained system:
        if let Some(mut system) = builder.build_system()
        {
            for (var, var_info) in declared
            {
                system.specify_variable(var, var_info[0], var_info[1], var_info[2]);
            }

            let soln = system.solve(margin, limit)?;
            for (var, val) in soln 
            {
                ctx.add_const_to_ctx(&var, val);
            }
            
            let remaining_eqns: Vec<String> = eqn_pool.iter()
                .filter(|x| !eqn_strings.contains(x) && x != &equation)
                .map(|x| x.to_owned())
                .collect();

            eqn_pool.clear();
            eqn_pool.extend(remaining_eqns);

            // println!("{:#?}", eqn_pool);

            return Ok(true);
        }
    }

    Ok(false)
}

/// Solves a system of equations in plain-text format.
/// For more supported syntax, see `solve_with_preprocessors`
/// 
/// # Example
/// ```
/// ```
pub fn basic_solve(system: &str, ctx: &mut ContextHashMap, declared: &mut HashMap<String, [f64; 3]>, margin: f64, limit: usize) -> anyhow::Result<(Vec<String>, HashMap<String, f64>)>
{
    let mut log = vec![];
    let mut eqn_pool = system.split('\n')
        .filter(|x| x.contains('='))
        .map(|x| x.to_owned())
        .collect();

    loop
    {
        let mut log_step = String::default();
        // Get less expensive solutions:
        if try_solve_single_unknown_eqn(&mut eqn_pool, ctx, declared, &mut log_step, margin, limit)?
        {
            log.push(log_step);
            continue;
        }

        // Dig in and solve a more expensive subsystem:
        if try_solve_subsystem_of_equations(&mut eqn_pool, ctx, declared, &mut log_step, margin, limit)?
        {
            log.push(log_step);
            continue;
        }
        
        break;
    }

    let mut soln_map = HashMap::new(); 
    for (name, val) in ctx
    {
        match val
        {
            Token::Var(v) => {
                soln_map.insert(name.to_owned(), f64::from(*v.borrow()));
            },
            Token::Num(n) => {
                soln_map.insert(name.to_owned(), *n);
            },
            _ => {} 
        }
    }

    Ok((log, soln_map))
}

/// Solves a system of equations with additional syntax used to indicate 
/// unit conversions, constant known values, nicer if statements, and more.
/// 
/// # Example
/// ```
/// use nexsys::solve_with_preprocessors;
/// 
/// let system = r#"
/// keep x on [0, 100]
/// guess 3 for y
/// const nine = 9
/// 
/// x + y = nine
/// x - y = 4
/// 
/// if x > y:
///     i = 1
/// else:
///     i = -1
/// end
/// "#;
/// 
/// let (_log, soln) = solve_with_preprocessors(system, 0.0001, 100)
///     .expect("failed to solve system!");
/// 
/// assert!((f64::from(soln["x"]) - 6.5).abs() < 0.001);
/// assert!((f64::from(soln["y"]) - 2.5).abs() < 0.001);
/// assert!((f64::from(soln["i"]) - 1.0).abs() < 0.001);
/// ```
pub fn solve_with_preprocessors(system: &str, margin: f64, limit: usize) -> anyhow::Result<(Vec<String>, HashMap<String, f64>)>
{
    let mut ctx = new_context(); 
    let mut declared = HashMap::new();
    let compiled = compile(system, &mut ctx, &mut declared)?;

    basic_solve(&compiled, &mut ctx, &mut declared, margin, limit)
}