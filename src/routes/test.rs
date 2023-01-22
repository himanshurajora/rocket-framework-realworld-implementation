#[get("/add/<a>/<b>")]
pub fn add_two_numbers(a: i32, b: i32) -> String {
    let result = a + b;
    return result.to_string();
}
