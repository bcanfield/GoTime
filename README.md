<img src="client/public/go.svg" width="100" height="100" alt="GoTime Logo"/>

# GoTime
  
  [![tests](https://img.shields.io/github/actions/workflow/status/bcanfield/GoTime/test.yml?label=tests&logo=saturn)](https://github.com/bcanfield/GoTime/actions/workflows/test.yml)


**GoTime** is a multiplayer [Go](https://en.wikipedia.org/wiki/Go_(game)) game built with [SpacetimeDB](https://spacetimedb.com/).

<p align="left">
  <a href="https://codespaces.new/bcanfield/GoTime"><img src="https://img.shields.io/badge/Open%20in-Github%20Codespaces-blue?style=flat-square&logo=github" alt="Open in Github Codespaces"></a>
  <a href="https://codesandbox.io/p/github/bcanfield/GoTime/main"><img src="https://img.shields.io/badge/Open%20in-CodeSandbox-blue?style=flat-square&logo=codesandbox" alt="Open in CodeSandbox"></a>
</p>

## ✨ Features

- **Fast Multiplayer**: Built with SpacetimeDB for rapid state updates and low-latency interactions
- **Real-Time Game Analysis**: Dynamic board analysis, scoring (both area & territory methods), and move legality checks
- **Modern Tech Stack**: React frontend and Rust backend

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/bcanfield/GoTime.git
cd GoTime

# Install dependencies
npm install

# Start SpacetimeDB
npm run db:start

# Publish the module to SpacetimeDB
npm run db:publish

# Start the UI
npm run client:dev

##### Optional

# To see the SpacetimeDB logs:
npm run db:logs

# To re-generate the Typescript bindings
npm run generate
```

---

## 📖 About the Game

GoTime is a proof-of-concept implementation of the ancient board game Go. It demonstrates how SpacetimeDB can be used to create responsive multiplayer experiences with complex game logic.

The repository contains:
- A client app written in [React](https://react.dev/)  
- A server app written in [Rust](https://www.rust-lang.org/)

For the rules of Go, see our [GO_RULES.md](/GO_RULES.md) document or visit one of the many excellent online resources for learning the game.

## 🛠️ Development Guide

### Dev Container Support

This project uses a dev container for a consistent development environment with Rust, Node.js, and the SpacetimeDB CLI pre-installed. Open the project in GitHub Codespaces or CodeSandbox using the badges above.

### Manual Setup

If you prefer to develop locally, make sure you have:
- Node.js (v16+)
- Rust (latest stable)
- SpacetimeDB CLI


## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is open source and available under the [MIT License](LICENSE).



