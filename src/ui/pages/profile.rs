use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Entry, Image, Label, ScrolledWindow};
use std::sync::Arc;

use crate::core::services::PublisherService;

pub struct ProfilePage {
    pub widget: GtkBox,
    publisher_service: Arc<PublisherService>,
}

impl ProfilePage {
    pub fn new(
        publisher_service: Arc<PublisherService>,
        on_back: impl Fn() + 'static,
        on_edit: impl Fn() + 'static,
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Profile"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        title_bar.append(&back_btn);
        title_bar.append(&title_label);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let profile_box = GtkBox::new(gtk::Orientation::Vertical, 24);
        profile_box.set_margin_top(24);
        profile_box.set_margin_bottom(24);
        profile_box.set_margin_start(24);
        profile_box.set_margin_end(24);
        profile_box.set_halign(Align::Center);
        profile_box.set_width_request(500);

        let avatar = Image::from_icon_name("avatar-default-symbolic");
        avatar.set_width_request(100);
        avatar.set_height_request(100);
        avatar.add_css_class("profile-avatar");

        let name_label = Label::new(Some("Loading..."));
        name_label.add_css_class("title-1");
        name_label.set_halign(Align::Center);

        let username_label = Label::new(Some("@user"));
        username_label.add_css_class("subtitle");
        username_label.set_opacity(0.7);
        username_label.set_halign(Align::Center);

        let bio_label = Label::new(Some("Bio loading..."));
        bio_label.set_wrap(true);
        bio_label.set_xalign(0.0);
        bio_label.set_halign(Align::Center);

        let stats_box = GtkBox::new(gtk::Orientation::Horizontal, 32);
        stats_box.set_halign(Align::Center);
        stats_box.set_margin_top(16);

        let posts_stat = create_stat_item("0", "Posts");
        let followers_stat = create_stat_item("0", "Followers");
        let following_stat = create_stat_item("0", "Following");

        stats_box.append(&posts_stat.0);
        stats_box.append(&followers_stat.0);
        stats_box.append(&following_stat.0);

        let edit_btn = Button::with_label("Edit Profile");
        edit_btn.add_css_class("suggested-action");
        edit_btn.set_margin_top(16);
        edit_btn.set_halign(Align::Center);

        let edit_overlay = GtkBox::new(gtk::Orientation::Vertical, 16);
        edit_overlay.set_halign(Align::Center);

        profile_box.append(&avatar);
        profile_box.append(&name_label);
        profile_box.append(&username_label);
        profile_box.append(&bio_label);
        profile_box.append(&stats_box);
        profile_box.append(&edit_btn);

        main_content.set_child(Some(&profile_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        ProfilePage {
            widget,
            publisher_service,
        }
    }
}

fn create_stat_item(value: &str, label_text: &str) -> (gtk::Box, Label) {
    let stat_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
    stat_box.set_halign(Align::Center);

    let value_label = Label::new(Some(value));
    value_label.add_css_class("title-3");
    value_label.set_halign(Align::Center);

    let label = Label::new(Some(label_text));
    label.add_css_class("subtitle");
    label.set_opacity(0.7);
    label.set_halign(Align::Center);

    stat_box.append(&value_label);
    stat_box.append(&label);

    (stat_box, value_label)
}
