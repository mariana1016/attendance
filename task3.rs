fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2;
    });

    let replaced = process_vector(numbers, |x| {
        if x > 2 {
            0
        }
        else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}
