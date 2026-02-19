/// ContextRecall: A context-aware shell history manager

// Declare the modules
mod db;
mod finder;

use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Records a new command into the history
    Record {
        /// The command string that was executed
        cmd: String,
        
        /// The exit code of the command
        #[arg(short, long, default_value_t = 0)]
        exit_code: i32,
    },
    /// Searches the history for the current context
    Search,
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let cli = Cli::parse();

    // The '?' operator will now automatically convert ANY error 
    // (Database or IO) into our generic Boxed error.
    let conn = db::get_db_connection()?;

    match &cli.command {
        Commands::Record { cmd, exit_code } => {
            let current_dir = env::current_dir()?; // This ? works for IO errors now too!
            
            let context_path = match finder::find_project_root(&current_dir) {
                Some(p) => p.to_string_lossy().into_owned(),
                None => "global".to_string(),
            };

            db::record_command(&conn, &context_path, cmd, *exit_code)?;
            
            println!("Recorded: '{}' in context '{}'", cmd, context_path);
        }
        Commands::Search => {
            let current_dir = env::current_dir()?;
            
            // 1. Determine Context
            let context_path = match finder::find_project_root(&current_dir) {
                Some(p) => p.to_string_lossy().into_owned(),
                None => "global".to_string(),
            };

            // 2. Get History
            let history = db::get_context_history(&conn, &context_path)?;

            // 3. Print each command on a new line
            // This is what 'fzf' will read!
            for cmd in history {
                println!("{}", cmd);
            }
        }
    }

    Ok(())
}
