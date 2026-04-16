use adw::prelude::*;
use adw::Application;
use std::sync::Arc;
use tracing::info;

use crate::core::services::AuthService;
use crate::core::services::ChatService;
use crate::core::services::PostsService;
use crate::ui::pages::{
    ChatPage, DashboardPage, LoginPage, PostsPage, RealmsPage, SettingsPage, ThoughtsPage,
    WalletsPage,
};

pub fn setup_and_run(app: &Application) -> anyhow::Result<()> {
    let api_client = Arc::new(crate::core::network::ApiClient::new());
    let auth_service = Arc::new(AuthService::new(api_client.clone()));
    let chat_service = Arc::new(ChatService::new(api_client.clone()));
    let posts_service = Arc::new(PostsService::new(api_client));

    let main_window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Solian")
        .default_width(1200)
        .default_height(800)
        .build();

    let content_stack = gtk::Stack::new();
    content_stack.set_transition_type(gtk::StackTransitionType::Crossfade);

    let login_page = LoginPage::new(
        &auth_service,
        &main_window,
        Arc::new({
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("dashboard")
        }),
    );
    content_stack.add_named(&login_page.widget, Some("login"));

    let dashboard_page = DashboardPage::new(
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("login")
        },
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("chat")
        },
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("settings")
        },
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("posts")
        },
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("realms")
        },
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("thoughts")
        },
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("wallets")
        },
    );
    content_stack.add_named(&dashboard_page.widget, Some("dashboard"));

    let chat_page = ChatPage::new(
        chat_service,
        {
            let stack = content_stack.clone();
            move || stack.set_visible_child_name("dashboard")
        },
        |_room_id| {},
    );
    content_stack.add_named(&chat_page.widget, Some("chat"));

    let settings_page = SettingsPage::new({
        let stack = content_stack.clone();
        move || stack.set_visible_child_name("dashboard")
    });
    content_stack.add_named(&settings_page.widget, Some("settings"));

    let posts_page = PostsPage::new(posts_service, {
        let stack = content_stack.clone();
        move || stack.set_visible_child_name("dashboard")
    });
    content_stack.add_named(&posts_page.widget, Some("posts"));

    let realms_page = RealmsPage::new({
        let stack = content_stack.clone();
        move || stack.set_visible_child_name("dashboard")
    });
    content_stack.add_named(&realms_page.widget, Some("realms"));

    let thoughts_page = ThoughtsPage::new({
        let stack = content_stack.clone();
        move || stack.set_visible_child_name("dashboard")
    });
    content_stack.add_named(&thoughts_page.widget, Some("thoughts"));

    let wallets_page = WalletsPage::new({
        let stack = content_stack.clone();
        move || stack.set_visible_child_name("dashboard")
    });
    content_stack.add_named(&wallets_page.widget, Some("wallets"));

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.append(&content_stack);

    main_window.set_content(Some(&main_box));

    content_stack.set_visible_child_name("login");
    main_window.present();
    info!("GTK4 UI started successfully");

    Ok(())
}
