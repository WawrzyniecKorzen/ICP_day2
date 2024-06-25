#[ic_cdk::query]
fn greet(name: String, number: i8 ) -> String {
    format!("Hello, {} {}!", name, number)
}
