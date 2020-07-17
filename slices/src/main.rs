// Slices are references/borrows of a contiguous sequence in a
// collection, rather than the whole collection, that follow
// the same rules as normal references
fn main() {
    // `s1` owns the `String` returned from `String::from("hello world")`
    let s1: String = String::from("hello world");

    // String slices type is `&str`
    // `hello` is a reference to just the first 5 characters
    // of `s1` (inclusive start index and exclusive end index)
    let hello: &str = &s1[0..5];
    let world: &str = &s1[6..11];

    // Can omit starting index if it is 0
    let hello2: &str = &s1[..5];

    // Can omit end index if it is the length of the string
    let world2: &str = &s1[6..];

    // This line would cause a compile time error because
    // there are immutable references to `s1` (via slices)
    // and the `clear` function tries to mutably borrow `s1`
    // s1.clear();

    // Other slice types are `&[type]`
    let array: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &array[..2];
}
