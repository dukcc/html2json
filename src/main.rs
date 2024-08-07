use clap::Parser;
use html_parser::{Dom, Node, Result};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The HTML file you want to convert to JSON
    file: PathBuf,
    /// HTML tags that you want to exclude from the JSON
    #[arg(short, long)]
    exclude: Option<Vec<String>>,
}

fn preprocess_html(dom: &mut Dom, exclude_tags: &Option<Vec<String>>) {
    if let Some(tags) = exclude_tags {
        dom.children.retain(|node| !should_exclude(node, tags));
        for node in dom.children.iter_mut() {
            remove_excluded_tags(node, tags);
        }
    }
}

fn should_exclude(node: &Node, exclude_tags: &[String]) -> bool {
    if let Node::Element(element) = node {
        return exclude_tags
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(&element.name));
    }
    false
}

fn remove_excluded_tags(node: &mut Node, exclude_tags: &[String]) {
    if let Node::Element(element) = node {
        element
            .children
            .retain(|child| !should_exclude(child, exclude_tags));
        for child in element.children.iter_mut() {
            remove_excluded_tags(child, exclude_tags);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let output_path = args.file.with_extension("json");

    let html_content = std::fs::read_to_string(&args.file)?;

    let mut dom = Dom::parse(&html_content)?;

    preprocess_html(&mut dom, &args.exclude);

    let json = dom.to_json_pretty()?;
    println!("Wrote output to: {:?}", &output_path);

    std::fs::write(&output_path, json)?;
    Ok(())
}
