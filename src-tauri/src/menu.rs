use tauri::utils::assets::EmbeddedAssets;
use tauri::{Context, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

// 应用菜单项
pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
    let _name = &context.package_info().name;
    let app_menu = Submenu::new("原生菜单", Menu::new().add_native_item(MenuItem::Quit));
    let file_menu = Submenu::new(
        "文件",
        Menu::new()
            .add_item(CustomMenuItem::new("new_file".to_string(), "站点管理器"))
            .add_item(CustomMenuItem::new(
                "new_file".to_string(),
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
            .add_item(CustomMenuItem::new("edit_file".to_string(), "退出")),
    );
    let edit_menu = Submenu::new(
        "编辑",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "网络配置向导"))
            .add_item(CustomMenuItem::new("redo".to_string(), "清除个人信息"))
            .add_item(CustomMenuItem::new("redo".to_string(), "设置")),
    );
    let view_menu = Submenu::new(
        "查看",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "刷新"))
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

    Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(transfer_menu)
        .add_submenu(app_menu)
}

// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    // 菜单所属的窗口
    let _win = Some(event.window());
    // 匹配菜单 id
    match event.menu_item_id() {
        "new_file" => {
            dbg!("new file");
        }
        "edit_file" => {}
        "undo" => {
            dbg!("undo");
        }
        "redo" => {
            dbg!("redo");
        }
        _ => {}
    }
}
