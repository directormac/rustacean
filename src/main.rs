mod blocks;
mod funcs;
mod pattern_matching;
mod tuples;
mod variables;

fn main() {
    variables::variable_assignments(); //variables.rs
    tuples::tup_types();
    blocks::blocks();
    pattern_matching::matchy_matchy();

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
