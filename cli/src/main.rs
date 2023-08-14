#![allow(unused)]
// use std::env::args;
// use std::io;
// use reqwest::Error;

// Functional
use std::env;
use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use serde_json::{json, Result, Value, Error};
use serde::{Deserialize, Serialize};
use regex::Regex;

// Pretty print
use colored::*;
use colored_json::prelude::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use std::io::{self, Write};

// Terminal
use indicatif::ProgressBar;
use spinners::{Spinner, Spinners};



#[derive(Parser)]
#[clap(author="Tony Stark", version, about="Jarvis available in your terminal")]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long)]
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
    
}

#[derive(Subcommand)]
enum Commands {
    /// AI create
    Code(AddArgs),
    /// AI review code
    Review(AddArgs)
}

#[derive(Args)]
struct AddArgs {
    #[arg(short, long)]
    request: String,
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}


fn extract_between_backticks(input: &str) -> Option<String> {
    let re = Regex::new(r"(?s)```(.*?)```").unwrap();
    if let Some(captures) = re.captures(input) {
        Some(captures.get(1).map_or("", |m| m.as_str()).to_string())
    } else {
        None
    }
}

/// Terminal
fn parse_and_print(json_data: &str) -> std::result::Result<String, serde_json::Error> {
    
    // Deserialize JSON data into a serde_json::Value
    let data: Value = serde_json::from_str(json_data)?;

    // Convert the JSON data back to a string
    let formatted_json = serde_json::to_string_pretty(&data).unwrap();
    
    // Replace the escaped characters using regex
    let re = Regex::new(r"\\n").unwrap();
    let formatted_json_re: String = (re.replace_all(&formatted_json, "\n")).to_string();
    
    // Print the formatted output to the terminal
    // println!("{}", formatted_json);

    if let Some(extracted) = extract_between_backticks(&formatted_json_re) {
        println!("Extracted: {}", extracted);
        Ok((extracted))
    } else {
        println!("No match found.");
        Ok((formatted_json_re))
    }
}


/// General functions
fn greet() {
    println!("Good day sir.");
}

fn get_time() {
    let current_time = chrono::Local::now().format("%H:%M:%S");
    println!("The current time is: {}", current_time);
}

fn make_sandwich() {
    println!("Here you are sir: \n 🥪")
}


/// Read and write files
fn read_file(file_path: &PathBuf) {
    let f = File::open(file_path).expect("Couldn't open file :(");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line);
    reader
        .read_to_string(&mut line)
        .expect("cannot read string");
    println!("input: {}", line);
    println!("First line is {:?} bytes long", len);
}

fn write_file(file_contents: &[u8], file_path: &PathBuf) -> std::result::Result<String, String> {
    let mut file = File::create(file_path).unwrap();
    match file.write_all(file_contents) {
        Ok(file) => return Ok("Success".to_string()),
        Err(e) => return Err(e.to_string())
    }
}

/// AI 
fn generate_code(args: &AddArgs) -> std::result::Result<String, String> {

    let code = match call_api_endpoint(&args.request) {
        Ok(code) => return Ok(code.to_string()),
        Err(e) => return Err(e.to_string())
    };

   
}


#[tokio::main]
async fn call_api_endpoint(args: &String) -> std::result::Result<String, reqwest::Error> {

    // Create a new reqwest client
    let client = reqwest::Client::new();
    
    let host = env::var("HOST").is_ok();
    let port = env::var("PORT").is_ok();
    
    // Replace the URL with the actual API endpoint you want to call
    let url = ("{}:{}/generate_text", host, port);

    let json_payload = json!({
        "prompt": args,
    });

    match client.post(url).json(&json_payload).send().await {
        // Check if the response was successful (status code 2xx)
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await;
                return body
            } else {
                return Ok("hit the else".to_string())
            }
        }
        Err(e) => return Ok(e.to_string())
    };
    

}


fn main() -> std::io::Result<()> {
    println!("Certainly.");

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Code(request) => {
            if request.request.is_empty()  {
                println!("Is there something I can help you with sir?");
            } else {

                
                let code = match generate_code(&request) {
                    Ok(code) => {
                        // Format the code in the response
                        let pretty_code = parse_and_print(&code);
                        // println!("***&***{:#?}", pretty_code);
                        match write_file(pretty_code?.as_bytes(), &request.file) {
                        Ok(result) => {
                            println!("Write file: {:?}", result);
                        }
                        Err(err) => {
                            println!("Error while writing file: {}", err);
                        }    
                        }
                    }
                    Err(err) => {
                        println!("Error: {}", err)
                    }
                };
            }
        },
        Commands::Review(request) => {
            println!("Request: {}", request.request);
            println!("file: {:?}", request.file);
        }
    
    }

    Ok(())
}
