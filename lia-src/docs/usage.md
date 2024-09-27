# LiA CLI User Guide

LiA (Linux Assistant) is a command-line tool that helps you store, manage, and execute your frequently used Linux commands and scripts.

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
      - [`log`](#log)
    - [Full Examples](#full-examples)
      - [Adding a Command](#adding-a-command)
      - [Listing Commands](#listing-commands)
      - [Searching Commands](#searching-commands)
      - [Running a Command](#running-a-command)
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

3. **Build the Project:**

   ```bash
   make init
   make migrate
   make build
   ```

   > **Note:**
   >
   > Currently, LiA's database only runs on port `5432:5432`. Make sure this port is available on your system. If you have a PostgreSQL server running on this port, you may need to stop it before running LiA. In the future, this port will be configurable.

4. **Install the Binary:**

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
- `-t`, `--tags <new_tags>`: (Optional) New tags.

**Example:**

```bash
$ lia update "list_files" --command_text "ls -la /home/user" --description "List all files in home directory"
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
Name: list_files
Description: List all files
Command: ls -la
Tags: ["list", "files"]
---
Name: disk_usage
Description: Check disk usage
Command: df -h
Tags: ["disk", "usage"]
---
```

---

#### `search`

Searches for commands matching the query.

**Usage:**

```bash
lia search <query>
```

- `<query>`: The search query.

**Example:**

```bash
$ lia search "disk"
Name: disk_usage
Description: Check disk usage
Command: df -h
Tags: ["disk", "usage"]
---
```

---

#### `run`

Executes a stored command by its name.

**Usage:**

```bash
lia run <name>
```

- `<name>`: Name of the command to execute.

**Example:**

```bash
$ lia run "list_files"
total 64
drwxr-xr-x  8 user user  4096 Oct  1 12:34 .
drwxr-xr-x 18 user user  4096 Oct  1 10:20 ..
-rw-r--r--  1 user user   220 Apr  4  2018 .bash_logout
-rw-r--r--  1 user user  3771 Apr  4  2018 .bashrc
...
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

### Full Examples

#### Adding a Command

**Command:**

```bash
$ lia add "check_updates" "sudo apt update && sudo apt upgrade -y" --description "Update system packages" --tags "update,upgrade"
Command added successfully.
```

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

#### Searching Commands

**Command:**

```bash
$ lia search "update"
Name: check_updates
Description: Update system packages
Command: sudo apt update && sudo apt upgrade -y
Tags: ["update", "upgrade"]
---
```

#### Running a Command

**Command:**

```bash
$ sudo lia run "check_updates"
[sudo] password for user:
Hit:1 http://archive.ubuntu.com/ubuntu focal InRelease
Get:2 http://archive.ubuntu.com/ubuntu focal-updates InRelease [114 kB]
...
```

---

## Notes

- Ensure you have initialized the database before using other commands by running `lia init`.
- When using `lia run`, the output of the stored command will be displayed in real-time.
- Use `sudo` when necessary, especially for commands that require elevated permissions.
- Tags are useful for categorizing and searching through your stored commands.

---

**Experience the convenience of having all your essential Linux commands at your fingertips with LiA!**
