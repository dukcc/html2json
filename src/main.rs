use clap::Parser;
use html_parser::{Dom, Result};
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The HTML file you want to convert to JSON
    file: PathBuf,
    /// The output file (optional, will default to the same name, same path)
    output: PathBuf,
    /// HTML tags that you want to exclude from the JSON
    #[arg(short, long)]
    exclude: Vec<String>,
}

fn preprocess_html(html: &str, tags_to_exclude: Vec<String>) -> String {
    let mut cleaned_html = html.to_string();

    let doctype_pattern = r#"<!DOCTYPE[^>]*>"#;
    let re_doctype = Regex::new(doctype_pattern).unwrap();
    cleaned_html = re_doctype.replace_all(&cleaned_html, "").into_owned();

    for tag in tags_to_exclude {
        let pattern = format!(r"<{}[^>]*>.*?</{}>", tag, tag);
        let re = Regex::new(&pattern).unwrap();

        cleaned_html = re.replace_all(&cleaned_html, "").into_owned();
    }

    cleaned_html
}

fn main() -> Result<()> {
    let args = Args::parse();

    let html_content = std::fs::read_to_string(&args.file)?;

    let cleaned_html = preprocess_html(&html_content, args.exclude);

    let dom = Dom::parse(&cleaned_html)?;
    let json = dom.to_json_pretty()?;
    println!("Wrote output to: {:?}", &args.output);

    std::fs::write(&args.output, json)?;
    Ok(())
}
