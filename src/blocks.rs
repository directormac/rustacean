pub fn blocks() {
    let awsome_number = { 73 };

    // Simple block that the last statement without semicolon is the evaluation
    let x = {
        let y = 37;
        let z = 36;
        y + z
    };

    let the_truth = true;
    // The cool thing about it, it can access things from parent block
    let awesome_truth: f32 = {
        if the_truth {
            73.0
        } else {
            -73.0
        }
    };

    println!("The awesome number is {}", awsome_number);
    print!(" this is what x evaluated to {x}");
    print!(" the true awesome number is  {:?}", awesome_truth);
}
