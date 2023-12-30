use gmatlib::{Matrix, row_vec};

#[test]
fn ensure_try_inplace_invert_3_works_as_expected()
{
    let mut a = Matrix::from_vec(
        3, 
        vec![ 1.0,  2.0, -1.0,
              2.0,  1.0,  2.0,
             -1.0,  2.0,  1.0]
    ).unwrap();

    a.try_inplace_invert().unwrap();

    let a_vec: Vec<f64> = <Matrix<f64> as Into<Vec<f64>>>::into(a);

    let check = vec![
        3.0/16.0 as f64, 1.0/4.0 as f64, -5.0/16.0 as f64,
        1.0/4.0  as f64,     0.0 as f64,  1.0/4.0  as f64,
       -5.0/16.0 as f64, 1.0/4.0 as f64,  3.0/16.0 as f64
    ];

    assert_eq!(a_vec, check);
}

#[test]
fn ensure_try_inplace_invert_4_works_as_expected()
{
    let mut a = Matrix::from_vec(
        4, 
        vec![4.0,  0.0,  0.0,  0.0,
             0.0,  0.0,  2.0,  0.0,
             0.0,  1.0,  2.0,  0.0,
             1.0,  0.0,  0.0,  1.0]
    ).unwrap();

    a.try_inplace_invert().unwrap();

    let a_vec: Vec<f64> = <Matrix<f64> as Into<Vec<f64>>>::into(a);

    let check = vec![ 0.25, 0.0, 0.0, 0.0,
                      0.0, -1.0, 1.0, 0.0,
                      0.0,  0.5, 0.0, 0.0,
                     -0.25, 0.0, 0.0, 1.0];

    assert_eq!(a_vec, check);
}

#[test]
fn ensure_try_inplace_invert_n_works_as_expected()
{
    let mut a = Matrix::from_vec(
        5,
        vec![ 3.0, 11.0,  2.0, 17.0, 22.0, 
              4.0, 10.0, 12.0, 18.0, 23.0, 
              8.0,  9.0, 14.0, 19.0, 24.0, 
              5.0,  7.0, 15.0, 20.0, 25.0, 
              6.0, 13.0, 16.0, 21.0, 26.0]
    ).unwrap();

    a.try_inplace_invert().expect("Failed to invert matrix");

    let shift = 10.0f64.powi(4);

    let a_vec: Vec<f64> = <Matrix<f64> as Into<Vec<f64>>>::into(a)
        .into_iter()
        .map(|i: f64| {
            (i * shift).floor() / shift
        })
        .collect();

    let check = vec![
        0.0181, -0.1819,  0.2909, -0.1091, -0.0182,
       -0.0091,  0.0909, -0.0205, -0.1955,  0.1340,
       -0.1182,  0.1818, -0.0160, -0.0410, -0.0069,
        0.4236, -2.2364, -0.5469,  0.9081,  1.2513,
       -0.2691,  1.6909,  0.3945, -0.5855, -1.0310
    ];

    assert_eq!(a_vec, check);
}

#[test]
fn ensure_that_readme_example_works() 
{
    //use gmatlib::{Matrix, row_vec};

    // Create a matrix with 3 columns
    let a: Matrix<i32> = Matrix::from_vec(
        3, 
        vec![1, 2, 3,
             4, 5, 6,
             7, 8, 9]
    ).unwrap();

    // Matrices support appropriate binary operations
    let b = row_vec![0_i32, 1_i32, 0_i32] * (a * 3);

    // ...and concise indexing 
    assert_eq!(
        b[(0, 1)],
        15
    );
}

#[test]
fn ensure_that_matrix_struct_implements_clone_and_partialeq_works_as_expected()
{
    let a: Matrix<i32> = Matrix::new_identity(3);
    let b: Matrix<i32> = a.clone();

    assert_eq!(a, b);
}

#[test]
fn ensure_inplace_transpose_method_works_on_case_not_in_doctest()
{
    let a: Matrix<i32> = Matrix::from_vec(
        3, 
        vec![1, 2, 3, 
             4, 5, 6]
    ).unwrap();
    assert_eq!(a.get_rows(), 2);
    assert_eq!(a.get_cols(), 3);
    
    // Swap rows and cols
    let b = a.transpose();
    assert_eq!(b.get_rows(), 3);
    assert_eq!(b.get_cols(), 2);
    
    let b_vec: Vec<i32> = b.into();
    
    assert_eq!(
        b_vec,
        vec![1, 4,
             2, 5,
             3, 6]
    );
}