// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.



fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // let v = [10, 20, 30, 20];// TODO: declare your vector here with the macro for vectors
    // let v = vec![1, 2, 3, 4];
    let mut v = Vec::new();
    v.push(a[0]);
    v.push(a[1]);
    v.push(a[2]);
    v.push(a[3]);
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
