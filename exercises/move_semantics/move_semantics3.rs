// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);  // `vec0` is moved here

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Someone on Discord said that the new owner of `vec0`, which is `vec` in this case,
// can re-delare the mutability of it. Which is why we don't need to declare `vec0` like this:
// let mut vec0 = Vec::new();

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
