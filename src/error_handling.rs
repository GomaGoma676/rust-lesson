pub fn run() {
    let res1 = division_option(5.0, 0.0);
    match res1 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Not allowed !!"),
    }
    let res2 = division_result(5.0, 1.0);
    match res2 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("{}", e),
    }
    let a = [0, 1];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("Total is: {}", x),
        None => println!("Out of index !!"),
    }
}
fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}
fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed !!"))
    } else {
        Ok(x / y)
    }
}
fn sum(a: &[i32]) -> Option<i32> {
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
