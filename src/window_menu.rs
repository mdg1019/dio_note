use muda::{Menu, Submenu, MenuItem, MenuEvent, accelerator::{Accelerator, Modifiers, Code}};

pub fn build_menu() -> Result<Menu, Box<dyn std::error::Error>> {
    let menu = Menu::new();
    let file_submenu = Submenu::new("File", true);

    let open_item = MenuItem::new("Open", true, Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyO)));
    file_submenu.append(&open_item)?;

    menu.append(&file_submenu)?;

    let open_id = open_item.id().clone();

    MenuEvent::set_event_handler(Some(move |evt: MenuEvent| {
        if evt.id() == &open_id {
            println!("Open menu item clicked");
        }
    }));

    Ok(menu)
}