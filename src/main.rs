fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    match vec.get(index) {
        Some(value) => println!("The value at index {} is {}", index, value),
        None => println!("There is no value at index {}", index),
    }
}

// Borrow the elements so the caller can continue using its vector.
fn sum_elements(values: &[i32]) -> i32 {
    values.iter().sum()
}

// Modify the caller's vector by adding the same value at both ends.
fn insert_at_both_ends(values: &mut Vec<i32>, value: i32) {
    values.insert(0, value);
    values.push(value);
}

// Move all elements from second into first, leaving second empty.
fn append_vectors(first: &mut Vec<i32>, second: &mut Vec<i32>) {
    first.append(second);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    match vec.get(2) {
        Some(third_value) => println!("The third value in the vector is: {}", third_value),
        None => println!("The vector has fewer than three elements."),
    }

    // Retrieve the last value
    match vec.last() {
        Some(last_value) => println!("The last value in the vector is: {}", last_value),
        None => println!("There is no last value because the vector is empty."),
    }

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let sum = sum_elements(&vec);
    println!("The sum of all elements is: {}", sum);

    let mut v = vec![1, 2, 3];
    println!("Original vector: {:?}", v);

    insert_at_both_ends(&mut v, 10);
    println!("After inserting at both ends: {:?}", v);

    let mut other = vec![4, 5, 6];
    append_vectors(&mut v, &mut other);
    println!("After appending: {:?}", v);
    println!("Second vector after appending: {:?}", other);
}

#[cfg(test)]
mod tests {
    use super::sum_elements;

    #[test]
    fn sums_elements() {
        assert_eq!(sum_elements(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_elements(&[-5, 2, -1]), -4);
    }

    #[test]
    fn empty_vector_has_zero_sum_and_no_items() {
        let values: Vec<i32> = vec![];
        assert_eq!(sum_elements(&values), 0);
        assert_eq!(values.first(), None);
        assert_eq!(values.last(), None);
        assert_eq!(values.get(2), None);
    }
}
