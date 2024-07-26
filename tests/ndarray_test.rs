// use ndarray;
use ndarray::prelude::*;

#[test]
fn test_vector_sum() {
    println!("Array initialization from vector");
    let array_0 = ndarray::Array1::from_vec(vec![7, 8, 9]);
    println!("{array_0:?}");
    let array_1 = ndarray::Array::<i32, Ix1>::from_vec(vec![1, 1, 1]);

    let res = array_0 + array_1;
    println!("Result {res:?}");
}

#[test]
fn test_dot_product() {
    println!("Array initialization from vector");
    let array_0 = ndarray::Array1::from_vec(vec![7, 8, 9]);
    println!("{array_0:?}");
    let array_1 = ndarray::Array::<i32, Ix1>::from_vec(vec![1, 1, 1]);

    let res = array_0.dot(&array_1);
    println!("Result {res:?}");
}

#[test]
fn test_lanes() {
    println!("Array initialization from vector");
    let array_0 = arr2(&[[7, 8, 9], [0, 1, 2]]);
    println!("{array_0:?}");
    let first_col = array_0.lanes(Axis(0));
    println!("first col:{}", first_col.into_iter().next().unwrap());
    let first_row = array_0.lanes(Axis(1));
    println!("first row:{}", first_row.into_iter().next().unwrap());
);
}

fn main() {
    println!("ndarray in rust");
    println!("Array initialization from vector");
    let array_0 = arr2(&[[7, 8, 9], [0, 1, 2]]);
    println!("{array_0:?}");
}
