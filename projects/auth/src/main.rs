use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();

    let user = String::from("testuser");
    let passwd: Option<String> = None;

    let response = client
        .get("https://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();

    println!("{:?}", response);

    Ok(())
}
