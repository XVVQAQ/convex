#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod router;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let router: Rc<RefCell<router::Router<Route>>> =
        Rc::new(RefCell::new(router::Router::new(Route::Home)));

    ui.set_current_route(router.borrow().current());

    let r = router.clone();
    let w = ui.as_weak();
    ui.on_navigate_to(move |route| {
        // 主页面清空栈，子页面正常入栈
        match route {
            Route::Home | Route::Favorite => {
                r.borrow_mut().navigate_to_root(route);
            }
            _ => {
                r.borrow_mut().navigate_to(route);
            }
        }
        if let Some(ui) = w.upgrade() {
            ui.set_current_route(r.borrow().current());
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

    println!("App started — routing system ready");
    ui.run()?;

    Ok(())
}
