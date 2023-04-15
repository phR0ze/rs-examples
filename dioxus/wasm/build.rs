// Build script for the app
// -------------------------------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    build_css()?;
    Ok(())
}

// Recursively build out the top level compiled_styles.css from the various *.scss files
// -------------------------------------------------------------------------------------------------
use glob::glob;
use rsass::output;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

fn build_css() -> Result<(), Box<dyn Error>> {
    let scss_output = "./src/compiled_styles.css";
    let mut scss = File::create(scss_output)?;

    let mut contents = String::from("/* This file is automatically generated, edits will be overwritten. */\n");

    for entry in glob("src/**/*.scss").expect("Failed to read glob pattern") {
        let path = entry?;
        let data = fs::read_to_string(path)?;
        contents += data.as_ref();
    }

    let format = output::Format { style: output::Style::Compressed, ..Default::default() };
    let css = rsass::compile_scss(contents.as_bytes(), format)?;

    scss.write_all(&css)?;
    scss.flush()?;

    Ok(())
}
