pub fn matchy_matchy() {
    let x = bool_match(true);
    let truthy = CheckTruthy::Truthy;
    let falsy = CheckTruthy::Falsy;
    println!("{}", inblock_match(truthy));
    println!("{}", inblock_match(falsy));
    println!("{x}");
}

fn bool_match(truth: bool) -> i32 {
    match truth {
        true => -73,
        false => 73,
    }
}

#[derive(Debug)]
enum CheckTruthy {
    Truthy,
    Falsy,
}

fn inblock_match(check: CheckTruthy) -> &'static str {
    match check {
        CheckTruthy::Truthy => "This is truthy",
        CheckTruthy::Falsy => "This is falsy",
    }
}
