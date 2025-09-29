use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut res = reqwest::blocking::get("https://httpbin.org/get")?;
    // Send a synchronous (blocking) GET request to the URL using the reqwest crate.
    let mut body = String::new(); // Create a mutable, empty String that will hold the response body.
    res.read_to_string(&mut body)?;
    // Read the response body into the `body` string.

    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: \n{}", body);
    Ok(())
}
