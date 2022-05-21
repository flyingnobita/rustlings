// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // Solution 1
    let vec0 = Vec::new();

    // Solution 3
    // let mut vec0 = Vec::new();

    // Solution 1
    let mut vec1 = fill_vec(vec0.clone());

    // Solution 2
    // let mut vec1 = fill_vec(&vec0);

    // Solution 3
    // let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Solution 1
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // Solution 2
    // fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {

    // Solution 3
    // fn fill_vec(vec: &mut Vec<i32>) {

    // Solution 1
    let mut vec = vec;

    // Solution 2
    // let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // Solution 3
    vec
}
