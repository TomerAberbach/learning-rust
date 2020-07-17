// Values can be borrowed to avoid having to
// take and return ownership
fn main() {
    // `s1` owns the `String` returned from `String::from("hello")`
    let s1: String = String::from("hello");

    // The value of `s1` is borrowed (`&s1` is a reference to `s1`)
    println!("{}", borrow(&s1));

    // `s1` still has ownership of its value here

    // `s2` owns the `String` returned from `String::from("world")`
    let mut s2: String = String::from("world");

    // The value of `s2` is borrowed (`&s2` is a reference to `s2`)
    // with mutation allowed
    actually_change(&mut s2);

    let r1: &mut String = &mut s2;
    let r2: &mut String = &mut s2;

    // This line would cause a compile-time error because you cannot have
    // more than one mutable reference (that is used) at the same time (no data races!)
    // println!("{}, {}", r1, r2);

    // You can have multiple immutable references (no data races possible)
    let r3: &String = &s2;
    let r4: &String = &s2;
    println!("{}, {}", r3, r4);

    // This line would cause a compile-time error because you can't have both
    // immutable and mutable references at the same time (no data races, and
    // no immutable values changing)
    // let r5: &mut String = &mut s2;
    // println!("{}, {}, {}", r3, r4, r5);
}

fn borrow(s: &String) -> usize {
    s.len()

    // `s` is reference so nothing is dropped here
}

fn try_change(s: &String) {
    // This line would cause a compile-time error because `s` is not a mutable reference
    // s.push_str("wow");
}

fn actually_change(s: &mut String) {
    // Works because `s` is a mutable reference
    s.push_str("wow");
}

// fn no_reference_to_dropped_value() -> &String {
//     let s: String = String::from("hello");
//
//     // `s` is dropped here! Can't return a reference
//     // to a dropped value (no dangling pointers)
//     return &s
// }
