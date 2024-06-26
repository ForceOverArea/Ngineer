use std::collections::HashMap;
use std::hash::Hash;
use gmatlib::Matrix;
use crate::errors::NewtonRaphsonSolverError;

const _DX_: f64 = 0.001; 

/// A basic implementation of the 1-D newton-raphson method.
/// This function allows the caller to choose an initial guess value,
/// a margin of error, and a maximum number of iterations prior to 
/// returning a value. 
/// 
/// This function also guarantees that the root, if found, is
/// within `margin` of the actual root AND that `f(guess)` is
/// within `margin` of `0.0`.
/// 
/// # Example
/// ```
/// use std::io::Error;
/// use geqslib::newton::newton_raphson;
/// 
/// fn x_squared(x: f64) -> Result<f64, Error>
/// {
///     Ok(x * x)
/// }
/// 
/// let x = newton_raphson(x_squared, 1.0, 0.0001, 100).unwrap();
/// 
/// assert!((x - 0.0001).abs() < 0.001); // solution is APPROXIMATE. In this case, very close to 0.
/// ```
pub fn newton_raphson<E>(f: impl Fn(f64) -> Result<f64, E>, guess: f64, margin: f64, limit: usize) -> anyhow::Result<f64>
where anyhow::Error: From<E>
{
    // Catch illegal margin of error
    if margin <= 0.0
    {
        return Err(NewtonRaphsonSolverError::NegativeMargin.into());
    }

    // Allow user to manually prevent stack overflow
    if limit == 0
    {
        return Err(NewtonRaphsonSolverError::ReachedIterationLimit.into());
    }

    let y = f(guess)?;
    let y_prime = (f(guess + _DX_)? - y) / _DX_;
    let delta = y / y_prime;

    // Check if we are sufficiently close to the solution:
    if y.abs() <= margin && delta <= margin // ...in both the y AND x directions...
    {
        return Ok(guess); // ...if so, exit early
    }

    // ...if not, calculate next iteration
    let next_guess = guess - delta;

    newton_raphson(f, next_guess, margin, limit - 1)
}

/// A basic implementation of the Newton-Raphson method for multivariate
/// systems. This function allows the caller to specify an initial guess 
/// vector as a `HashMap<String, f64>`, a margin of error, and a maximum 
/// number of iterations prior to returning a value.
/// 
/// This function also guarantees that the root, if found, is within `margin` 
/// of the actual root AND that F(`guess`) has a magnitude within `margin` of 
/// `0.0` where 'F' is the "system vector" containing f1, f2, ..., fn.
/// 
/// # Example
/// ```
/// use std::io::Error;
/// use std::collections::HashMap;
/// use geqslib::newton::multivariate_newton_raphson;
/// 
/// fn f1(x: &HashMap<String, f64>) -> Result<f64, Error>
/// {
///     Ok(x["x"] + x["y"] - 9.0)
/// }
/// 
/// fn f2(x: &HashMap<String, f64>) -> Result<f64, Error>
/// {
///     Ok(x["x"] - x["y"] - 4.0)
/// }
/// 
/// let mut guess = HashMap::from([
///     ("x".to_string(), 7.0),
///     ("y".to_string(), 2.0),
/// ]);
/// 
/// let soln = multivariate_newton_raphson(
///     vec![f1, f2],
///     &mut guess,
///     0.0001,
///     50,
/// ).unwrap();
/// 
/// assert!(soln["x"] - 6.5 < 0.0001);
/// assert!(soln["y"] - 2.5 < 0.0001);
/// ```
pub fn multivariate_newton_raphson<K, E>(f: Vec<impl Fn(&HashMap<K, f64>) -> Result<f64, E>>, guess: &mut HashMap<K, f64>, margin: f64, limit: usize) -> anyhow::Result<&mut HashMap<K, f64>>
where 
    K: Clone + Eq + Hash,
    anyhow::Error: From<E>,
{
    // Catch illegal margin of error
    if margin <= 0.0
    {
        return Err(NewtonRaphsonSolverError::NegativeMargin.into());
    }

    // Allow user to manually prevent stack overflow
    if limit == 0
    {
        return Err(NewtonRaphsonSolverError::ReachedIterationLimit.into());
    }

    // Establish system size and ensure number of functions == number of vars
    let n = f.len();
    if guess.len() != n
    {
        return Err(NewtonRaphsonSolverError::ImproperlyConstrainedSystem.into());
    }

    // Build jacobian w/ F(X) values... we will mutate them to F'(X) later
    let mut elements = vec![];
    for func in &f 
    {
        let row = &mut vec![func(guess)?; n];
        elements.append(row);
    }
    let mut jacobian = Matrix::from_vec(n, elements)?; // <- should this be a panic on failure?
    
    // Copy keys to iterate over hashmap
    let vars = Vec::from_iter(
        guess.keys().map(|x| x.to_owned())
    );

    // Correct jacobian values and invert
    for (j, var) in vars.iter().enumerate()
    {
        if let Some(v) = guess.get_mut(var)
        {
            *v += _DX_;
        } 
        for i in 0..n
        {
            // mutate values to partial derivatives
            jacobian[(i, j)] = (f[i](guess)? - jacobian[(i, j)]) / _DX_;
        }
        if let Some(v) = guess.get_mut(var)
        {
            *v -= _DX_;
        } 
    }

    jacobian.try_inplace_invert()?;

    // Calculate current error
    let mut y = vec![0.0; n];
    for i in 0..n
    {
        y[i] = f[i](guess)?;
    }
    let error = y.iter()
        .map(|v| v.powi(2))
        .sum::<f64>();

    // Calculate change vector and its magnitude
    let deltas = jacobian * Matrix::from_col_vec(y);
    let change = deltas.iter()
        .map(|d| d.powi(2))
        .sum::<f64>()
        .sqrt();

    if error <= margin && change <= margin
    {
        return Ok(guess);
    }

    // Build next guess vector
    for (i, var) in vars.iter().enumerate().take(n)
    {
        if let (Some(guess_val), delta) = (guess.get_mut(var), deltas[(i, 0)])
        {
            *guess_val -= delta;
        }
    }

    // COMPUTER, ENHANCE!
    multivariate_newton_raphson(f, guess, margin, limit - 1)
}