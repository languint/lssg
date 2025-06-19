use std::{fs, path::PathBuf, process};

use owo_colors::OwoColorize;

use crate::cli::Config;

pub fn create_dist(current_dir: &PathBuf) {
    // Create the distribution directory if it doesn't exist
    if let Err(err) = fs::create_dir_all(current_dir.join("dist")) {
        println!("Error creating distribution directory: {}", err);
        process::exit(1);
    }
}

pub fn create_file(current_dir: &PathBuf, output_file: &str, translated: &str) {
    let output_dir = current_dir.join("dist");
    let result = fs::write(output_dir.join(output_file), translated).map_err(|e| {
        format!(
            "{} Failed to write translated content: {}",
            "Error".bold().red(),
            e
        )
    });

    if let Err(err) = result {
        println!("{}", err);
    }
}

pub fn get_headers(config: &Config) -> String {
    format!(
        r#"
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <meta name="description" content="{}">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.1/css/catppuccin-frappe.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></script>
    {}
    "#,
        config.title,
        config.description,
        if config.content.latex_enabled {
            "<script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>"
        } else {
            ""
        }
    )
}

pub fn create_dom(translated: &str, config: &Config) -> String {
    let headers = get_headers(config);
    let styles = css_styles(config);

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            {}
        </head>
        <body>
            <div id="content">{}</div>
            <script>hljs.highlightAll();</script>
        </body>
        <style>
            {}
        </style>
        </html>
        "#,
        headers, translated, styles
    );

    html
}

pub fn css_styles(config: &Config) -> String {
    match config.theme.as_str() {
        "frappe" => {
            format!(
                r#"
                    :root {{
                        --base: #303446;
                        --mantle: #292c3c;
                        --crust: #232634;
                        --surface-0: #414559;
                        --surface-1: #51576d;
                        --surface-2: #626880;
                        --overlay-0: #737994;
                        --overlay-1: #838ba7;
                        --overlay-2: #949cbb;
                        --subtext-0: #a5adce;
                        --subtext-1: #b5bfe2;
                        --text: #c6d0f5;
                        --red: #e78284;
                    }}

                    html, body {{
                        margin: 0;
                        padding: 0;
                        font-family: Ubuntu, sans-serif;
                        background-color: var(--base);
                        color: var(--text);
                        overflow-x: hidden;
                    }}

                    #content {{
                        margin-left: 2rem;
                        margin-right: 2rem;
                        padding: 0;
                        height: 100vh;
                        display: flex;
                        flex-direction: column;

                    }}

                    pre {{
                        width: fit-content;
                        border-radius: 0.5rem;
                        padding: 1rem;
                        display: block;
                        background-color: var(--crust) !important;
                    }}

                    code {{
                        width: fit-content;
                        background-color: var(--crust) !important;
                    }}

                    hr {{
                        border: none;
                        border-top: 1px solid var(--overlay-1);
                        margin: 1rem 0;
                    }}

                    img {{
                        max-width: fit-content;
                        height: auto;
                    }}

                    video {{
                        max-width: 100%;
                        height: auto;
                    }}

                    a {{
                        color: var(--text);
                        text-decoration: none;
                    }}

                    a:hover {{
                        text-decoration: underline;
                    }}
                "#,
            )
        }
        _ => {
            format!(
                r#"
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/atom-one-dark.min.css">
                "#,
            )
        }
    }
}
