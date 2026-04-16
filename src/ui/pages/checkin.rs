use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::services::CheckInService;

pub struct CheckInPage {
    pub widget: GtkBox,
    checkin_service: Arc<CheckInService>,
}

impl CheckInPage {
    pub fn new(checkin_service: Arc<CheckInService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Check In"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let add_btn = gtk::Button::from_icon_name("plus-symbolic");

        title_bar.append(&back_btn);
        title_bar.append(&title_label);
        title_bar.append(&add_btn);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let content_box = GtkBox::new(gtk::Orientation::Vertical, 16);
        content_box.set_margin_start(16);
        content_box.set_margin_end(16);
        content_box.set_margin_top(16);

        let location_entry = Entry::new();
        location_entry.set_placeholder_text(Some("Enter your location..."));
        location_entry.set_hexpand(true);

        let status_entry = Entry::new();
        status_entry.set_placeholder_text(Some("What's your status? (optional)"));
        status_entry.set_hexpand(true);

        let checkin_btn = Button::with_label("Check In Now");
        checkin_btn.add_css_class("suggested-action");
        checkin_btn.set_hexpand(true);

        let recent_label = Label::new(Some("Recent Check-ins"));
        recent_label.add_css_class("title-3");
        recent_label.set_halign(Align::Start);
        recent_label.set_margin_top(16);

        let checkins_list = ListBox::new();

        let placeholder = Label::new(Some("No recent check-ins"));
        placeholder.set_opacity(0.6);
        placeholder.set_halign(Align::Center);
        placeholder.set_margin_top(24);
        checkins_list.append(&placeholder);

        content_box.append(&location_entry);
        content_box.append(&status_entry);
        content_box.append(&checkin_btn);
        content_box.append(&recent_label);
        content_box.append(&checkins_list);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        CheckInPage {
            widget,
            checkin_service,
        }
    }
}
