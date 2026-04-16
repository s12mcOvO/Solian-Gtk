use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Entry, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::services::PublisherService;

pub struct SearchPage {
    pub widget: GtkBox,
    publisher_service: Arc<PublisherService>,
}

impl SearchPage {
    pub fn new(
        publisher_service: Arc<PublisherService>,
        on_back: impl Fn() + 'static,
        _on_select_user: fn(String),
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Search"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        title_bar.append(&back_btn);
        title_bar.append(&title_label);

        let search_box = GtkBox::new(gtk::Orientation::Vertical, 16);
        search_box.set_margin_start(16);
        search_box.set_margin_end(16);
        search_box.set_margin_top(16);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Search users, posts..."));
        search_entry.set_hexpand(true);

        let results_list = ListBox::new();
        results_list.add_css_class("search-results");

        let results_scroll = ScrolledWindow::new();
        results_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        results_scroll.set_vexpand(true);
        results_scroll.set_child(Some(&results_list));

        let placeholder_label = Label::new(Some("Enter a search query to find users and posts"));
        placeholder_label.set_opacity(0.6);
        placeholder_label.set_halign(Align::Center);
        results_list.append(&placeholder_label);

        search_box.append(&search_entry);
        search_box.append(&results_scroll);

        widget.append(&title_bar);
        widget.append(&search_box);

        SearchPage {
            widget,
            publisher_service,
        }
    }
}

fn create_search_result_row(name: &str, bio: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("search-result-row");

    let container = GtkBox::new(gtk::Orientation::Horizontal, 12);
    container.set_margin_start(12);
    container.set_margin_end(12);
    container.set_margin_top(8);
    container.set_margin_bottom(8);

    let avatar = gtk::Image::from_icon_name("avatar-default-symbolic");
    avatar.set_width_request(40);
    avatar.set_height_request(40);

    let info_box = GtkBox::new(gtk::Orientation::Vertical, 2);
    info_box.set_hexpand(true);

    let name_label = Label::new(Some(name));
    name_label.add_css_class("search-result-name");
    name_label.set_halign(Align::Start);
    name_label.set_ellipsize(pango::EllipsizeMode::End);

    let bio_label = Label::new(Some(bio));
    bio_label.add_css_class("search-result-bio");
    bio_label.set_opacity(0.6);
    bio_label.set_halign(Align::Start);
    bio_label.set_ellipsize(pango::EllipsizeMode::End);

    info_box.append(&name_label);
    info_box.append(&bio_label);

    container.append(&avatar);
    container.append(&info_box);

    row.set_child(Some(&container));
    row
}
