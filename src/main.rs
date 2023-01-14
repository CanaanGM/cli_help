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
fn main() {
    let args: HelpMeArgs = HelpMeArgs::parse(); 
    println!("{}", args.Qustion)
}
