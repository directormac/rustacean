// Everything lives inside a function!!!! except these 2, they are special
// const and static are special they are immutable and memory was allocated even before the main function
// They are global!
const AWESOME_NUMBER: i32 = 73;
static NOT_PI: f32 = 3.1337;
const WASTE_OF_MEMORY: &str = "Hello, Variables";

// Yes this is how we name things now. No camelCase or PascalCase
pub fn variable_assignments() {
    let x = 73; // Type is infered
    let typed_x: i32 = x - 73 + 2 * 73 / 2; // declared the type using the :
    let hello = "Hello, ";
    let world = String::from("World!");

    // if you are gonna unuse something that throws out data into the universe use _
    let _ = -73;
    let _x = "useless value";

    //This line adds the const and static they and the (var as type) is a way of converting
    //primitives
    let awesome_pi = (AWESOME_NUMBER as f32) + NOT_PI;

    println!("Hello World");
    print!("{WASTE_OF_MEMORY}");
    // Statements can go on multiple lines they are ended with ;
    println!(
        // inline comments wont affect the coode
        "This is the variable.rs\n 
        x should go here {} \n
        tpyed_x is the same as x {} \n
        _x wont warn about being unused because of _ \n
        an str  and String {hello} {world} \n
        and now the awesome_pi {} \n
        ", // But you cannot do an inline comment inside double quotes
        x, typed_x, awesome_pi
    ); // The statement ends here
}
