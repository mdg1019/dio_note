mod files;
mod global;
mod window_menu;

use dioxus::prelude::*;
use dioxus_desktop::{Config};

#[derive(Clone)]
struct TextEditorState {
    content: String,
    filename: String,
}

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
    let text_editor_state = use_signal(|| TextEditorState{
        content: String::new(),
        filename: String::new(),
    });
    use_context_provider(|| text_editor_state);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        TextEditor {}
    }
}
#[component]
pub fn TextEditor() -> Element {
    let mut text_editor_state = use_context::<Signal<TextEditorState>>();

    rsx! {
        div {
            class: "text-editor",
            div {
                input {
                    r#type: "text",
                    value: "{text_editor_state.read().filename}",
                    oninput: move |e| text_editor_state.write().filename = e.value(),
                    placeholder: "Filename"
                }
                button {
                    onclick: move |_| {
                        let state = text_editor_state.read();
                        if let Err(e) = save_file(&state.filename, &state.content) {
                            eprintln!("Error saving file: {}", e);
                        }
                    },
                    "Save"
                }
                button {
                    onclick: move |_| {
                        let filename = text_editor_state.read().filename.clone();
                        match load_file(&filename) {
                            Ok(text) => text_editor_state.write().content = text,
                            Err(e) => eprintln!("Error loading file: {}", e),
                        }
                    },
                    "Load"
                }
                textarea {
                    class: "editor-area",
                    value: "{text_editor_state.read().content}",
                    oninput: move |e| text_editor_state.write().content = e.value(),
                    rows: "20",
                    cols: "80",
                }
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
