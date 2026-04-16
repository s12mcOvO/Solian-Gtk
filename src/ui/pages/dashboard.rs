use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Label, ScrolledWindow};

pub struct DashboardPage {
    pub widget: GtkBox,
}

impl DashboardPage {
    pub fn new(
        on_logout: impl Fn() + 'static,
        on_chat: impl Fn() + 'static,
        on_settings: impl Fn() + 'static,
        on_posts: impl Fn() + 'static,
        on_realms: impl Fn() + 'static,
        on_thoughts: impl Fn() + 'static,
        on_wallets: impl Fn() + 'static,
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let title_label = Label::new(Some("Solian"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let settings_btn = gtk::Button::from_icon_name("emblem-system-symbolic");
        settings_btn.set_valign(Align::Center);
        settings_btn.connect_clicked(move |_| {
            on_settings();
        });

        title_bar.append(&title_label);
        title_bar.append(&settings_btn);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let content_box = GtkBox::new(gtk::Orientation::Vertical, 16);
        content_box.set_margin_top(16);
        content_box.set_margin_bottom(16);
        content_box.set_margin_start(16);
        content_box.set_margin_end(16);
        content_box.set_width_request(400);
        content_box.set_halign(Align::Center);

        let welcome_label = Label::new(Some("Welcome to Solian!"));
        welcome_label.add_css_class("title-2");
        welcome_label.set_halign(Align::Center);

        let desc_label = Label::new(Some("Your Solar Network Desktop Client"));
        desc_label.set_opacity(0.7);
        desc_label.set_halign(Align::Center);

        let menu_box = GtkBox::new(gtk::Orientation::Vertical, 8);
        menu_box.set_margin_top(24);

        let messages_btn = create_menu_button("Messages", "chat-symbolic");
        messages_btn.connect_clicked(move |_| on_chat());

        let posts_btn = create_menu_button("Posts", "document-symbolic");
        posts_btn.connect_clicked(move |_| on_posts());

        let realms_btn = create_menu_button("Realms", "users-symbolic");
        realms_btn.connect_clicked(move |_| on_realms());

        let thoughts_btn = create_menu_button("Thoughts", "lightbulb-symbolic");
        thoughts_btn.connect_clicked(move |_| on_thoughts());

        let wallets_btn = create_menu_button("Wallets", "wallet-symbolic");
        wallets_btn.connect_clicked(move |_| on_wallets());

        let logout_button = Button::with_label("Logout");
        logout_button.add_css_class("destructive-action");
        logout_button.set_margin_top(24);
        logout_button.connect_clicked(move |_| {
            on_logout();
        });

        menu_box.append(&messages_btn);
        menu_box.append(&posts_btn);
        menu_box.append(&realms_btn);
        menu_box.append(&thoughts_btn);
        menu_box.append(&wallets_btn);

        content_box.append(&welcome_label);
        content_box.append(&desc_label);
        content_box.append(&menu_box);
        content_box.append(&logout_button);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        DashboardPage { widget }
    }
}

fn create_menu_button(label: &str, icon: &str) -> gtk::Button {
    let btn = gtk::Button::new();
    btn.set_halign(Align::Start);
    btn.set_hexpand(true);
    btn.set_size_request(200, 44);

    let icon_widget = gtk::Image::from_icon_name(icon);
    icon_widget.set_icon_size(gtk::IconSize::Large);

    let button_box = GtkBox::new(gtk::Orientation::Horizontal, 12);
    button_box.append(&icon_widget);
    button_box.append(&gtk::Label::new(Some(label)));

    btn.set_child(Some(&button_box));
    btn
}
