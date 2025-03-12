# SpacetimeDB Demo

This project demonstrates a demo using SpacetimeDB. The client is written in **React** and the server is written in **Rust**.

## Prerequisites

- **SpacetimeDB Documentation:** For more details on SpacetimeDB, refer to the [SpacetimeDB Docs](https://docs.spacetimedb.com).
- **Rust:**  
  Install Rust if you haven't already (Note: This might not be necessary if you're running everything in a Docker container).  
  You can install Rust from the [official site](https://www.rust-lang.org/tools/install).  
  Ensure that Rust is added to your `PATH` (e.g., `export PATH="$PATH:$HOME/.cargo/bin"`).

## Setup

### Starting the Database

To start the database, run:

```bash
npm run db-up
```

## Using the Spacetime CLI via Docker

### Creating a Module

To create a new module using Rust:

```bash
npm run spacetime -- init --lang rust test-module
```

### Publishing a Module

To create a new module using Rust:

```bash
npm run spacetime -- publish --project-path server quickstart-chat
```

### Calling a Reducer

To call a reducer (e.g., sending a message):

```bash
npm run spacetime -- call quickstart-chat send_message 'Hello, World!'
```

### Viewing Logs

To view logs and verify that the reducer was called:

````bash
npm run spacetime -- logs quickstart-chat```
````

## Other Commands

Generate the backend types into the directory `client/src/module_bindings`:

```bash
npm run spacetime-generate-types
```
