use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

// 应用菜单项
pub fn init() -> Menu {
    // let _name = &context.package_info().name;
    // let app_menu = Submenu::new("原生菜单", Menu::new().add_native_item(MenuItem::Quit));
    let file_menu = Submenu::new(
        "文件",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "site_manager".to_string(),
                "站点管理器",
            ))
            .add_item(CustomMenuItem::new(
                "add_current_to_site".to_string(),
                "添加当前连接到站点管理器",
            ))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "新标签"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "关闭标签"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "导出"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "导入"))
            .add_item(CustomMenuItem::new(
                "edit_file".to_string(),
                "显示正在被编辑的文件",
            ))
            .add_item(CustomMenuItem::new("quit".to_string(), "退出")),
    );
    let edit_menu = Submenu::new(
        "编辑",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "网络配置向导"))
            .add_item(CustomMenuItem::new(
                "erase_personal_information".to_string(),
                "清除个人信息",
            ))
            .add_item(CustomMenuItem::new("redo".to_string(), "设置")),
    );
    let view_menu = Submenu::new(
        "查看",
        Menu::new()
            .add_item(CustomMenuItem::new("refresh_page".to_string(), "刷新"))
            .add_item(CustomMenuItem::new("redo".to_string(), "目录列表过滤器")),
    );
    let transfer_menu = Submenu::new(
        "传输",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "处理队列"))
            .add_item(CustomMenuItem::new(
                "redo".to_string(),
                "对已存在文件的默认操作",
            )),
    );
    let server_menu = Submenu::new(
        "服务器",
        Menu::new().add_item(CustomMenuItem::new("undo".to_string(), "取消当前操作")),
    );
    let lable_menu = Submenu::new(
        "书签",
        Menu::new()
            .add_item(CustomMenuItem::new("add_label".to_string(), "添加书签"))
            .add_item(CustomMenuItem::new("label_manager".to_string(), "书签管理")),
    );
    let help_menu = Submenu::new(
        "帮助",
        Menu::new().add_item(CustomMenuItem::new("add_label".to_string(), "检查更新")),
    );
    let version_menu = Submenu::new(
        "新版本",
        Menu::new().add_item(CustomMenuItem::new("add_label".to_string(), "版本")),
    );

    Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(transfer_menu)
        .add_submenu(server_menu)
        .add_submenu(lable_menu)
        .add_submenu(help_menu)
        .add_submenu(version_menu)
}

// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    // 菜单所属的窗口
    let win = Some(event.window());
    // 匹配菜单 id
    match event.menu_item_id() {
        "site_manager" => {
            let _ = win.unwrap().emit("site_manager", "打开站点管理器").unwrap();
        }
        "refresh_page" => {
            let _ = win.unwrap().emit("refresh_page", "刷新整个页面").unwrap();
        }
        "quit" => {
            let _ = win.unwrap().close();
        }
        "add_label" => {
            let _ = win.unwrap().emit("add_label", "添加标签").unwrap();
        }
        "label_manager" => {
            let _ = win.unwrap().emit("label_manager", "标签管理").unwrap();
        }
        "add_current_to_site" => {
            let _ = win
                .unwrap()
                .emit("add_current_to_site", "添加当前连接到站点管理器")
                .unwrap();
        }
        "erase_personal_information" => {
            let _ = win
                .unwrap()
                .emit("erase_personal_information", "清除个人信息")
                .unwrap();
        }
        "edit_file" => {}
        "undo" => {
            dbg!("undo");
        }
        _ => {}
    }
}
