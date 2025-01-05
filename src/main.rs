use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

/// Represents the top-level Notebook structure of an ipynb file.
#[derive(Debug, Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

/// Represents a single cell in the Notebook. A cell can be "markdown" or "code".
#[derive(Debug, Deserialize)]
struct Cell {
    #[serde(rename = "cell_type")]
    cell_type: String,

    /// The lines of source code or markdown (depending on `cell_type`).
    source: Vec<String>,

    /// Outputs only exist for code cells; for markdown cells, this might be empty.
    outputs: Option<Vec<Output>>,
}

/// Represents a single output object in a code cell (like stdout, display_data, etc.).
#[derive(Debug, Deserialize)]
struct Output {
    #[serde(rename = "output_type")]
    output_type: String,

    /// For "stream" outputs (like stdout), text is often populated.
    text: Option<Vec<String>>,

    /// For "execute_result"/"display_data", data might contain "text/plain" or other MIME types.
    data: Option<HashMap<String, Vec<String>>>,

    /// Not used in this script, but included for completeness.
    name: Option<String>,
}

/// Convert a code cell to Markdown output, wrapping code in fenced blocks,
/// and including any text-based outputs as fenced blocks as well.
fn convert_code_cell_to_markdown(cell: &Cell) -> String {
    let mut sb = Vec::new();

    // 1) Code block (assumed Python by default)
    sb.push("```python".to_string());
    for line in &cell.source {
        // Trim trailing newline just as in the V code.
        sb.push(line.trim_end_matches('\n').to_string());
    }
    sb.push("```".to_string());
    sb.push("".to_string());

    // 2) Process outputs
    if let Some(outputs) = &cell.outputs {
        for output in outputs {
            match output.output_type.as_str() {
                "stream" => {
                    // If there's any text, wrap it in a fenced block as well
                    if let Some(text_lines) = &output.text {
                        if !text_lines.is_empty() {
                            sb.push("```".to_string());
                            sb.push(text_lines.join(""));
                            sb.push("```".to_string());
                        }
                    }
                }
                "execute_result" | "display_data" => {
                    // Check for text/plain
                    if let Some(data_map) = &output.data {
                        if let Some(plain_text) = data_map.get("text/plain") {
                            sb.push("```".to_string());
                            sb.push(plain_text.join(""));
                            sb.push("```".to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    sb.join("\n")
}

/// Convert a markdown cell to Markdown output by joining all lines as-is.
fn convert_markdown_cell_to_markdown(cell: &Cell) -> String {
    cell.source.join("")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} input.ipynb [output.md]", args[0]);
        return;
    }

    let input_file = &args[1];

    // Check if the file exists
    if !Path::new(input_file).exists() {
        eprintln!("Error: file '{}' not found", input_file);
        return;
    }

    // Read the file contents
    let ipynb_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Parse the JSON into our Notebook struct
    let notebook: Notebook = match serde_json::from_str(&ipynb_content) {
        Ok(nb) => nb,
        Err(e) => {
            eprintln!("Error parsing ipynb JSON: {}", e);
            return;
        }
    };

    // Convert cells to Markdown
    let mut out_lines = Vec::new();
    for cell in notebook.cells {
        match cell.cell_type.as_str() {
            "markdown" => {
                out_lines.push(convert_markdown_cell_to_markdown(&cell));
                out_lines.push(String::new()); // blank line
            }
            "code" => {
                out_lines.push(convert_code_cell_to_markdown(&cell));
                out_lines.push(String::new()); // blank line
            }
            _ => {}
        }
    }

    // If an output file is specified, write to it; otherwise, print to stdout.
    if args.len() > 2 {
        let output_file = &args[2];
        if let Err(e) = fs::write(output_file, out_lines.join("\n")) {
            eprintln!("Error writing to output file: {}", e);
            return;
        }
        println!("Conversion complete. Output written to '{}'", output_file);
    } else {
        println!("{}", out_lines.join("\n"));
    }
}
