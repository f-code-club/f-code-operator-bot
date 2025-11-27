# f-code-operator-bot

A Discord bot for managing f-code candidates.

## Features

*   Ping the bot to check if it's online.
*   Get help with the bot's commands.
*   Add, delete, and verify candidates.

## Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/f-code-operator-bot.git
    ```
2.  Install the Rust toolchain: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
3.  Create a `.env` file and add the following environment variables:
    ```
    DISCORD_TOKEN=your-discord-token
    ```
4.  Run the database migrations:
    ```bash
    sqlx migrate run
    ```
5.  Run the bot:
    ```bash
    cargo run --release
    ```

## Usage

The following commands are available:

### `/ping`

Test if bot is responsive.

**Example:**
```
/ping
```

### `/help`

Shows a help message, either for all commands or for a specific command.

**Parameters:**
*   `command` (optional): The command to get help for.

**Example:**
```
/help
```
```
/help add
```

### `/add`

Add one or more candidate IDs to the candidates database from a text file.

**Permissions:** `Moderator`

**Parameters:**
*   `id`: A text file with candidate IDs (one per line, UTF-8).

**Example:**
```
/add [attach file with IDs]
```

### `/delete`

Delete a candidate by ID from the candidates database.

**Permissions:** `Moderator`

**Parameters:**
*   `id`: The ID of the candidate to delete.

**Example:**
```
/delete SE1000
```

### `/verify`

Verify a candidate and assign a role upon success. The bot will check if the user's nickname or global name contains the provided ID.

**Parameters:**
*   `id`: The candidate's ID to verify.

**Example:**
```
/verify SE1000
```

## Database

The bot uses an SQLite database to store information about candidates. The database is created automatically when the bot is first run. The database schema is defined in the `migrations` directory.
