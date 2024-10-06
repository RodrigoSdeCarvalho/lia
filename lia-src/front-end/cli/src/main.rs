use std::io::{stdout, Write};
use tokio::task;

use clap::{Parser, Subcommand, Args, arg};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use lia_core::{
    LiaCore,
    errors::LiaCoreError,
    models::command::{NewCommand, UpdateCommand, Command}
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
    List {
        /// The maximum number of commands to display.
        #[arg(short, long, default_value = "100")]
        limit: i64,
    },
    /// Searches for commands matching the query.
    Search {
        /// The search query.
        #[arg(short, long)]
        query: Option<String>,

        /// Tags to filter by (comma-separated).
        #[arg(short, long)]
        tags: Option<String>,

        /// The maximum number of results to display.
        #[arg(short, long, default_value = "10")]
        limit: i64,
    },    
    /// Executes a stored command by its name.
    Run {
        /// Name of the command to execute.
        name: String,
    },
    /// Deletes commands by name or tags.
    Delete {
        /// Name of the command to delete.
        #[arg(short, long)]
        name: Option<String>,

        /// Tags to filter commands for deletion (comma-separated).
        #[arg(short, long)]
        tags: Option<String>,

        /// Flag to indicate deletion of all commands.
        #[arg(long)]
        all: bool,
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
        Commands::List { limit } => {
            match lia_core.get_all_commands(limit, 0).await {
                Ok(commands) => {
                    display_commands_paginated(commands);
                }
                Err(e) => println!("Error retrieving commands: {}", e),
            }
        }
        Commands::Search { query, tags, limit } => {
            let tags_vec = tags.map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>()
            });

            let query = query.unwrap_or_default();

            let commands = match lia_core.search_commands(&query, tags_vec, limit, 0).await {
                Ok(c) => c,
                Err(_) => {
                    println!("Error searching for commands.");
                    return;
                }
            };
            display_commands_paginated(commands);
        }
        Commands::Run { name } => {
            let path = match std::env::current_dir() {
                Ok(p) => p,
                Err(_) => {
                    println!("Error getting current directory.");
                    return;
                }
            };

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            // let handle = thread::spawn(move || {
            //     while let Ok(line) = rx.recv() {
            //         println!("{}", line);
            //     }
            // });

            let handle = task::spawn(async move {
              while let Some(line) = rx.recv().await {
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
                Ok(_) => handle.await.expect("Failed to join thread"),
                Err(_) => println!("Error running command."),
            };
        }
        Commands::Delete { name, tags , all} => {
            if name.is_none() && tags.is_none() && !all {
                eprintln!("Error: You must provide either a name or tags to delete commands.");
                return;
            } else if all {
                println!("Are you sure you want to delete all commands? [y/N]");
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input).map_err(LiaCoreError::IoError) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error reading input: {}", e);
                        return;
                    }
                }
                let input = input.trim().to_lowercase();

                if input == "y" || input == "yes" {
                    match lia_core.delete_all_commands().await {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Error deleting commands.");
                            return;
                        }
                    }
                    println!("All commands deleted successfully.");
                } else {
                    println!("Deletion cancelled.");
                }
                return;
            }

            let tags_vec = tags.map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>()
            });

            let commands_to_delete = match lia_core.find_commands_for_deletion(name.clone(), tags_vec.clone()).await {
                Ok(c) => c,
                Err(_) => {
                    println!("Error finding commands for deletion.");
                    return;
                }
            }; 

            if commands_to_delete.is_empty() {
                println!("No commands found matching the criteria for deletion.");
                return;
            }

            println!("The following commands will be deleted:");
            for cmd in &commands_to_delete {
                println!("Name: {}", cmd.name);
                println!("Description: {}", cmd.description.clone().unwrap_or_default());
                println!("Command: {}", cmd.command_text);
                println!("Tags: {:?}", cmd.tags.clone().unwrap_or_default());
                println!("---");
            }

            println!("Are you sure you want to delete these commands? [y/N]");
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input).map_err(LiaCoreError::IoError) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error reading input: {}", e);
                    return;
                }
            }
            let input = input.trim().to_lowercase();

            if input == "y" || input == "yes" {
                match lia_core.delete_commands(name, tags_vec).await {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error deleting commands.");
                        return;
                    }
                }
                println!("Commands deleted successfully.");
            } else {
                println!("Deletion cancelled.");
            }
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

fn display_commands_paginated(commands: Vec<Command>) {
    const PAGE_SIZE: usize = 10;
    let mut current_page: usize = 0;
    let mut display = true;
    let total_pages = if commands.is_empty() {
        1
    } else {
        (commands.len() + PAGE_SIZE - 1) / PAGE_SIZE
    };

    if let Err(e) = enable_raw_mode() {
        eprintln!("Error enabling raw mode: {}", e);
        return;
    }

    let mut stdout = stdout();
    if let Err(e) = execute!(stdout, Hide) {
        eprintln!("Error hiding cursor: {}", e);
    }

    loop {
        if display{
            let start = current_page * PAGE_SIZE;
            let end = std::cmp::min(start + PAGE_SIZE, commands.len());
            let current_commands = &commands[start..end];

            for (i, cmd) in current_commands.iter().enumerate() {
                let command_number = start + i + 1;

                if let Err(e) = write!(stdout, "Command {}\r\n", command_number) {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
                if let Err(e) = write!(stdout, "Name: {}\r\n", cmd.name) {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
                if let Err(e) = write!(
                    stdout,
                    "Description: {}\r\n",
                    cmd.description.clone().unwrap_or_default()
                ) {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
                if let Err(e) = write!(stdout, "Command: {}\r\n", cmd.command_text) {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
                let tags: Vec<String> = cmd
                    .tags
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|tag| tag.clone())
                    .collect();
                if let Err(e) = write!(stdout, "Tags: {:?}\r\n", tags) {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
                if let Err(e) = write!(stdout, "---\r\n") {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
            }

            if let Err(e) = write!(stdout, "Page {}/{}\r\n", current_page + 1, total_pages) {
                eprintln!("Error writing to stdout: {}", e);
                break;
            }
            if let Err(e) = write!(
                stdout,
                "Use Up/Down arrows to navigate, 'q' to quit.\r\n"
            ) {
                eprintln!("Error writing to stdout: {}", e);
                break;
            }
            if let Err(e) = write!(stdout, "---\r\n") {
                eprintln!("Error writing to stdout: {}", e);
                break;
            }

            if let Err(e) = stdout.flush() {
                eprintln!("Error flushing stdout: {}", e);
                break;
            }
        }

        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Enter => {
                    if current_page + 1 < total_pages {
                        display = true;
                        current_page += 1;
                    } else {
                        display = false;
                    }
                }
                _ => {
                    display = false;
                }
            },
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading event: {}", e);
                break;
            }
        }
    }

    if let Err(e) = execute!(stdout, Show) {
        eprintln!("Error showing cursor: {}", e);
    }
    if let Err(e) = disable_raw_mode() {
        eprintln!("Error disabling raw mode: {}", e);
    }
    if let Err(e) = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)) {
        eprintln!("Error clearing screen: {}", e);
    }
}
