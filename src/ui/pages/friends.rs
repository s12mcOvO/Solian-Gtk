use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::services::FriendsService;

pub struct FriendsPage {
    pub widget: GtkBox,
    friends_service: Arc<FriendsService>,
}

impl FriendsPage {
    pub fn new(friends_service: Arc<FriendsService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Friends"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        title_bar.append(&back_btn);
        title_bar.append(&title_label);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let friends_list = ListBox::new();
        friends_list.add_css_class("friends-list");

        let placeholder = Label::new(Some("No friends yet. Start following people!"));
        placeholder.set_opacity(0.6);
        placeholder.set_halign(Align::Center);
        placeholder.set_margin_top(48);
        friends_list.append(&placeholder);

        main_content.set_child(Some(&friends_list));

        widget.append(&title_bar);
        widget.append(&main_content);

        FriendsPage {
            widget,
            friends_service,
        }
    }
}

pub struct BlocklistPage {
    pub widget: GtkBox,
    friends_service: Arc<FriendsService>,
}

impl BlocklistPage {
    pub fn new(friends_service: Arc<FriendsService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Blocked Users"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        title_bar.append(&back_btn);
        title_bar.append(&title_label);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let blocklist = ListBox::new();
        blocklist.add_css_class("blocklist");

        let placeholder = Label::new(Some("No blocked users"));
        placeholder.set_opacity(0.6);
        placeholder.set_halign(Align::Center);
        placeholder.set_margin_top(48);
        blocklist.append(&placeholder);

        main_content.set_child(Some(&blocklist));

        widget.append(&title_bar);
        widget.append(&main_content);

        BlocklistPage {
            widget,
            friends_service,
        }
    }
}
