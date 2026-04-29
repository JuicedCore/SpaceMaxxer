<img width="952" height="578" alt="Screenshot 2026-04-29 161059" src="https://github.com/user-attachments/assets/08a3c741-ba52-425f-9d7e-ccb5be303e7d" /># Spacemaxxer

A 3D file system visualizer built with **Rust** and **Unity**.

Instead of looking at standard, boring file trees, **Spacemaxxer** scans your drive and generates a navigable 3D cityscape out of your data. Big files equal massive buildings. It uses Rust to do the heavy lifting fast, and Unity to render the results.

---

## How It Actually Works

The architecture is split into a 3-step pipeline to keep the Unity main thread from freezing up when scanning massive directories.

1. **The Scanner (`cli_tool.exe`):** A custom Rust binary that recursively crawls whatever folder you select. It bypasses the usual OS bloat and just dumps the raw hierarchy into a JSON file.
2. **The Plotter (`plotter.exe`):** Another Rust tool that reads the raw map and runs a squarified treemap algorithm. It calculates the exact 3D coordinates ($x, y, z$), dimensions ($w, h, d$), and assigns a deterministic RGB color based on the file hash. 
3. **The Renderer (Unity/C#):** The C# frontend uses an async bridge to trigger the Rust executables in the background. Once the final JSON is ready, Unity steps in, reads the layout, and instantiates the 3D blocks. 

---

## Stuff It Does

* **Async Processing:** The UI and loading screens stay smooth while Rust scans the disk in the background. 
* **Smart Culling:** Automatically ignores 0-byte files and null values so the engine doesn't waste memory rendering microscopic, cubes.
* **Hash-Based Colors:** Every file gets a specific color based on its properties, making it easy to visually identify what is eating up your storage.

---

## Running the App

**Command to generate the json output at `./output.json`**
```bash
cargo run -- . output.json --json
```

**Command to get floating point json with folders:**
```bash
cargo run -p plotter -- --unity --show-folders
```

If you are cloning this to build it in Unity yourself, you must drop the compiled Rust binaries (cli_tool.exe and plotter.exe) into the Assets/StreamingAssets/ folder before you hit build.
The Unity project is seperate.

---
Preview of a smaple folder being visualzied

<img width="952" height="578" alt="image" src="https://github.com/user-attachments/assets/bce5cf1d-ecfe-4963-a39d-8defdf8b540e" />
