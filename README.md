# PWClient
 <p align="left">
   <img src="https://raw.githubusercontent.com/Pratech1015/PWClient/refs/heads/main/src-tauri/icons/icon.png" alt="PWClient Icon" width="120">
 </p>

---

### Description

**Physics Wallah Client** is a lightweight desktop client built using the [Tauri](https://tauri.app) framework.
It combines a Rust backend with a clean frontend written in HTML, CSS, and JavaScript, delivering a fast and resource-efficient experience while studying.

---

## Overview

PWClient is designed to be compact, responsive, easy to use and it uses [Physics Wallah Web](https://pw.live) for its main UI.
It takes advantage of caching and fixed PW web bugs, to maintain peaceful learning environment.

---

## Features

- Lightweight and fast desktop application

- Secure, Rust-based backend

- Modern and responsive HTML/CSS/JS interface

- Cross-platform compatibility (Windows, Linux, macOS)

- Fully developed from scratch without third-party UI frameworks

- Extendable with additional modules and utilities

---
## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework |   Tauri    |
|  Backend  |    Rust    |
|  Frontend | HTML, CSS, JS |
| Package   | Cargo, npm |

---

## Installation

PWClient will be distributed as a standalone, bundled executable — no manual dependency installation required.
Once released, simply download the appropriate package for your system from the [**Releases**](https://github.com/Pratech1015/PWClient/releases) section.

---

## Project Structure

```text
PWClient/
├── .vscode/            # VSCode Extensions
├── src-tauri/          # Rust backend (logic, configuration)
├── src/                # Frontend Code (HTML, CSS, JS)
├── .gitignore          # Git ignore files
├── README.md           # Project 
├── package-lock.json   # Client Dependency Lock
└── package.json        # Client Version Configuration
```
---

## Build (for developers)

If you wish to build from source:

```bash
git clone https://github.com/Pratech1015/PWClient.git
cd PWClient
npm install
npm run tauri dev