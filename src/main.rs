mod funcs;

fn main() {
    println!("Hello, world!"); // This is a macrom, its not as easy at it looks

    // Cheap knock off of println macro
    let _ = funcs::withargs::printme("Hello, Cheap World!\n"); // Go to the definition
    let a = 70;
    let b: i32 = 3; // Not infered
    let sum = funcs::withargs::add(a, b);
    println!("{a}+ {b} = {sum}");

    let doubled = funcs::withargs::double(sum);
    println!("sum was {} now its doubled {}", sum, doubled) // alternate

    // Now comes the elephant in the room Strings and all its types in rust

    // Flow control section
}
