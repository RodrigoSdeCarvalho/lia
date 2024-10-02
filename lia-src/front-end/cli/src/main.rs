use std::{
    sync::mpsc::channel,
    thread,
};
use clap::{Parser, Subcommand, Args, arg};

use lia_core::{
    LiaCore, 
    models::command::{NewCommand, UpdateCommand}
};
use system::{Logger, set_process_name, SysConfigs};

#[derive(Parser)]
#[command(
    name = "CLILiA", 
    version = "0.1", 
    author = "Your Name", 
    about = "Linux Assistant CLI",
    long_about = "LIA (Linux Assistant) helps you store, manage, and execute your frequently used Linux commands and scripts."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the database and configurations.
    Init,
    /// Adds a new command to LiA's storage | Example: lia add "ls" "ls -la" --description "List all files" --tags "list,files"
    Add(CLIAddCommand),
    /// Updates an existing command.
    Update(CLIUpdateCommand),
    /// Lists all stored commands.
    List,
    /// Searches for commands matching the query.
    Search {
        /// The search query.
        #[arg(short, long)]
        query: Option<String>,

        /// Tags to filter by (comma-separated).
        #[arg(short, long)]
        tags: Option<String>,
    },    
    /// Executes a stored command by its name.
    Run {
        /// Name of the command to execute.
        name: String,
    },
    /// Toggle logging on/off. Must be run with sudo.
    Log {
        /// Enable logging
        #[arg(long)]
        on: bool,
        /// Disable logging
        #[arg(long)]
        off: bool,
    },
}

#[derive(Args)]
struct CLIAddCommand {
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

#[derive(Args)]
struct CLIUpdateCommand {
    /// Name of the command to update.
    name: String,
    /// New command text.
    #[arg(short, long)]
    command_text: Option<String>,
    /// New description.
    #[arg(short, long)]
    description: Option<String>,
    /// New tags.
    #[arg(short, long)]
    tags: Option<String>,
}

#[tokio::main]
async fn main() {
    set_process_name("CLI LiA");

    let cli = Cli::parse();

    let lia_core = if let Ok(core) = LiaCore::new().await {
        core
    } else {
        Logger::error("Failed to create LiaCore instance", true);
        return;
    };

    match cli.command {
        Commands::Init => {
            println!("Initializing database...");
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
        Commands::Update(update_cmd) => {
            let tags_vec = update_cmd.tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());
            let new_cmd = UpdateCommand {
                name: update_cmd.name,
                new_command_text: update_cmd.command_text,
                new_description: update_cmd.description,
                new_tags: tags_vec,
            };
            match lia_core.update_command(new_cmd).await {
                Ok(_) => println!("Command updated successfully."),
                Err(e) => println!("Error updating command: {}", e),
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
        Commands::Search { query, tags } => {
            let tags_vec = tags.map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>()
            });

            let query = query.unwrap_or_default();

            let commands = match lia_core.search_commands(&query, tags_vec).await {
                Ok(c) => c,
                Err(_) => {
                    println!("Error searching for commands.");
                    return;
                }
            };
            for cmd in commands {
                println!("Name: {}", cmd.name);
                println!("Description: {}", cmd.description.unwrap_or_default());
                println!("Command: {}", cmd.command_text);
                println!("Tags: {:?}", cmd.tags.unwrap_or_default());
                println!("---");
            }
        }
        Commands::Run { name } => {
            let path = match std::env::current_dir() {
                Ok(p) => p,
                Err(_) => {
                    println!("Error getting current directory.");
                    return;
                }
            };

            let (tx, rx) = channel();
            let handle = thread::spawn(move || {
                while let Ok(line) = rx.recv() {
                    println!("{}", line);
                }
            });

            let cmd = match lia_core.get_command_by_name(&name).await {
                Ok(cmd) => cmd,
                Err(_) => {
                    println!("Command not found.");
                    return;
                }
            };

            match lia_core.run_command_stream(cmd, &path, tx).await {
                Ok(_) => handle.join().expect("Failed to join thread"),
                Err(_) => println!("Error running command."),
            };
        }
        Commands::Log { on, off } => {
            let is_root = LiaCore::is_sudo_user();
            if !is_root {
                println!("This command must be run with sudo.");
                return;
            }

            let toggle = on || !off;
            SysConfigs::set_log(toggle, false, None);
            println!("Logging turned {}", if toggle { "on" } else { "off" });
        }
    }
}
