use std::sync::Arc;
use clap::{Parser, Subcommand};
use colored::*;
use std::io::Write;
use tracing::info;

use crate::agent::Agent;
use crate::context::Context;
use crate::core::config::Config;
use crate::llm::ollama::OllamaProvider;
use crate::Result;

#[derive(Parser)]
#[command(name = "oberon")]
#[command(about = "Local-first AI Agent Runtime")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Run in dry-run mode (no actual changes)
    #[arg(short, long, global = true)]
    pub dry_run: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a task
    Run {
        /// Natural language task description
        task: Vec<String>,
    },
    /// Show available tools
    Tools,
    /// Show configuration
    Config,
    /// Replay a previous session
    Replay {
        session_id: String,
    },
}

impl Cli {
    pub async fn run() -> Result<()> {
        let cli = Self::parse();

        let config = Arc::new(Config::load()?);
        let llm = Arc::new(OllamaProvider::from_config(&config.llm));
        let context = Arc::new(Context::new((*config).clone(), llm));

        if cli.dry_run {
            context.sandbox.set_dry_run(true);
            info!("Dry-run mode enabled");
        }

        match cli.command {
            Some(Commands::Run { task }) => {
                let task_str = task.join(" ");
                info!("Running task: {}", task_str);

                let mut agent = Agent::new(context);

                let pb = indicatif::ProgressBar::new_spinner();
                pb.set_message("Thinking...");
                pb.enable_steady_tick(std::time::Duration::from_millis(100));

                let result = agent.run(&task_str).await?;

                pb.finish_and_clear();
                println!("{}", result.green());
            }
            Some(Commands::Tools) => {
                let tools = context.tools.list_tools();
                println!("Available tools:");
                for tool in tools {
                    println!("  - {}", tool);
                }
            }
            Some(Commands::Config) => {
                println!("{}", toml::to_string_pretty(&*config).unwrap());
            }
            Some(Commands::Replay { session_id }) => {
                println!("Replaying session: {}", session_id);
                // TODO: implement replay from log
            }
            None => {
                interactive_loop(context).await?;
            }
        }

        Ok(())
    }
}

async fn interactive_loop(context: Arc<Context>) -> Result<()> {
    let mut agent = Agent::new(context);

    println!("{}", "Oberon Agent Runtime".bold().blue());
    println!("Type 'exit' to quit\n");

    loop {
        print!("{} ", ">".bold().green());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if !input.is_empty() {
            match agent.run(input).await {
                Ok(response) => println!("{}\n", response),
                Err(e) => eprintln!("{} {}\n", "Error:".red().bold(), e),
            }
        }
    }

    Ok(())
}