use serde::Serialize;
use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
    Menu::new()
        .add_item(CustomMenuItem::new("new".to_string(), "New"))
        .add_submenu(create_manage_menu())
        .add_item(CustomMenuItem::new("overview".to_string(), "Overview"))
        .add_item(CustomMenuItem::new("about".to_string(), "About"))
}

fn create_manage_menu() -> Submenu {
    let import = CustomMenuItem::new("import".to_string(), "Import");
    let sync = CustomMenuItem::new("sync".to_string(), "Sync");
    let backup = CustomMenuItem::new("backup".to_string(), "Backup");

    Submenu::new(
        "Manage",
        Menu::new().add_item(import).add_item(sync).add_item(backup),
    )
}

#[derive(Clone, Serialize)]
struct MenuCommandPayload(Option<String>);

pub fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "new" => event
            .window()
            .emit("new", MenuCommandPayload(None))
            .expect("Command new failed from menu"),
        "overview" => event
            .window()
            .emit("overview", MenuCommandPayload(None))
            .expect("Command overview failed from menu"),
        "about" => event
            .window()
            .emit("about", MenuCommandPayload(None))
            .expect("Command about failed from menu"),
        "import" => event
            .window()
            .emit("import", MenuCommandPayload(None))
            .expect("Command import failed from menu"),
        "sync" => event
            .window()
            .emit("sync", MenuCommandPayload(None))
            .expect("Command sync failed from menu"),
        "backup" => event
            .window()
            .emit("backup", MenuCommandPayload(None))
            .expect("Command backup failed from menu"),
        _ => {}
    };
}
