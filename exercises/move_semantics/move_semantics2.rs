// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = Vec::new();

    // 1st solution: make a clone of `vec0` s.t. we can still use it afterward
    // let vec0_clone = vec0.clone();
    // let mut vec1 = fill_vec(vec0_clone);

    // 2nd solution function call
    // let mut vec1 = fill_vec(&vec0);

    // 3rd solution function call
    fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // 3rd solution no need to do anything with `vec1`
    // vec1.push(88);
    //
    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}


// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();
//
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//
//     vec
// }

// 2sn solution: instead of taking ownership of vec0, we'll borrow it
// and copy the data within the function body
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();
//
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//
//     vec
// }

// 3rd solution: not return anything, hence we can get rid of `vec1`
fn fill_vec(vec: &Vec<i32>) {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
