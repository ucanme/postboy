//! Postboy - A Postman-like API testing tool
//!
//! Main entry point for the Postboy application.

use gpui::*;
use gpui_component::Root;

fn main() {
    // Initialize tokio runtime for database operations
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Initialize database and services before starting the app
    let (_db, collection_service) = rt.block_on(async {
        // Get current directory for persistent file-based storage
        let current_dir = std::env::current_dir()
            .expect("Failed to get current directory");

        let db_path = current_dir.join("postboy.db");

        println!("Initializing database at: {:?}", db_path);

        // Initialize file-based database in current directory
        let db = postboy_store::open_store(postboy_store::StoreConfig {
            db_path: db_path.to_str().unwrap().to_string(),
            ..Default::default()
        }).await.unwrap();

        // Create the collection service
        let collection_service = postboy_service::collection::CollectionService::new(db.clone());

        (db, collection_service)
    });

    let app = Application::new();

    app.run(move |cx| {
        // Initialize gpui-component
        gpui_component::init(cx);

        // Open the main window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point { x: px(100.0), y: px(100.0) },
                    size: gpui::Size { width: px(1400.0), height: px(900.0) },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Postboy".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| postboy_ui::layout::main_window::MainWindow::new(
                    window,
                    collection_service,
                    cx
                ));
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            },
        ).unwrap();
    });
}
