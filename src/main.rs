use std::collections::HashMap;
use std::env;
use std::fs;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

fn main() {
    let args: Vec<String> = env::args().collect();

    let authorized_key_file = &args[1];

    let contents = fs::read_to_string(authorized_key_file)
        .expect("Should have been able to read the file");

    let client= reqwest::blocking::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build().expect("How the FUCK (Could not create HTTP client)");

    let request = client.get("https://api.github.com/meta").send();
    
    let result = request.expect("The request failed to send lmao").json::<HashMap<String,serde_json::Value>>().expect("I did not get json?")["actions"].clone();

    if let serde_json::Value::Array(result) = result {
        let length = result.len();
        print!("from=\"");
        for (i, cidr) in result.iter().enumerate() {
            if let serde_json::Value::String(cidr) = cidr {
                if i == length-1 {
                    print!("{cidr}\",");
                }
                else {
                    print!("{cidr},");
                }
            };
        }
        println!("{contents}")
    }
}
