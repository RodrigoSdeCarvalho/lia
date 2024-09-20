use clap::{Parser, Subcommand, Args, arg};
use lia_core::{LiaCore, models::command::NewCommand};

use system::Logger;

#[derive(Parser)]
#[command(name = "CLILIA", version = "0.1", author = "Your Name", about = "Linux Assistant CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the database and configurations.
    Init,
    /// Adds a new command to LIA's storage.
    Add(AddCommand),
    /// Lists all stored commands.
    List,
    /// Executes a stored command by its name.
    Run {
        /// Name of the command to execute.
        name: String,
    },
    // Additional commands like Delete, Update, Search can be added here
}

#[derive(Args)]
struct AddCommand {
    /// A unique name for the command.
    name: String,
    /// The command or script to store.
    command_text: String,
    /// Description of the command.
    #[arg(short, long)]
    description: Option<String>,
    /// Comma-separated tags for categorization.
    #[arg(short, long)]
    tags: Option<String>,
}

#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    let lia_core = if let Ok(core) = LiaCore::new().await {
        core
    } else {
        Logger::error("Failed to create LiaCore instance", true);
        return;
    };

    match cli.command {
        Commands::Init => {
            match LiaCore::init().await {
                Ok(_) => println!("Database initialized successfully."),
                Err(e) => println!("Error initializing database: {}", e),
            }
        }
        Commands::Add(add_cmd) => {
            let tags_vec = add_cmd.tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());
            let new_cmd = NewCommand {
                name: add_cmd.name,
                description: add_cmd.description,
                command_text: add_cmd.command_text,
                tags: tags_vec,
            };
            match lia_core.add_command(new_cmd).await {
                Ok(_) => println!("Command added successfully."),
                Err(e) => println!("Error adding command: {}", e),
            }
        }
        Commands::List => {
            match lia_core.get_all_commands().await {
                Ok(commands) => {
                    for cmd in commands {
                        println!("Name: {}", cmd.name);
                        println!("Description: {}", cmd.description.unwrap_or_default());
                        println!("Command: {}", cmd.command_text);
                        println!("Tags: {:?}", cmd.tags.unwrap_or_default());
                        println!("---");
                    }
                }
                Err(e) => println!("Error retrieving commands: {}", e),
            }
        }
        Commands::Run { name } => {
            match lia_core.get_command_by_name(&name).await {
                Ok(cmd) => {
                    let output = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&cmd.command_text)
                        .output()
                        .expect("Failed to execute command");

                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                Err(e) => println!("Error executing command: {}", e),
            }
        }
    }
}
