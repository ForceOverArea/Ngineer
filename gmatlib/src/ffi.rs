use std::ffi::{c_uint, c_void, c_double};
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::mem;
use std::panic::catch_unwind;
use std::ptr::{copy_nonoverlapping, null_mut};
use crate::{Matrix, Element};

/// A more C-friendly format for `Matrix<T>` that must be 
/// manually deallocated.
pub struct CompactMatrix<T>
where T: Element<T>
{
    pub rows: usize,
    pub cols: usize,
    pub vals: *mut T,
}

impl <T> CompactMatrix<T>
where T: Element<T>
{
    /// Converts a `CompactMatrix<T>` to a `Matrix<T>`, allowing
    /// one to operate on a pointer passed from 
    fn to_matrix(&self) -> Matrix<T>
    {
        let nvals = self.rows * self.cols;
        Matrix
        {
            rows: self.rows,
            cols: self.cols,
            vals: unsafe { Vec::from_raw_parts(self.vals, nvals, nvals) },
        }
    }
}

#[no_mangle]
pub extern "C" fn new_double_matrix(rows: c_uint, cols: c_uint) -> *mut c_void
{
    let a = Matrix::new(rows as usize, cols as usize).to_compact_matrix();
    let ptr: *mut CompactMatrix<c_double>;
    let layout = Layout::new::<CompactMatrix<c_double>>();
    unsafe
    {
        ptr = alloc(layout) as *mut CompactMatrix<c_double>;
        if ptr.is_null() { 
            handle_alloc_error(layout); 
        }
        copy_nonoverlapping(&a as *const CompactMatrix<c_double>, ptr, 1);
    }
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn new_double_identity_matrix(n: c_uint) -> *mut c_void
{
    let a = Matrix::new_identity(n as usize).to_compact_matrix();
    let ptr: *mut CompactMatrix<c_double>;
    let layout = Layout::new::<CompactMatrix<c_double>>();
    unsafe
    {
        ptr = alloc(layout) as *mut CompactMatrix<c_double>;
        if ptr.is_null() {
            handle_alloc_error(layout); 
        }
        copy_nonoverlapping(&a as *const CompactMatrix<c_double>, ptr, 1);
    }
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn inplace_row_swap(ptr: *mut c_void, r1: c_uint, r2: c_uint) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { (*(ptr as *mut CompactMatrix<c_double>)).to_matrix() };
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
        let mut a = unsafe { 
            (*(ptr as *mut CompactMatrix<c_double>)).to_matrix() 
        };
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
pub extern "C" fn inplace_row_add(ptr: *mut c_void, r1: c_uint, r2: c_uint) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe { 
            (*(ptr as *mut CompactMatrix<c_double>)).to_matrix() 
        };
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
        let mut a = unsafe { 
            (*(ptr as *mut CompactMatrix<c_double>)).to_matrix()
        };
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
pub extern "C" fn augment_with(ptr_a: *mut c_void, ptr_b: *mut c_void) -> *mut c_void
{
    let (a, b) = unsafe {(
        (*(ptr_a as *mut CompactMatrix<c_double>)).to_matrix(),
        (*(ptr_b as *mut CompactMatrix<c_double>)).to_matrix()
    )};

    let ab = match a.augment_with(&b) {
        Ok(x)  => x.to_compact_matrix(),
        Err(_) => return null_mut(), // return early and indicate failure via NULL
    };

    mem::forget(a); // Prevent drop that would deallocate matrix data. We don't inform the
    mem::forget(b); // caller that a or b will be deallocated, so we shouldn't do it here.

    let ptr: *mut CompactMatrix<c_double>;
    let layout = Layout::new::<CompactMatrix<c_double>>();

    unsafe 
    {
        ptr = alloc(layout) as *mut CompactMatrix<c_double>;
        if ptr.is_null() { 
            handle_alloc_error(layout); 
        }
        copy_nonoverlapping(&ab as *const CompactMatrix<c_double>, ptr, 1);
    }
    
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn try_inplace_invert(ptr: *mut c_void) -> c_uint
{
    let mut a = unsafe {
        (*(ptr as *mut CompactMatrix<c_double>)).to_matrix()
    };

    let result = match a.try_inplace_invert()
    {
        Ok(_)  => 1,
        Err(_) => 0,
    };

    mem::forget(a);
    result
}

#[no_mangle]
pub extern "C" fn index_mut_double_matrix(ptr: *mut c_void, i: c_uint, j: c_uint, value: c_double) -> c_uint
{
    let res = catch_unwind(|| {
        let mut a = unsafe {
            (*(ptr as *mut CompactMatrix<c_double>)).to_matrix()
        };
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
        let a = unsafe {
            (*(ptr as *mut CompactMatrix<c_double>)).to_matrix()
        };
        let value = a[(i as usize, j as usize)];

        mem::forget(a);

        value
    });

    match res
    {
        Ok(o)  => o,
        Err(_) => {
            f64::MIN
        }
    }
}

#[no_mangle]
pub extern "C" fn clone_double_matrix(ptr: *mut c_void) -> *mut c_void
{
    let a = unsafe {(*(ptr as *mut CompactMatrix<c_double>)).to_matrix()};
    let b = a.clone();
    mem::forget(a);

    (&mut b.to_compact_matrix() as *mut CompactMatrix<c_double>) as *mut c_void
}

#[no_mangle]
pub extern "C" fn free_double_matrix(ptr: *mut c_void) -> ()
{
    let layout = Layout::new::<CompactMatrix<c_double>>();
    unsafe 
    {
        dealloc(ptr as *mut u8, layout);
    }
}
