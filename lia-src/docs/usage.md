# LiA CLI User Guide

LiA (Linux Assistant) is a command-line tool that helps you store, manage, and execute your frequently used Linux commands and scripts. With LiA, you can organize your commands with names, descriptions, and tags, making it easy to search and execute them whenever needed.

---

## Table of Contents

- [LiA CLI User Guide](#lia-cli-user-guide)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Commands](#commands)
      - [`init`](#init)
      - [`add`](#add)
      - [`update`](#update)
      - [`list`](#list)
      - [`search`](#search)
      - [`run`](#run)
      - [`delete`](#delete)
      - [`log`](#log)
    - [Examples](#examples)
      - [Adding a Command](#adding-a-command)
      - [Listing Commands](#listing-commands)
      - [Searching Commands](#searching-commands)
      - [Running a Command](#running-a-command)
      - [Deleting a Command](#deleting-a-command)
  - [Notes](#notes)

---

## Installation

To install LiA, follow the steps below:

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/RodrigoSdeCarvalho/lia.git
   ```

2. **Navigate to the Directory:**

   ```bash
   cd lia/lia-src
   ```

3. **Install Tauri dependencies:**

   Tauri is a necessary dependency for LiA Desktop.

   Check the [Tauri installation guide](https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-linux) for the necessary dependencies for your system.

   Also, check this page to ensure you won't have any issues with the [Tauri Migration to webkit2gtk-4.1](https://v2.tauri.app/blog/tauri-2-0-0-alpha-3/)

4. **Build the Project:**

   ```bash
   make init
   make migrate
   make build
   ```

   > **Note:**
   >
   > Currently, LiA's database runs on port `5432`. Ensure this port is available on your system. If you have a PostgreSQL server running on this port, you may need to stop it before running LiA. In future versions, this port will be configurable.

5. **Install the Binary:**

   ```bash
   make install
   ```

---

## Usage

LiA provides several commands to interact with your stored commands and scripts.

### Commands

#### `init`

Initializes the database and configurations.

**Usage:**

```bash
lia init
```

**Example:**

```bash
$ lia init
Initializing database...
Database initialized successfully.
```

---

#### `add`

Adds a new command to LiA's storage.

**Usage:**

```bash
lia add <name> <command_text> [OPTIONS]
```

- `<name>`: A unique name for the command.
- `<command_text>`: The command or script to store.

**Options:**

- `-d`, `--description <description>`: (Optional) Description of the command.
- `-t`, `--tags <tags>`: (Optional) Comma-separated tags for categorization.

**Example:**

```bash
$ lia add "list_files" "ls -la" --description "List all files" --tags "list,files"
Command added successfully.
```

---

#### `update`

Updates an existing command.

**Usage:**

```bash
lia update <name> [OPTIONS]
```

- `<name>`: Name of the command to update.

**Options:**

- `-c`, `--command_text <new_command_text>`: (Optional) New command text.
- `-d`, `--description <new_description>`: (Optional) New description.
- `-t`, `--tags <new_tags>`: (Optional) New tags (comma-separated).

**Example:**

```bash
$ lia update "list_files" --command_text "ls -la /home/user" --description "List all files in home directory" --tags "list,files,home"
Command updated successfully.
```

---

#### `list`

Lists all stored commands.

**Usage:**

```bash
lia list
```

**Example:**

```bash
$ lia list
Name: check_updates
Description: Update system packages
Command: sudo apt update && sudo apt upgrade -y
Tags: ["update", "upgrade"]
---
Name: list_files
Description: List all files
Command: ls -la
Tags: ["list", "files"]
---
```

---

#### `search`

Searches for commands matching an optional query and/or tags.

**Usage:**

```bash
lia search [OPTIONS]
```

**Options:**

- `-q`, `--query <query>`: (Optional) The search query.
- `-t`, `--tags <tags>`: (Optional) Comma-separated tags to filter by.

**Notes:**

- You can search by query, tags, both, or neither.
- If no query or tags are provided, all commands are returned.

**Examples:**

1. **Search by Query Only:**

   ```bash
   $ lia search --query "update"
   Name: check_updates
   Description: Update system packages
   Command: sudo apt update && sudo apt upgrade -y
   Tags: ["update", "upgrade"]
   ---
   ```

2. **Search by Tags Only:**

   ```bash
   $ lia search --tags "files"
   Name: list_files
   Description: List all files
   Command: ls -la
   Tags: ["list", "files"]
   ---
   ```

3. **Search by Query and Tags:**

   ```bash
   $ lia search --query "list" --tags "home"
   Name: list_files
   Description: List all files in home directory
   Command: ls -la /home/user
   Tags: ["list", "files", "home"]
   ---
   ```

4. **Search with Neither Query nor Tags:**

   ```bash
   $ lia search
   Name: check_updates
   Description: Update system packages
   Command: sudo apt update && sudo apt upgrade -y
   Tags: ["update", "upgrade"]
   ---
   Name: list_files
   Description: List all files
   Command: ls -la
   Tags: ["list", "files"]
   ---
   ```

---

#### `run`

Executes a stored command by its name.

**Usage:**

```bash
lia run <name> [OPTIONS]
```

- `<name>`: Name of the command to execute.

**Options:**

- `-p`, `--path <path>`: (Optional) The path where the command should be executed. Defaults to the current directory.

**Example:**

```bash
$ lia run "list_files" --path "/var/log"
total 64
drwxr-xr-x  8 root root  4096 Oct  1 12:34 .
drwxr-xr-x 18 root root  4096 Oct  1 10:20 ..
-rw-r--r--  1 root root   220 Apr  4  2018 logfile.log
...
```

**Notes:**

- If the command requires `sudo`, run LiA with `sudo`:

  ```bash
  sudo lia run "check_updates"
  ```

---

#### `delete`

Deletes commands by name or tags, with confirmation.

**Usage:**

```bash
lia delete [OPTIONS]
```

**Options:**

- `-n`, `--name <name>`: (Optional) Name of the command to delete.
- `-t`, `--tags <tags>`: (Optional) Comma-separated tags to filter commands for deletion.
- `--all`: (Optional) Delete all commands.

**Notes:**

- At least one of `--name` or `--tags` or `all` must be provided.
- LiA will display the commands that match the criteria and ask for confirmation before deletion. Unless `--all` is used, in which case all commands will be deleted.

**Examples:**

1. **Delete by Name:**

   ```bash
   $ lia delete --name "list_files"
   The following commands will be deleted:
   Name: list_files
   Description: List all files
   Command: ls -la
   Tags: ["list", "files"]
   ---
   Are you sure you want to delete these commands? [y/N] y
   Commands deleted successfully.
   ```

2. **Delete by Tags:**

   ```bash
   $ lia delete --tags "update"
   The following commands will be deleted:
   Name: check_updates
   Description: Update system packages
   Command: sudo apt update && sudo apt upgrade -y
   Tags: ["update", "upgrade"]
   ---
   Are you sure you want to delete these commands? [y/N] n
   Deletion cancelled.
   ```

3. **Delete All Commands:**

   ```bash
   $ lia delete --all
   Are you sure you want to delete all commands? [y/N] y
   All commands deleted successfully.
   ```

---

#### `log`

Toggles logging on/off. **Note:** Must be run with `sudo`.

**Usage:**

To enable logging:

```bash
sudo lia log --on
```

To disable logging:

```bash
sudo lia log --off
```

**Options:**

- `--on`: Enable logging.
- `--off`: Disable logging.

**Example:**

```bash
$ sudo lia log --on
Logging turned on
```

---

### Examples

#### Adding a Command

**Command:**

```bash
$ lia add "check_updates" "sudo apt update && sudo apt upgrade -y" --description "Update system packages" --tags "update,upgrade"
Command added successfully.
```

---

#### Listing Commands

**Command:**

```bash
$ lia list
Name: check_updates
Description: Update system packages
Command: sudo apt update && sudo apt upgrade -y
Tags: ["update", "upgrade"]
---
Name: list_files
Description: List all files
Command: ls -la
Tags: ["list", "files"]
---
```

---

#### Searching Commands

1. **Search by Query:**

   ```bash
   $ lia search --query "update"
   Name: check_updates
   Description: Update system packages
   Command: sudo apt update && sudo apt upgrade -y
   Tags: ["update", "upgrade"]
   ---
   ```

2. **Search by Tags:**

   ```bash
   $ lia search --tags "files"
   Name: list_files
   Description: List all files
   Command: ls -la
   Tags: ["list", "files"]
   ---
   ```

3. **Search by Query and Tags:**

   ```bash
   $ lia search --query "list" --tags "home"
   Name: list_files
   Description: List all files in home directory
   Command: ls -la /home/user
   Tags: ["list", "files", "home"]
   ---
   ```

---

#### Running a Command

**Command:**

```bash
$ lia run "list_files" --path "/var/log"
total 64
drwxr-xr-x  8 root root  4096 Oct  1 12:34 .
drwxr-xr-x 18 root root  4096 Oct  1 10:20 ..
-rw-r--r--  1 root root   220 Apr  4  2018 logfile.log
...
```

---

#### Deleting a Command

1. **Delete by Name:**

   ```bash
   $ lia delete --name "list_files"
   The following commands will be deleted:
   Name: list_files
   Description: List all files
   Command: ls -la
   Tags: ["list", "files"]
   ---
   Are you sure you want to delete these commands? [y/N] y
   Commands deleted successfully.
   ```

2. **Delete by Tags:**

   ```bash
   $ lia delete --tags "update"
   The following commands will be deleted:
   Name: check_updates
   Description: Update system packages
   Command: sudo apt update && sudo apt upgrade -y
   Tags: ["update", "upgrade"]
   ---
   Are you sure you want to delete these commands? [y/N] n
   Deletion cancelled.
   ```

---

## Notes

- **Initialization:**
  - Ensure you have initialized the database before using other commands by running `lia init`.

- **Running Commands:**
  - When using `lia run`, the output of the stored command will be displayed in real-time.
  - Use `sudo` when necessary, especially for commands that require elevated permissions.

- **Tags:**
  - Tags are useful for categorizing and searching through your stored commands.
  - You can assign multiple tags to a command by separating them with commas.

- **Searching:**
  - The `search` command is flexible—you can search by query, tags, both, or neither.
  - Searching without any parameters will list all commands.

- **Deleting Commands:**
  - The `delete` command requires confirmation before deleting any commands.
  - Be cautious when deleting commands, especially when using tags that may match multiple commands.

- **Logging:**
  - Logging can be toggled on or off using the `log` command.
  - Remember to run `lia log` with `sudo` privileges.

---

**Experience the convenience of having all your essential Linux commands at your fingertips with LiA!**

---

**Feedback and Contributions:**

- If you encounter any issues or have suggestions for improvements, feel free to open an issue or contribute to the project on GitHub: [https://github.com/RodrigoSdeCarvalho/lia](https://github.com/RodrigoSdeCarvalho/lia)

---

**Enjoy using LiA to enhance your Linux command-line productivity!**
