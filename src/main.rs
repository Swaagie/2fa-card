use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use clap::Parser;
use eyre::Result;

/// Provide the two numbers shown by Lynx UI to generate the 2fa value.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    number1: usize,
    number2: usize,

    /// Set a custom YAML config file, defaults to ~/.2fa-card.yaml
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

struct Dictionary {
    values: Vec<String>,
}

impl Dictionary {
    fn from(path: &PathBuf) -> Result<Self> {
        let config = std::fs::read_to_string(path)?;
        let values = serde_yaml::from_str(&config)?;

        Ok(Self { values })
    }

    fn get(&self, value: usize) -> String {
        if value > self.values.len() || value == 0 {
            return String::from("");
        }

        self.values[value - 1].clone()
    }
}

#[derive(Debug)]
struct Factor {
    value: String,
}

impl Factor {
    fn new(number1: usize, number2: usize, dictionary: &Dictionary) -> Self {
        let value = format!("{}{}", dictionary.get(number1), dictionary.get(number2));

        Self { value }
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = match cli.config {
        Some(path) => path,
        None => {
            let mut default_config_path = home::home_dir().expect("Could not find home directory");
            default_config_path.push(".2fa-card.yaml");

            default_config_path
        }
    };

    let dictionary = Dictionary::from(&config_path)?;
    println!("{}", Factor::new(cli.number1, cli.number2, &dictionary));

    Ok(())
}
