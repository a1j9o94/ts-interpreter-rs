use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional file to execute
    file: Option<String>,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();
    
    if let Some(file) = cli.file {
        println!("Executing file: {}", file);
        // TODO: Implement file execution
    } else {
        println!("Starting REPL...");
        // TODO: Implement REPL
    }
}