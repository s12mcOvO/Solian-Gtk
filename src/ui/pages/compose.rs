use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Entry, Label, TextView};
use std::sync::Arc;

use crate::core::services::PostsService;

pub struct ComposePage {
    pub widget: GtkBox,
    posts_service: Arc<PostsService>,
}

impl ComposePage {
    pub fn new(posts_service: Arc<PostsService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("New Post"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let post_btn = gtk::Button::with_label("Post");
        post_btn.add_css_class("suggested-action");

        title_bar.append(&back_btn);
        title_bar.append(&title_label);
        title_bar.append(&post_btn);

        let main_content = GtkBox::new(gtk::Orientation::Vertical, 16);
        main_content.set_margin_start(16);
        main_content.set_margin_end(16);
        main_content.set_margin_top(16);
        main_content.set_margin_bottom(16);

        let content_box = GtkBox::new(gtk::Orientation::Horizontal, 12);

        let avatar = gtk::Image::from_icon_name("avatar-default-symbolic");
        avatar.set_width_request(48);
        avatar.set_height_request(48);

        let input_area = GtkBox::new(gtk::Orientation::Vertical, 8);
        input_area.set_hexpand(true);

        let text_view = TextView::new();
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        text_view.set_editable(true);
        text_view.set_hexpand(true);
        text_view.set_vexpand(true);
        text_view.set_margin_top(8);
        text_view.set_margin_bottom(8);
        text_view.set_margin_start(8);
        text_view.set_margin_end(8);

        let placeholder_label = Label::new(Some("What's on your mind?"));
        placeholder_label.set_opacity(0.5);
        placeholder_label.set_margin_start(12);
        placeholder_label.set_margin_top(12);

        input_area.append(&text_view);

        let options_bar = GtkBox::new(gtk::Orientation::Horizontal, 8);
        options_bar.set_margin_top(8);

        let image_btn = gtk::Button::from_icon_name("image-x-generic-symbolic");
        let emoji_btn = gtk::Button::from_icon_name("emoji-object-symbolic");
        let poll_btn = gtk::Button::from_icon_name("poll-symbolic");
        let sticker_btn = gtk::Button::from_icon_name("Stickers-symbolic");

        options_bar.append(&image_btn);
        options_bar.append(&emoji_btn);
        options_bar.append(&poll_btn);
        options_bar.append(&sticker_btn);

        input_area.append(&options_bar);

        content_box.append(&avatar);
        content_box.append(&input_area);

        main_content.append(&content_box);

        widget.append(&title_bar);
        widget.append(&main_content);

        ComposePage {
            widget,
            posts_service,
        }
    }
}
