use serde::Serialize;
use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
    Menu::new()
        .add_submenu(create_new_menu())
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

fn create_new_menu() -> Submenu {
    let new_node = CustomMenuItem::new("new_node".to_string(), "Node");
    let new_category = CustomMenuItem::new("new_category".to_string(), "Category");

    Submenu::new("New", Menu::new().add_item(new_node).add_item(new_category))
}


/// A simple struct which has the option of containing contents
/// The primary requirement isn't so much that the frontend has a payload to work with
/// It just needs to know that something was emitted tied to a certain id.
#[derive(Clone, Serialize)]
struct MenuCommandPayload(Option<String>);

/// Emits signals depending on which menu item was click.
/// E.g. "New/Node" menu item clicked => "new_node" signal emitted => Frontend listens => Frontend redirects
/// Emitted signals are tied to the window in question. 
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
        "new_node" => event
            .window()
            .emit("new_node", MenuCommandPayload(None))
            .expect("Command new node failed from menu"),
        "new_category" => event
            .window()
            .emit("new_category", MenuCommandPayload(None))
            .expect("Command new category failed from menu"),
        _ => {}
    };
}
