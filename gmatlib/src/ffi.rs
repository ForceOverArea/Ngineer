use std::ffi::{c_double, c_uint, c_void};
use std::mem;
use std::panic::catch_unwind;
use std::ptr::null_mut;
use crate::{Matrix, MatrixInversionError};

#[no_mangle]
pub extern "C" fn new_double_matrix(rows: c_uint, cols: c_uint) -> *mut c_void
{
    // We need to use catch_unwind to prevent UB if caller exceeds isize::MAX bytes
    let res = catch_unwind(|| {
        let a = Box::new(Matrix::<c_double>::new(rows as usize, cols as usize));
        Box::into_raw(a) as *mut c_void
    });
    
    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn new_double_identity_matrix(n: c_uint) -> *mut c_void
{
    let res = catch_unwind(|| {
        let a = Box::new(Matrix::<c_double>::new_identity(n as usize));
        Box::into_raw(a) as *mut c_void
    });
    
    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn inplace_row_swap(ptr: *mut c_void, r1: c_uint, r2: c_uint) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        a.inplace_row_swap(r1 as usize, r2 as usize);
        
        mem::forget(a); // Prevent drop that would deallocate the matrix data
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn inplace_row_scale(ptr: *mut c_void, row: c_uint, scalar: c_double) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        a.inplace_row_scale(row as usize, scalar);

        mem::forget(a); // Prevent drop that would deallocate the matrix data
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn inplace_scale(ptr: *mut c_void, scalar: c_double) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        a.inplace_scale(scalar);

        mem::forget(a); // Prevent drop that would deallocate the matrix data
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn inplace_row_add(ptr: *mut c_void, r1: c_uint, r2: c_uint) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        a.inplace_row_add(r1 as usize, r2 as usize);

        mem::forget(a); // Prevent drop that would deallocate the matrix data
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn inplace_scaled_row_add(ptr: *mut c_void, r1: c_uint, r2: c_uint, scalar: c_double) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        a.inplace_scaled_row_add(r1 as usize, r2 as usize, scalar);

        mem::forget(a); // Prevent drop that would deallocate the matrix data
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn multiply_matrix(ptr_a: *mut c_void, ptr_b: *mut c_void) -> *mut c_void
{
    let res = catch_unwind(|| {
        let (a, b) = unsafe 
        {(
            Box::from_raw(ptr_a as *mut Matrix<c_double>),
            Box::from_raw(ptr_b as *mut Matrix<c_double>),
        )};
    
        let ab = match a.multiply_matrix(&b) 
        {
            Ok(x)  => Box::new(x),
            Err(_) => return null_mut(), // return early and indicate failure via NULL
        };
    
        mem::forget(a); // Prevent drop that would deallocate matrix data. We don't inform the
        mem::forget(b); // caller that a or b will be deallocated, so we shouldn't do it here.
    
        Box::into_raw(ab) as *mut c_void
    });

    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn augment_with(ptr_a: *mut c_void, ptr_b: *mut c_void) -> *mut c_void
{
    let res = catch_unwind(|| {
        let (a, b) = unsafe 
        {(
            Box::from_raw(ptr_a as *mut Matrix<c_double>),
            Box::from_raw(ptr_b as *mut Matrix<c_double>),
        )};
    
        let ab = match a.augment_with(&b) 
        {
            Ok(x)  => Box::new(x),
            Err(_) => return null_mut(), // return early and indicate failure via NULL
        };
    
        mem::forget(a); // Prevent drop that would deallocate matrix data. We don't inform the
        mem::forget(b); // caller that a or b will be deallocated, so we shouldn't do it here.
    
        Box::into_raw(ab) as *mut c_void
    });
    
    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn subset(ptr: *mut c_void, r1: c_uint, c1: c_uint, r2: c_uint, c2: c_uint) -> *mut c_void
{
    let res = catch_unwind(|| {
        let a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        let b = Box::new(a.subset(r1 as usize, c1 as usize, r2 as usize, c2 as usize));
        mem::forget(a); // Prevent drop that would deallocate matrix data.
        Box::into_raw(b) as *mut c_void
    });

    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn trace(ptr: *mut c_void) -> c_double
{
    let res = catch_unwind(|| {
        let a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        let trace = match a.trace()
        {
            Ok(t) => t,
            Err(_) => c_double::NAN
        };

        mem::forget(a); // Prevent drop that would deallocate the matrix data

        trace
    });

    match res 
    {
        Ok(t)  => t as c_double,
        Err(_) => c_double::MIN,
    }
}

#[no_mangle]
pub extern "C" fn transpose(ptr: *mut c_void) -> *mut c_void
{
    let res = catch_unwind(|| {
        let a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        let b = Box::new(a.transpose());
        mem::forget(a);

        Box::into_raw(b) as *mut c_void
    });

    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn try_inplace_invert(ptr: *mut c_void) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        let status = match a.try_inplace_invert()
        {
            Ok(_)  => c_uint::MAX,
            Err(e) => 
            {
                match e.downcast()
                {
                    Ok(MatrixInversionError::DeterminantWasZero)    => 0,
                    Ok(MatrixInversionError::SingularValueWasZero)  => 1,
                    Ok(MatrixInversionError::ZeroDuringInversion)   => 2,
                    Err(_) => 3,
                }
            }
        };
    
        mem::forget(a);
        status
    });
    
    res.unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn index_mut_double_matrix(ptr: *mut c_void, i: c_uint, j: c_uint, value: c_double) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        a[(i as usize, j as usize)] = value;

        mem::forget(a);
    });

    match res
    {
        Ok(_)  => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn index_double_matrix(ptr: *mut c_void, i: c_uint, j: c_uint) -> c_double
{
    let res = catch_unwind(|| {
        let a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
        let value = a[(i as usize, j as usize)];
        mem::forget(a);
        value
    });

    match res
    {
        Ok(o)  => o,
        Err(_) =>
        {
            c_double::MIN
        }
    }
}

#[no_mangle]
pub extern "C" fn clone_double_matrix(ptr: *mut c_void) -> *mut c_void
{
    let res = catch_unwind(|| {
        // Get the actual matrix instance
        let a = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };

        // Use clone to allocate a new instance and mem::forget it AND the old instance
        let b = a.clone();
        mem::forget(a);
    
        Box::into_raw(b) as *mut c_void
    });
    
    match res
    {
        Ok(ptr) => ptr,
        Err(_)  => null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn free_double_matrix(ptr: *mut c_void)
{
    // Try to dealloc. if a panic occurs, abort and leak mem 
    // to avoid UB in the name of Ferris.
    let _ = catch_unwind(|| {
        let _drop_this = unsafe { Box::from_raw(ptr as *mut Matrix<c_double>) };
    });
}
