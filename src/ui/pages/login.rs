use adw::prelude::*;
use gtk::prelude::*;
use gtk::{Align, Entry};
use std::sync::Arc;
use tracing::info;

use crate::core::services::AuthService;

pub struct LoginPage {
    pub widget: gtk::Box,
}

impl LoginPage {
    pub fn new(
        auth_service: &Arc<AuthService>,
        _window: &adw::ApplicationWindow,
        on_login_success: Arc<dyn Fn()>,
    ) -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 24);
        widget.set_halign(Align::Center);
        widget.set_valign(Align::Center);
        widget.set_margin_top(48);
        widget.set_margin_bottom(48);
        widget.set_margin_start(48);
        widget.set_margin_end(48);

        let header = gtk::Label::new(Some("Solian"));
        header.add_css_class("title-1");
        header.set_halign(Align::Center);

        let subtitle = gtk::Label::new(Some("Solar Network Desktop"));
        subtitle.add_css_class("subtitle");
        subtitle.set_halign(Align::Center);
        subtitle.set_opacity(0.7);

        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 24);
        content_box.set_halign(Align::Center);
        content_box.set_valign(Align::Center);
        content_box.set_width_request(300);

        let username_entry = Entry::new();
        username_entry.set_placeholder_text(Some("Username"));
        username_entry.set_width_chars(30);

        let password_entry = Entry::new();
        password_entry.set_placeholder_text(Some("Password"));
        password_entry.set_visibility(false);
        password_entry.set_width_chars(30);

        let error_label = gtk::Label::new(None);
        error_label.add_css_class("error");
        error_label.set_visible(false);
        error_label.set_halign(Align::Center);

        let login_button = gtk::Button::with_label("Login");
        login_button.add_css_class("suggested-action");
        login_button.set_halign(Align::Center);
        login_button.set_hexpand(false);
        login_button.set_size_request(200, -1);

        let spinner = gtk::Spinner::new();
        spinner.set_visible(false);
        spinner.set_halign(Align::Center);

        let spinner_for_click = spinner.clone();
        let error_label_for_click = error_label.clone();
        let login_button_for_state = login_button.clone();

        let username_clone = username_entry.clone();
        let password_clone = password_entry.clone();

        let auth_service_clone = auth_service.clone();
        let on_login_success_clone = on_login_success.clone();

        login_button.connect_clicked(move |_| {
            let username = username_clone.text().to_string();
            let password = password_clone.text().to_string();

            if username.is_empty() || password.is_empty() {
                error_label_for_click.set_text("Please fill in all fields");
                error_label_for_click.set_visible(true);
                return;
            }

            error_label_for_click.set_visible(false);
            spinner_for_click.set_visible(true);
            spinner_for_click.start();
            login_button_for_state.set_sensitive(false);

            let auth_service_inner = auth_service_clone.clone();
            let error_label_err = error_label_for_click.clone();
            let spinner_err = spinner_for_click.clone();
            let login_btn_err = login_button_for_state.clone();

            let rt = tokio::runtime::Runtime::new().unwrap();
            match rt.block_on(auth_service_inner.login(&username, &password)) {
                Ok(_user) => {
                    info!("Login successful");
                    on_login_success_clone();
                }
                Err(e) => {
                    eprintln!("Login failed: {}", e);
                    error_label_err.set_text("Login failed. Please check your credentials.");
                    error_label_err.set_visible(true);
                    spinner_err.set_visible(false);
                    spinner_err.stop();
                    login_btn_err.set_sensitive(true);
                }
            }
        });

        content_box.append(&header);
        content_box.append(&subtitle);
        content_box.append(&username_entry);
        content_box.append(&password_entry);
        content_box.append(&error_label);
        content_box.append(&login_button);
        content_box.append(&spinner);

        widget.append(&content_box);

        Self { widget }
    }
}
