use std::ptr::null;
use std::{collections::HashMap, ffi::CStr, panic::catch_unwind};
use std::ffi::{c_char, c_double, c_int, CString, c_uint, c_void};
use geqslib::shunting::ContextHashMap;

pub use geqslib::ffi::free_solution_string;
pub use geqslib::ffi::{add_const_to_ctx, free_context_hash_map, new_context_hash_map, new_default_context_hash_map};

macro_rules! copy_to_owned_string {
    ($s: expr) => {
        String::from_utf8_lossy(
            CStr::from_ptr($s).to_bytes()
        ).to_owned()
    };
} 

#[no_mangle]
pub extern "C" fn new_declared_hash_map() -> *mut c_void
{
    Box::into_raw(
        Box::new(
            HashMap::<String, [f64; 3]>::new()
        )
    ) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn add_declared_variable(declared: *mut c_void, var: *const c_char, guess: c_double, min: c_double, max: c_double) -> c_int
{
    let res = catch_unwind(|| {
        let var_to_add = copy_to_owned_string!(var);
        (*(declared as *mut HashMap<String, [f64; 3]>))
            .insert(
                var_to_add.to_string(), 
                [guess, min, max]
            );
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn basic_solve(system: *const c_char, ctx: *mut c_void, declared: *mut c_void, margin: c_double, limit: c_uint) -> *const c_char
{
    let res = catch_unwind(|| {
        let rust_system = copy_to_owned_string!(system);
        
        let maybe_soln = crate::basic_solve(
            &rust_system, 
            &mut *(ctx as *mut ContextHashMap), 
            &mut *(declared as *mut HashMap<String, [f64; 3]>), 
            margin, 
            limit as usize
        );

        match maybe_soln 
        {
            Ok((log, soln)) => {
                let steps_str = log.join("\n");
                let soln_str = soln.iter()
                    .fold(String::new(), |mut acc, (x, y)| { 
                        acc.push_str(&format!("{} {:#?}", x, y));
                        acc
                    });
                CString::new(format!("{{ log: \"{:#?}\", \nsoln: \"{:#?}\" }}", steps_str, soln_str))
                    .expect("failed to create C-compatible solution error string!")
                    .into_raw()
            },
            Err(_) => null() as *const c_char,
        }
    });
    
    res.unwrap_or(null() as *const c_char)
}

#[no_mangle]
pub unsafe extern "C" fn solve_with_preprocessors(system: *const c_char, margin: c_double, limit: c_uint) -> *const c_char
{
    let res = catch_unwind(|| {
        let c_str = CStr::from_ptr(system);
        let rust_system =  String::from_utf8_lossy(c_str.to_bytes()).to_owned();

        let maybe_soln = crate::solve_with_preprocessors(
            &rust_system, 
            margin, 
            limit as usize
        );

        match maybe_soln
        {
            Ok((log, soln)) => {
                let steps_str = log.join("\n");
                let soln_str = soln.iter()
                    .fold(String::new(), |mut acc, (x, y)| {
                        acc.push_str(&format!("{} {:#?}", x, y));
                        acc
                    });
                CString::new(format!("{{ log: \"{:#?}\", \nsoln: \"{:#?}\" }}", steps_str, soln_str))
                    .expect("failed to create C-compatible solution error string!")
                    .into_raw()
            },
            Err(_) => null() as *const c_char,
        }
    });

    res.unwrap_or(null() as *const c_char)
}

#[no_mangle]
pub unsafe extern "C" fn free_declared_hash_map(declared: *mut c_void)
{
    let _dropper = Box::from_raw(declared as *mut HashMap<String, [f64; 3]>);
}