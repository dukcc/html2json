# html2json

A simple Rust CLI tool for converting HTML markdown to JSON format.

(one of my first rust projects)

Originally built for processing [Bible HTML data](https://ebible.org) into JSON for the [Bibleio API](https://github.com/bibleio)

## Usage

I plan to publish this as an installable CLI tool, but for now you're gonna have to clone this repo, and run it with Cargo:
```bash
cargo run Example.html
```
Exclude certain tags:
```bash
cargo run Example.html --exclude head --exclude style --exclude script
```

## Plans

- Allow exclusion of certain classes and ids (example: `--exclude div.class1 --exclude ul#nav`)
- Allow exclusion of strings/phrases (example: `--excludeStr "&#160;")
- Publish installer/package
