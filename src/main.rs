#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod log;
mod router;

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Context;
use slint::{Model, include_modules};

include_modules!();

fn main() -> anyhow::Result<()> {
    log::init_from_env().context("failed to init logger")?;

    let ui = AppWindow::new().context("failed to create window")?;

    push_log_to_ui(&ui);

    let router: Rc<RefCell<router::Router<Route>>> =
        Rc::new(RefCell::new(router::Router::new(Route::Home)));

    ui.set_current_route(router.borrow().current());

    let r = router.clone();
    let w = ui.as_weak();
    ui.on_navigate_to(move |route| {
        // 从 Slint 读取 root-routes，判断是否为主页面
        let is_root = w.upgrade().is_some_and(|ui| {
            let model = ui.get_root_routes();
            (0..model.row_count()).any(|i| model.row_data(i) == Some(route))
        });
        r.borrow_mut().navigate_to(route, is_root);
        if let Some(ui) = w.upgrade() {
            ui.set_current_route(r.borrow().current());
            if route == Route::Log {
                push_log_to_ui(&ui);
            }
        }
    });

    let r = router.clone();
    let w = ui.as_weak();
    ui.on_go_back(move || {
        r.borrow_mut().go_back();
        if let Some(ui) = w.upgrade() {
            ui.set_current_route(r.borrow().current());
        }
    });

    log::log_info!("comic app started");
    ui.run().context("app window run failed")?;

    Ok(())
}

fn push_log_to_ui(ui: &AppWindow) {
    let entries: Vec<slint::SharedString> = log::recent_entries()
        .iter()
        .map(|s| slint::SharedString::from(s.as_str()))
        .collect();
    let model = std::rc::Rc::new(slint::VecModel::from(entries));
    ui.set_app_log(model.into());
    log::log_info!("pushed {} log entries to ui", log::recent_entries().len());
}
