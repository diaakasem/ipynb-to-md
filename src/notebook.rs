use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Notebook {
    pub cells: Vec<Cell>,
}

#[derive(Debug, Deserialize)]
pub struct Cell {
    #[serde(rename = "cell_type")]
    pub cell_type: String,
    pub source: Vec<String>,
}

use crate::cli::OutputFormat;

/// Process the notebook based on the output format
pub fn process_notebook(content: String, output_format: OutputFormat) -> String {
    let notebook: Notebook =
        serde_json::from_str(&content).expect("Failed to deserialize notebook");

    let mut output = String::new();

    match output_format {
        OutputFormat::Text => {
            for cell in notebook.cells {
                if cell.cell_type == "markdown" {
                    // println!("Markdown Cell: {}", cell.source.join(""));
                    output.push_str(&format!("Markdown Cell: {}\n", cell.source.join("")));
                } else if cell.cell_type == "code" {
                    // println!("Code Cell: {}", cell.source.join(""));
                    output.push_str(&format!("Code Cell: {}\n", cell.source.join("")));
                }
            }
        }
        OutputFormat::Html => {
            println!("<html><body>");
            for cell in notebook.cells {
                if cell.cell_type == "markdown" {
                    // println!("<p>{}</p>", cell.source.join(""));
                    output.push_str(&format!("<p>{}</p>\n", cell.source.join("")));
                } else if cell.cell_type == "code" {
                    // println!("<pre>{}</pre>", cell.source.join(""));
                    output.push_str(&format!("<pre>{}</pre>\n", cell.source.join("")));
                }
            }
            // println!("</body></html>");
            output.push_str("</body></html>");
        }
    }
    output
}
