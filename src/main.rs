mod files;
mod global;
mod window_menu;

use dioxus::prelude::*;
use dioxus_desktop::{Config};


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    let result = run();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let global = global::Global::init()?;
    println!("Documents Directory: {:?}", global.documents_dir);
    println!("Current Directory: {:?}", global.current_dir);

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_menu(Some(window_menu::build_menu()?)))
        .launch(App);

    Ok(())
}

#[component]
fn App() -> Element {  
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        TextEditor {}
    }
}

#[component]
pub fn TextEditor() -> Element {
    let mut content = use_signal(|| String::new());
    let mut filename = use_signal(|| String::from("untitled.txt"));

    rsx! {
        div {
            class: "text-editor",
            div {
                class: "toolbar",
                input {
                    r#type: "text",
                    value: "{filename}",
                    oninput: move |e| filename.set(e.value()),
                    placeholder: "Filename"
                }
                button {
                    onclick: move |_| {
                        if let Err(e) = save_file(&filename(), &content()) {
                            eprintln!("Error saving file: {}", e);
                        }
                    },
                    "Save"
                }
                button {
                    onclick: move |_| {
                        match load_file(&filename()) {
                            Ok(text) => content.set(text),
                            Err(e) => eprintln!("Error loading file: {}", e),
                        }
                    },
                    "Load"
                }
            }
            textarea {
                class: "editor-area",
                value: "{content}",
                oninput: move |e| content.set(e.value()),
                rows: "20",
                cols: "80"
            }
        }
    }
}

fn save_file(filename: &str, content: &str) -> std::io::Result<()> {
    use std::fs;
    let config = global::Global::get().expect("Config not initialized");
    let file_path = config.documents_dir.join(filename);
    fs::write(file_path, content)
}

fn load_file(filename: &str) -> std::io::Result<String> {
    use std::fs;
    let config = global::Global::get().expect("Config not initialized");
    let file_path = config.documents_dir.join(filename);
    fs::read_to_string(file_path)
}
