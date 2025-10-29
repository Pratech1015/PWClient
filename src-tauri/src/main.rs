#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager, WebviewUrl, WebviewWindowBuilder, Wry,
};

fn main() {
    tauri::Builder::<Wry>::default()
        .plugin(tauri_plugin_dialog::init()) // Enable dialog support
        .setup(|app| {
            // ===== File Menu =====
            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&MenuItemBuilder::with_id("back", "Back").accelerator("Alt+Left").build(app)?)
                .item(&MenuItemBuilder::with_id("reload", "Reload").accelerator("Ctrl+R").build(app)?)
                .item(&MenuItemBuilder::with_id("forward", "Forward").accelerator("Alt+Right").build(app)?)
                .item(&MenuItemBuilder::with_id("refresh", "Refresh").accelerator("F5").build(app)?)
                .item(&MenuItemBuilder::with_id("home", "Home").accelerator("Alt+Home").build(app)?)
                .item(&MenuItemBuilder::with_id("exit", "Exit").accelerator("Alt+F4").build(app)?)
                .build()?;

            // ===== Edit Menu =====
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .item(&MenuItemBuilder::with_id("copy", "Copy").accelerator("Ctrl+C").build(app)?)
                .item(&MenuItemBuilder::with_id("cut", "Cut").accelerator("Ctrl+X").build(app)?)
                .item(&MenuItemBuilder::with_id("paste", "Paste").accelerator("Ctrl+V").build(app)?)
                .item(&MenuItemBuilder::with_id("select_all", "Select All").accelerator("Ctrl+A").build(app)?)
                .item(&MenuItemBuilder::with_id("find", "Find in-page").accelerator("Ctrl+F").build(app)?)
                .build()?;

            // ===== View Menu =====
            let view_menu = SubmenuBuilder::new(app, "View")
                .item(&MenuItemBuilder::with_id("fullscreen", "Full Screen").accelerator("F11").build(app)?)
                .item(&MenuItemBuilder::with_id("zoom_in", "Zoom In").accelerator("Ctrl++").build(app)?)
                .item(&MenuItemBuilder::with_id("zoom_out", "Zoom Out").accelerator("Ctrl+-").build(app)?)
                .item(&MenuItemBuilder::with_id("reset_zoom", "Reset Zoom").accelerator("Ctrl+0").build(app)?)
                .build()?;

            // ===== Tools Menu =====
            let tools_menu = SubmenuBuilder::new(app, "Tools")
                .item(&MenuItemBuilder::with_id("downloads", "Downloads").accelerator("Ctrl+J").build(app)?)
                .item(&MenuItemBuilder::with_id("feature1", "Custom Feature 1 (WIP)").build(app)?)
                .item(&MenuItemBuilder::with_id("feature2", "Custom Feature 2 (WIP)").build(app)?)
                .build()?;

            // ===== Help Menu =====
            let help_menu = SubmenuBuilder::new(app, "Help")
                .item(&MenuItemBuilder::with_id("set_download_loc", "Set Download Location").build(app)?)
                .item(&MenuItemBuilder::with_id("about", "About PWClient").build(app)?)
                .item(&MenuItemBuilder::with_id("report", "Report Bugs (WIP)").build(app)?)
                .build()?;

            // ===== Final Menu =====
            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &edit_menu, &view_menu, &tools_menu, &help_menu])
                .build()?;

            // ===== Main Window =====
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("PWClient")
                .inner_size(1280.0, 720.0)
                .menu(menu)
                .build()?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                // ==== File Menu ====
                "exit" => {
                    app.exit(0);
                }
                "about" => {
                    // Open a small popup with about.html
                    if app.get_webview_window("about").is_none() {
                        let _ = WebviewWindowBuilder::new(app, "about", WebviewUrl::App("about.html".into()))
                            .title("About PWClient")
                            .inner_size(400.0, 300.0)
                            .resizable(false)
                            .decorations(true)
                            .center()
                            .build();
                    }
                }
                "reload" | "refresh" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("window.location.reload();");
                    }
                }
                "home" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("window.location.href = 'index.html';");
                    }
                }
                "back" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("history.back();");
                    }
                }
                "forward" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("history.forward();");
                    }
                }

                // ==== Edit Menu ====
                "copy" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.execCommand('copy');");
                    }
                }
                "cut" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.execCommand('cut');");
                    }
                }
                "paste" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.execCommand('paste');");
                    }
                }
                "select_all" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.execCommand('selectAll');");
                    }
                }
                "find" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("window.find('');");
                    }
                }

                // ==== View Menu ====
                "fullscreen" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval(
                            "if (!document.fullscreenElement) { document.documentElement.requestFullscreen(); } else { document.exitFullscreen(); }",
                        );
                    }
                }
                "zoom_in" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.body.style.zoom = (parseFloat(document.body.style.zoom || 1) + 0.1).toFixed(1);");
                    }
                }
                "zoom_out" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.body.style.zoom = (parseFloat(document.body.style.zoom || 1) - 0.1).toFixed(1);");
                    }
                }
                "reset_zoom" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("document.body.style.zoom = 1;");
                    }
                }

                // ==== Tools Menu ====
                "downloads" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("alert('Downloads page coming soon!');");
                    }
                }
                "feature1" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("alert('Custom Feature 1 - Work in Progress');");
                    }
                }
                "feature2" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("alert('Custom Feature 2 - Work in Progress');");
                    }
                }

                // ==== Help Menu ====
                "set_download_loc" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("alert('Set Download Location - Coming soon!');");
                    }
                }
                "report" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.eval("alert('Bug reporting feature coming soon!');");
                    }
                }

                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error running app");
}