use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Label, ScrolledWindow};
use std::sync::Arc;

use crate::core::models::SnPost;
use crate::core::services::PostsService;

pub struct PostsPage {
    pub widget: GtkBox,
    #[allow(dead_code)]
    posts_service: Arc<PostsService>,
}

impl PostsPage {
    pub fn new(posts_service: Arc<PostsService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Posts"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let new_post_btn = gtk::Button::from_icon_name("plus-symbolic");
        new_post_btn.add_css_class("suggested-action");

        title_bar.append(&back_btn);
        title_bar.append(&title_label);
        title_bar.append(&new_post_btn);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let posts_box = GtkBox::new(gtk::Orientation::Vertical, 16);
        posts_box.set_margin_top(16);
        posts_box.set_margin_bottom(16);
        posts_box.set_margin_start(16);
        posts_box.set_margin_end(16);
        posts_box.set_halign(Align::Center);
        posts_box.set_width_request(600);

        let placeholder_posts = vec![("Loading...", "Fetching timeline from API...", "")];

        for (author, content, time) in placeholder_posts {
            let post_card = create_placeholder_card(author, content, time);
            posts_box.append(&post_card);
        }

        main_content.set_child(Some(&posts_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        PostsPage {
            widget,
            posts_service,
        }
    }
}

fn create_placeholder_card(author: &str, content: &str, time: &str) -> GtkBox {
    let card = GtkBox::new(gtk::Orientation::Vertical, 12);
    card.add_css_class("post-card");
    card.set_margin_start(16);
    card.set_margin_end(16);
    card.set_margin_top(16);
    card.set_margin_bottom(16);

    let header = GtkBox::new(gtk::Orientation::Horizontal, 12);

    let avatar = gtk::Image::from_icon_name("avatar-default-symbolic");
    avatar.set_width_request(40);
    avatar.set_height_request(40);

    let author_info = GtkBox::new(gtk::Orientation::Vertical, 2);

    let author_label = Label::new(Some(author));
    author_label.add_css_class("post-author");
    author_label.set_halign(Align::Start);

    let time_label = Label::new(Some(time));
    time_label.add_css_class("post-time");
    time_label.set_opacity(0.6);
    time_label.set_halign(Align::Start);

    author_info.append(&author_label);
    author_info.append(&time_label);

    header.append(&avatar);
    header.append(&author_info);
    header.set_hexpand(true);

    let content_label = Label::new(Some(content));
    content_label.set_wrap(true);
    content_label.set_xalign(0.0);
    content_label.set_vexpand(true);

    card.append(&header);
    card.append(&content_label);

    card
}

fn create_post_card(post: &SnPost) -> GtkBox {
    let card = GtkBox::new(gtk::Orientation::Vertical, 12);
    card.add_css_class("post-card");
    card.set_margin_start(16);
    card.set_margin_end(16);
    card.set_margin_top(16);
    card.set_margin_bottom(16);

    let header = GtkBox::new(gtk::Orientation::Horizontal, 12);

    let avatar = gtk::Image::from_icon_name("avatar-default-symbolic");
    avatar.set_width_request(40);
    avatar.set_height_request(40);

    let author_info = GtkBox::new(gtk::Orientation::Vertical, 2);

    let author_name = post
        .author
        .as_ref()
        .and_then(|a| a.display_name.clone().or_else(|| Some(a.name.clone())))
        .unwrap_or_else(|| "Unknown".to_string());
    let author_label = Label::new(Some(&author_name));
    author_label.add_css_class("post-author");
    author_label.set_halign(Align::Start);

    let time_label = Label::new(Some(&post.created_at.clone().unwrap_or_default()));
    time_label.add_css_class("post-time");
    time_label.set_opacity(0.6);
    time_label.set_halign(Align::Start);

    author_info.append(&author_label);
    author_info.append(&time_label);

    header.append(&avatar);
    header.append(&author_info);
    header.set_hexpand(true);

    let content = post.content.clone().unwrap_or_default();
    let content_label = Label::new(Some(&content));
    content_label.set_wrap(true);
    content_label.set_xalign(0.0);
    content_label.set_vexpand(true);

    let actions = GtkBox::new(gtk::Orientation::Horizontal, 24);
    actions.set_margin_top(8);

    let likes = post.favourites_count.unwrap_or(0);
    let comments = post.replies_count.unwrap_or(0);
    let like_btn = create_action_button("heart-symbolic", &likes.to_string());
    let comment_btn = create_action_button("comment-symbolic", &comments.to_string());
    let share_btn = create_action_button("share-symbolic", "Share");

    actions.append(&like_btn);
    actions.append(&comment_btn);
    actions.append(&share_btn);

    card.append(&header);
    card.append(&content_label);
    card.append(&actions);

    card
}

fn create_action_button(icon: &str, label_text: &str) -> GtkBox {
    let button_box = GtkBox::new(gtk::Orientation::Horizontal, 4);

    let icon_widget = gtk::Image::from_icon_name(icon);

    let label = Label::new(Some(label_text));
    label.set_opacity(0.7);

    button_box.append(&icon_widget);
    button_box.append(&label);
    button_box
}
