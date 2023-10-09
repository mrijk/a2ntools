use serde::{Deserialize, Serialize};

use clap::{Parser};

use std::fs::File;
use std::io::{self, BufReader};
use std::str;

use std::path::PathBuf;

mod readers;
use readers::version_7::read_version_7_action_file;
use readers::version_16::read_version_16_action_file;
use readers::helpers::{read_u32};

fn read_version(reader: &mut BufReader<File>) -> u32 {
    read_u32(reader)
}

#[derive(Serialize, Deserialize)]
struct VersionOnly{
    version: u32
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long, default_value_t=false)]
    version_only: bool,
    #[clap(short, long, default_value="json")]
    format: String
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file_name = args.path.unwrap_or(PathBuf::from("test.atn"));

    let file = File::open(file_name)?;

    let mut reader = BufReader::new(file);

    let version = read_version(&mut reader);
    if args.version_only {
        let version_only = VersionOnly{version};
        println!("{}", serde_json::to_string(&version_only).unwrap());
        return Ok({});
    }

    let result = match version {
        7 => read_version_7_action_file(&mut reader),
        16 => read_version_16_action_file(&mut reader),
        _ => panic!("Unknown version {}", version)
    };

    match args.format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&result.to_json()).unwrap()),
        "yaml" => println!("{}", serde_yaml::to_string(&result.to_yaml()).unwrap()),
        _ => panic!("Unkown format {}", args.format)
    }
    
    Ok(())
}