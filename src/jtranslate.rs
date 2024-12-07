use clap::Parser;
use error_stack::{Report, Result};
use jlogger_tracing::{jdebug, jerror, jinfo, JloggerBuilder, LevelFilter};
use translib::md::MdEntry;
use translib::md::{md_parse, MdOperation};
use translib::{error::JTranslateError, translate_text};

#[derive(Parser, Debug)]
struct Cli {
    /// Input string.
    #[clap(short = 'T', long)]
    text: Option<String>,

    /// Input File.
    #[clap(short = 'F', long)]
    file: Option<String>,

    /// Input language
    #[clap(short, long, default_value_t=String::from("en"))]
    from: String,

    /// Output language
    #[clap(short, long, default_value_t=String::from("en"))]
    to: String,

    /// Verbose.
    #[clap(short, long, action=clap::ArgAction::Count)]
    verbose: u8,
}

fn process_text(text: &str, from: &str, to: &str) -> Result<(), JTranslateError> {
    let outputs = to.split(',').collect();
    match translate_text(text, from, outputs) {
        Ok(translated) => {
            for entry in translated.iter() {
                jinfo!(Language = entry.language());
                println!();
                println!("{}", entry.text());
                println!();
            }
        }
        Err(e) => jerror!("Error: {:?}", e),
    }
    Ok(())
}

fn process_normal_file(file: &str, from: &str, to: &str) -> Result<(), JTranslateError> {
    let text = std::fs::read_to_string(file)
        .map_err(|e| Report::new(JTranslateError::InvalidData).attach_printable(e))?;

    process_text(&text, from, to)
}

fn process_md_file(file: &str, from: &str, to: &str) -> Result<(), JTranslateError> {
    let text = std::fs::read_to_string(file)
        .map_err(|e| Report::new(JTranslateError::InvalidData).attach_printable(e))?;
    let outputs: Vec<&str> = to.split(',').collect();

    let entries = md_parse(&text)?;

    for to in outputs {
        jdebug!(func = "process_md_file", line = line!());
        for entry in &entries {
            match entry {
                MdEntry::Header(h) => {
                    println!("{}", h.to_md_str(Some((from, to)))?)
                }
                MdEntry::Paragraph(p) => {
                    println!("{}", p.to_md_str(Some((from, to)))?)
                }
                MdEntry::Item(i) => {
                    println!("{}", i.to_md_str(Some((from, to)))?)
                }
                MdEntry::MultipleLineRef(r) => {
                    println!("{}", r.to_md_str(Some((from, to)))?)
                }
                MdEntry::NewLine => {
                    println!();
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), JTranslateError> {
    let cli = Cli::parse();
    let max_level = match cli.verbose {
        0 => LevelFilter::INFO,
        _ => LevelFilter::DEBUG,
    };

    JloggerBuilder::new()
        .max_level(max_level)
        .log_runtime(false)
        .build();

    if let Some(file) = &cli.file {
        if file.ends_with(".md") {
            process_md_file(file, &cli.from, &cli.to)
        } else {
            process_normal_file(file, &cli.from, &cli.to)
        }
    } else if let Some(text) = &cli.text {
        process_text(text, &cli.from, &cli.to)
    } else {
        Ok(())
    }
}
