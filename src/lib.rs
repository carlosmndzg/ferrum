use std::{error::Error, fs::File, io::Read};

use dom::{Element, Node, NodeType, Text};

mod css;
mod dom;
mod html;
mod style;

pub struct Config {
    pub(crate) file_path: String,
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
    let dom = html::parse(&contents);

    let css = get_css(&dom);
    let stylesheet = css::parse(css);

    let style_tree = style::build_style_tree(&dom, &stylesheet);

    // println!("Stylesheet: {:#?}", stylesheet);
    println!("Style tree: {:#?}", style_tree);

    Ok(())
}

fn get_css(dom: &dom::Node) -> &str {
    let style_node = dom.find_first_node(&|n| is_style_node(n));

    if let Some(n) = style_node {
        if let Some(Node {
            node_type: NodeType::Text(Text { text }),
            ..
        }) = &n.children.first()
        {
            return text;
        }
    }

    ""
}

fn is_style_node(node: &Node) -> bool {
    matches!(&node.node_type, NodeType::Element(Element { tag_name, .. }) if tag_name == "style")
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
