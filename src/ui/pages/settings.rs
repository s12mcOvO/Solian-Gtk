use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Label, ScrolledWindow, Switch};

pub struct SettingsPage {
    pub widget: GtkBox,
}

impl SettingsPage {
    pub fn new(on_back: impl Fn() + 'static, on_logout: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_button = gtk::Button::from_icon_name("go-previous-symbolic");
        back_button.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Settings"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        title_bar.append(&back_button);
        title_bar.append(&title_label);

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

        let appearance_label = Label::new(Some("Appearance"));
        appearance_label.add_css_class("title-3");
        appearance_label.set_halign(Align::Start);

        let dark_mode_box = create_switch_row("Dark Mode", false);
        let animations_box = create_switch_row("Animations", true);

        let behavior_label = Label::new(Some("Behavior"));
        behavior_label.add_css_class("title-3");
        behavior_label.set_halign(Align::Start);
        behavior_label.set_margin_top(16);

        let sound_box = create_switch_row("Sound Effects", true);
        let notifications_box = create_switch_row("Push Notifications", true);

        let about_label = Label::new(Some("About"));
        about_label.add_css_class("title-3");
        about_label.set_halign(Align::Start);
        about_label.set_margin_top(16);

        let version_label = Label::new(Some("Version 0.1.0"));
        version_label.set_opacity(0.7);
        version_label.set_halign(Align::Start);
        version_label.set_margin_top(8);

        let logout_btn = Button::with_label("Logout");
        logout_btn.add_css_class("destructive-action");
        logout_btn.set_margin_top(24);
        logout_btn.set_hexpand(true);
        logout_btn.connect_clicked(move |_| {
            on_logout();
        });

        content_box.append(&appearance_label);
        content_box.append(&dark_mode_box);
        content_box.append(&animations_box);
        content_box.append(&behavior_label);
        content_box.append(&sound_box);
        content_box.append(&notifications_box);
        content_box.append(&about_label);
        content_box.append(&version_label);
        content_box.append(&logout_btn);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        SettingsPage { widget }
    }
}

fn create_switch_row(label_text: &str, default_active: bool) -> GtkBox {
    let switch_box = GtkBox::new(gtk::Orientation::Horizontal, 8);
    switch_box.set_margin_top(8);
    switch_box.set_margin_bottom(8);

    let label = Label::new(Some(label_text));
    label.set_hexpand(true);

    let switch = Switch::new();
    switch.set_active(default_active);

    switch_box.append(&label);
    switch_box.append(&switch);

    switch_box
}
