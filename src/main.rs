use clap::Parser;
use html_parser::{Dom, Result};
use regex::Regex;
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

fn preprocess_html(html: &str) -> String {
    let mut cleaned_html = html.to_string();

    let doctype_pattern = r#"<!DOCTYPE[^>]*>"#;
    let re_doctype = Regex::new(doctype_pattern).unwrap();
    cleaned_html = re_doctype.replace_all(&cleaned_html, "").into_owned();

    cleaned_html
}

fn main() -> Result<()> {
    let args = Args::parse();

    let output_path = args.file.with_extension("json");

    let html_content = std::fs::read_to_string(&args.file)?;

    let cleaned_html = preprocess_html(&html_content);

    let dom = Dom::parse(&cleaned_html)?;
    let json = dom.to_json_pretty()?;
    println!("Wrote output to: {:?}", &output_path);

    std::fs::write(&output_path, json)?;
    Ok(())
}
