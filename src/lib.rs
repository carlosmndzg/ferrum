use std::{error::Error, fs::File, io::Read};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. A file path is required.");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = get_file_contents(&config.file_path)?;

    println!("Contents: {}", contents);

    Ok(())
}

fn get_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec![String::from("program_name"), String::from("file_path")];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.file_path, "file_path");
    }

    #[test]
    #[should_panic(expected = "file path is required")]
    fn test_config_build_not_enough_args() {
        let args = vec![String::from("program_name")];

        Config::build(&args).unwrap();
    }
}
