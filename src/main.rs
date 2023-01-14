// get data from questioning user 
    // find a way to hide secret contact key per session or user
    // sanitize gotten data, never trust a random PoorFuckingInfantry

// contact space wizrad with a user query
// get data from said wizard
    // decode it and ? format it ?
// display data on bash with colors if possible 
#[macro_use]

mod args;
use args::HelpMeArgs;
use clap::Parser;
use dotenv::dotenv;
use tinyjson::JsonValue;

fn main() {
    dotenv().ok();
    let args: HelpMeArgs = HelpMeArgs::parse(); 
    let wizard_url = "https://api.openai.com/v1/completions";
    let api_key = std::env::var("APIKEY").expect("Key must be sat before proceding");
    
    let response = contact_space_wizard(&args.Qustion, &wizard_url, &api_key).unwrap() ;
    let parsed_response: JsonValue = response.parse().unwrap();

    println!("{:?}",parsed_response["choices"][0]["text"].stringify().unwrap() );

}

fn contact_space_wizard(question: &str, wizard_url: &str, api_key: &str) -> Result<(String),  ureq::Error>{
    let bearer_header = format!("Bearer {}", api_key);
    match ureq::post(wizard_url)
        .set("Authorization", &bearer_header )
        .send_json(ureq::json!({
            "model": "text-davinci-003",
            "prompt": question,
            "temperature": 1,
            "max_tokens": 100,
        }))?
        .into_string() {
            Ok(res) =>{
                
                return Ok(res) 
                
                },
            Err(_) => panic!("something went wrong contacting the space wizard")
        };
}