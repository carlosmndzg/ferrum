use std::{error::Error, fs, path::Path};

use dom::{Element, Node, NodeType, Text};

const USER_AGENT_STYLESHEET: &str = include_str!("../assets/default.css");

mod css;
mod dom;
mod html;
mod layout;
mod painter;
mod style;

pub struct Config<'a> {
    pub(crate) file_path: &'a Path,
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. A file path is required.");
        }

        let file_path = Path::new(&args[1]);

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let dom = html::parse(&contents);

    let author_stylesheet = css::parse_author(&dom, config.file_path);
    let user_agent_stylesheet = css::parse_ua(USER_AGENT_STYLESHEET);

    let style_tree = style::build_style_tree(&dom, &author_stylesheet, &user_agent_stylesheet);

    painter::paint(&style_tree, config.file_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec![String::from("program_name"), String::from("file_path")];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.file_path, Path::new("file_path"));
    }

    #[test]
    #[should_panic(expected = "file path is required")]
    fn test_config_build_not_enough_args() {
        let args = vec![String::from("program_name")];

        Config::build(&args).unwrap();
    }
}
