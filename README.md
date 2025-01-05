# ipynb-to-md

A simple command-line tool that converts Jupyter/IPython Notebook (.ipynb) files into Markdown (.md) files.
This is especially handy when you want to share your notebook content (including code snippets and outputs) in Markdown-based documentation.
I personally use it in Yazi to preview ipynb files.

## Table of Contents
-	Features
-	Requirements
-	Installation
-	Cloning the Repository
-	Building and Running
-	Usage
-	Example
-	License
-	Contributing

### Features
-	Cell Type Detection
    -   Automatically distinguishes between code cells and markdown cells.
-	Code Fencing
    -   Wraps source code in fenced code blocks (e.g., ```python).
-	Output Handling
    -   Extracts text-based outputs and displays them as fenced code blocks.
-	Easy Configuration
    -   Works via command-line parameters—simply specify your input and optionally an output file.
-	Lightweight & Fast
    -   Written in Rust with Serde for JSON parsing.

### Requirements
-	Rust (1.83+ or any current stable version)
    -   Install Rust by following the instructions at rustup.rs.
-	Cargo
    -   Comes bundled with the Rust toolchain, used for building and running Rust projects.

### Installation

#### Cloning the Repository

```bash
git clone https://github.com/<your-username>/ipynb-to-md.git
cd ipynb-to-md
```

#### Building and Running
1.	Build the project:

```bash
cargo build
```

2.	Run the project:

```bash
cargo run -- <input.ipynb> [output.md]
```

If you prefer a release build (optimized for performance), use:

```bash
cargo build --release
```

Then the compiled binary will be located in `target/release/ipynb-to-md`.

### Usage

#### Usage:
`ipynb-to-md <input.ipynb> [output.md]`

-	input.ipynb: The path to a valid Jupyter Notebook file.
-	output.md: Optional path to the resulting Markdown file.
    -	If not specified, the converted Markdown is printed to standard output.

#### Example:

```bash
cargo run -- my_notebook.ipynb converted_notebook.md
```

After running, you should see either an on-screen or file-based representation of your original notebook in Markdown format.

##### Example

Sample .ipynb content might look like this (in abbreviated JSON form):

```json
{
  "cells": [
    {
      "cell_type": "markdown",
      "source": ["# My Notebook\\n", "This is a markdown cell\\n"]
    },
    {
      "cell_type": "code",
      "source": ["print(\"Hello, world!\")\\n"],
      "outputs": [
        {
          "output_type": "stream",
          "text": ["Hello, world!\\n"]
        }
      ]
    }
  ],
  "metadata": {},
  "nbformat": 4,
  "nbformat_minor": 5
}
```

When converted, you’d get:

```markdown
# My Notebook
This is a markdown cell

```python
print("Hello, world!")
``\`

Hello, world!
```

## License

This project is distributed under the [MIT License](LICENSE). You are free to use, modify, and distribute this software as allowed by its license.

## Contributing

Contributions of all kinds are welcome! Feel free to open an issue for any bugs, feature requests, or general questions. If you’d like to make code changes:

1. Fork the repository.
2. Create a new feature branch.
3. Commit your changes.
4. Open a pull request describing what you changed and why.

We appreciate your feedback and contributions!

---

**Happy coding!**  
If you find this tool helpful, consider giving the repository a star ⭐ on GitHub!
