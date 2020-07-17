// Each value has one owner
pub fn main() {
    // `x` owns the 3 value
    let x: i32 = 3;

    // The value of `x` is copied because `i32` implements the `Copy` trait (types stored on the stack)
    // `y` owns the copy
    let y: i32 = x;

    // One owner per value so we can access both
    println!("{} {}", x, y);

    // `s1` owns the `String` returned from `String::from("hello")`
    let s1: String = String::from("hello");

    // The value of `s1` is moved (i.e. ownership is transferred to `s2`) because `String`
    // does not implement the `Copy` trait
    let s2: String = s1;

    // This would cause a compile-time error because `s1` no longer owns the value it was assigned
    // println!("{}", s1)

    // The value of `s2` is moved to the parameter of `takes_ownership`
    takes_ownership(s2);

    // This would cause a compile-time error because `s2` no longer owns the value it was assigned
    // println!("{}", s2);

    // `s3` the `String` returned from `String::from("howdy")`
    let s3: String = String::from("howdy");

    // The value of `s3` is moved to the parameter of `takes_and_returns_ownership`
    // and is then returned from `takes_and_returns_ownership` and moved to `s4`
    let s4: String = takes_and_returns_ownership(s3);
    println!("{}", s4);

    // The value of `s4` is moved to the parameter of `takes_and_returns_ownership`
    // and is dropped after it's returned because no one took ownership
    takes_and_returns_ownership(s4);
}

fn takes_ownership(x: String) {
    println!("{}", x)

    // The value of `x` is dropped (i.e. freed from the heap) because
    // it went out of scope and `String` implements the `Drop` trait
}

fn takes_and_returns_ownership(x: String) -> String {
    println!("{}", x);

    // The value of `x` is not dropped because it is returned from the function
    // and someone could take ownership
    return x
}
