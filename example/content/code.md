# Code
---

```rust
// Create the output directory if it doesn't exist
if let Err(err) = fs::create_dir_all(&output_dir) {
    println!(
        "{} Failed to create output directory: {}",
        "Error".bold().red(),
        err
    );
}

let result = fs::write(output_dir.join(output_file), translated).map_err(|e| {
    format!(
        "{} Failed to write translated content: {}",
        "Error".bold().red(),
        e
    )
});
```

Hello World!
