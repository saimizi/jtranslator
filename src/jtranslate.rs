use clap::Parser;
use jlogger_tracing::{jerror, jinfo, JloggerBuilder, LevelFilter};
use translib::translate_text;

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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let max_level = match cli.verbose {
        0 => LevelFilter::INFO,
        _ => LevelFilter::DEBUG,
    };

    JloggerBuilder::new()
        .max_level(max_level)
        .log_runtime(false)
        .build();

    let text = cli.text.unwrap_or_else(|| {
        if let Some(file) = &cli.file {
            if let Ok(content) = std::fs::read_to_string(file) {
                content
            } else {
                jerror!("Failed to read {}", file);
                String::new()
            }
        } else {
            String::new()
        }
    });

    if !text.is_empty() {
        let outputs = cli.to.split(',').collect();
        match translate_text(&text, &cli.from, outputs).await {
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
    }
}
