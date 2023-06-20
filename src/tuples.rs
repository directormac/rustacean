pub fn tup_types() {
    println!("Tuplesssssssss ");
    let pair = ('x', 73); // infered
    let typed_pair: (char, i32) = ('x', 73);

    // Useful in destructuring
    let (a_char, an_int) = ('x', 73);

    let (destrutured_char, destrutured_int) = destruct_me();
    print!("{:?} ", pair); // this is how you print types that dont implement fmt::Display
    println!("{:?}", typed_pair);

    //as per normal you can access a destructured tuple
    println!("a char is {a_char} an int is {an_int}");

    println!("{} {}", destrutured_char, destrutured_int);
}
// This is how you declare a function with a return type  fn name() -> type {}
fn destruct_me() -> (char, i32) {
    ('x', 73)
}
