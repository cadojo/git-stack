use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Push the current branch and its dependencies
    Push {
        /// Force push all branches
        #[arg(short, long)]
        force: bool,
    },
    /// Rebase the current branch and all dependent branches
    Rebase {
        /// Base branch to rebase onto
        #[arg(short, long, default_value = "main")]
        base: String,
    },
    /// List all stacked branches
    List,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Push { force } => {
            println!("Pushing branches (force: {})", force);
            Ok(())
        }
        Commands::Rebase { base } => {
            println!("Rebasing onto {}", base);
            Ok(())
        }
        Commands::List => {
            println!("Listing stacked branches");
            Ok(())
        }
    }
}
