use std::collections::HashMap;

fn sum(vector: &Vec<f64>) -> f64 {
    let mut sum = 0.0;

    for number in vector {
        sum += number
    }

    sum
}

fn mean(vector: &Vec<f64>) -> f64 {
    sum(vector) / vector.len() as f64
}

fn median(vector: &Vec<f64>) -> f64 {
    let mut clone = vector.clone();
    clone.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = vector.len() / 2;

    if vector.len() % 2 == 0 {
        (vector[mid] + vector[mid - 1]) / 2.0
    } else {
        vector[mid]
    }
}

fn mode(vector: &Vec<i32>) -> Option<i32> {
    let mut counts: HashMap<i32, u32> = HashMap::new();

    for number in vector {
        counts
            .entry(*number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counts
        .iter()
        .max_by_key(|(_, count)| **count)
        .map(|(number, _)| *number)
}

fn main() {
    let vector: Vec<f64> = vec![15.0, 32.2, 5.6, 5.6, -4.3];

    let vector2: Vec<i32> = vector.iter().map(|number| number.round() as i32).collect();

    println!("Sum: {}", sum(&vector));
    println!("Mean: {}", mean(&vector));
    println!("Median: {}", median(&vector));
    println!("Mode: {}", mode(&vector2).unwrap())
}
