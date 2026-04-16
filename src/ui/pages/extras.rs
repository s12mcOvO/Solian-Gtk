use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Entry, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::services::CountdownService;

pub struct CountdownPage {
    pub widget: GtkBox,
    countdown_service: Arc<CountdownService>,
}

impl CountdownPage {
    pub fn new(countdown_service: Arc<CountdownService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Countdowns"));
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

        let countdowns_label = Label::new(Some("Your Countdowns"));
        countdowns_label.add_css_class("title-3");
        countdowns_label.set_halign(Align::Start);

        let countdowns_list = ListBox::new();

        let placeholder = Label::new(Some("No countdowns yet. Create one!"));
        placeholder.set_opacity(0.6);
        placeholder.set_halign(Align::Center);
        placeholder.set_margin_top(24);
        countdowns_list.append(&placeholder);

        content_box.append(&countdowns_label);
        content_box.append(&countdowns_list);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        CountdownPage {
            widget,
            countdown_service,
        }
    }
}

pub struct RssPage {
    pub widget: GtkBox,
    rss_service: Arc<crate::core::services::RssService>,
}

impl RssPage {
    pub fn new(
        rss_service: Arc<crate::core::services::RssService>,
        on_back: impl Fn() + 'static,
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("RSS Feeds"));
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

        let feeds_label = Label::new(Some("Your RSS Feeds"));
        feeds_label.add_css_class("title-3");
        feeds_label.set_halign(Align::Start);

        let feeds_list = ListBox::new();

        let placeholder = Label::new(Some("No RSS feeds. Add one!"));
        placeholder.set_opacity(0.6);
        placeholder.set_halign(Align::Center);
        placeholder.set_margin_top(24);
        feeds_list.append(&placeholder);

        content_box.append(&feeds_label);
        content_box.append(&feeds_list);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        RssPage {
            widget,
            rss_service,
        }
    }
}
