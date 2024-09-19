# LIA Command-Line Tool Help Guide

LIA (Linux Assistant) is a command-line tool designed to help you store, manage, and execute your frequently used Linux commands and scripts efficiently.

Below is a guide on how to use each LIA command, with a focus on the `add` command.

---

## General Usage

```bash
lia <command> [options]
```

- Replace `<command>` with one of LIA's available commands.
- Use `[options]` to pass specific arguments or flags to the command.

---

## Commands Overview

- **`init`**: Initializes the database and configurations.
- **`add`**: Adds a new command to LIA's storage.
- **`list`**: Lists all stored commands.
- **`run`**: Executes a stored command by name.

---

## `init` Command

**Description**: Sets up LIA by initializing the database and necessary configurations.

### Usage

```bash
lia init
```

- Starts the Docker container for the database.
- Runs any required migrations to set up the database schema.

### Example

```bash
lia init
```

---

## `add` Command

**Description**: Adds a new command to LIA's storage for easy access and execution later.

### Usage

```bash
lia add <name> <command_text> [options]
```

- `<name>`: A unique identifier for your command (e.g., "list_home").
- `<command_text>`: The actual Linux command or script you want to store.

### Options

- `--description`, `-d`: A brief description of what the command does.
- `--tags`, `-t`: Comma-separated tags to categorize the command.

### Examples

1. **Add a simple command**:

   ```bash
   lia add "list_home" "ls -la ~"
   ```

   - Adds a command named `list_home` that lists all files in your home directory.

2. **Add a command with a description**:

   ```bash
   lia add "update_system" "sudo apt-get update && sudo apt-get upgrade -y" --description "Update system packages"
   ```

   - Adds a command named `update_system` with a description.

3. **Add a command with tags**:

   ```bash
   lia add "search_logs" "grep 'error' /var/log/syslog" --tags "logs,search"
   ```

   - Adds a command named `search_logs` and categorizes it with the tags `logs` and `search`.

4. **Add a command with a description and tags**:

   ```bash
   lia add "backup_home" "tar -czvf backup.tar.gz ~/" --description "Backup home directory" --tags "backup,home"
   ```

   - Adds a command named `backup_home` with both a description and tags.

### Notes

- **Unique Names**: Each command name must be unique. If you attempt to add a command with a name that already exists, you will receive an error.
- **Quoting Arguments**: If your command or arguments contain spaces, enclose them in quotes.
- **Sudo Commands**: When adding commands that require `sudo`, include `sudo` in the `command_text`. You will be prompted for your password upon execution if necessary.

---

## `list` Command

**Description**: Displays all stored commands along with their details.

### Usage

```bash
lia list
```

### Example

```bash
lia list
```

**Sample Output**:

```
Name: list_home
Description: List all files in your home directory
Command: ls -la ~
Tags: []

Name: update_system
Description: Update system packages
Command: sudo apt-get update && sudo apt-get upgrade -y
Tags: []
```

---

## `run` Command

**Description**: Executes a stored command by its name.

### Usage

```bash
lia run <name>
```

- `<name>`: The unique name of the command you wish to execute.

### Example

```bash
lia run "list_home"
```

- Executes the command associated with `list_home`.

### Notes

- **Sudo Commands**: If the command requires `sudo` privileges, you may be prompted to enter your password.
- **Output**: The output of the executed command will be displayed in your terminal.

---

## Additional Information

### Viewing Help

For help information about LIA and its commands, use:

```bash
lia --help
```

Or for a specific command:

```bash
lia <command> --help
```

### Example

```bash
lia add --help
```

**Sample Output**:

```
Usage: lia add <name> <command_text> [OPTIONS]

Adds a new command to LIA's storage.

Arguments:
  <name>          A unique name for the command
  <command_text>  The command or script to store

Options:
  -d, --description <DESCRIPTION>  Description of the command
  -t, --tags <TAGS>                Comma-separated tags for categorization
  -h, --help                       Print help information
```

---

## Tips for Using LIA

- **Meaningful Names**: Use descriptive names for your commands to make them easy to remember.
- **Organize with Tags**: Categorize your commands with tags for easier searching and management in future updates of LIA.
- **Descriptions**: Providing a description helps you recall the purpose of each command, especially if you have many stored.
- **Regular Updates**: Keep your stored commands up to date, especially if they involve paths or resources that may change.

---

## Example Workflow

1. **Initialize LIA**:

   ```bash
   lia init
   ```

2. **Add a Command**:

   ```bash
   lia add "greet" "echo 'Hello, World!'" --description "Greet the world"
   ```

3. **List Commands**:

   ```bash
   lia list
   ```

4. **Run a Command**:

   ```bash
   lia run "greet"
   ```

   **Output**:

   ```
   Hello, World!
   ```

---

## Troubleshooting

- **Command Not Found**: If you receive an error stating that a command was not found, ensure that you have added it correctly and that you're using the correct name.
- **Permission Denied**: If you encounter permission issues when running commands, check if `sudo` is required and included in your `command_text`.
- **Docker Issues**: Since LIA relies on Docker for the database, ensure Docker is installed and running on your system.

---

## Getting Help

If you need further assistance or encounter issues:

- **Check the Documentation**: Review the help information provided by the `--help` flag.
- **Update LIA**: Ensure you are using the latest version of LIA.
- **Contact Support**: Reach out to the maintainers or community for support.

---

By understanding and utilizing the commands and options available in LIA, especially the `add` command, you can significantly enhance your productivity and streamline your command-line workflows.

Feel free to explore and customize LIA to best suit your needs!
