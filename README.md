# rovert Chat

This program allows you chat with your local [ollama](https://ollama.com/) models.

## Stack

- **Tauri** (Rust) for the desktop app
- **Vue.js** for the web app
- **Tailwind** CSS

## Running the program

Make sure you have ollama installed on your system. You can download it [here](https://ollama.com/) and follow the installation instructions.

Then make sure you added the tauri origin to ollama cross-origin policy in environment variables.

```bash
export OLLAMA_ORIGINS=http://tauri.localhost*
```

After launching ollama server, you can run the program.

## Development

Clone the repository and install the dependencies.

```bash
git clone https://github.com/R-udren/rovert-planner

cd rovert-planner

deno install
```

Then you can run the program in development mode.

```bash
deno task tauri dev
```

## Build

You can build the program for production.

```bash
deno task tauri build
```
