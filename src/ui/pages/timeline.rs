use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Entry, Image, Label, ScrolledWindow};
use std::sync::Arc;

use crate::core::models::SnPost;
use crate::core::services::AuthService;
use crate::core::services::PostsService;

pub struct TimelinePage {
    pub widget: GtkBox,
    posts_service: Arc<PostsService>,
    auth_service: Arc<AuthService>,
    current_filter: std::cell::Cell<String>,
}

impl TimelinePage {
    pub fn new(
        posts_service: Arc<PostsService>,
        auth_service: Arc<AuthService>,
        on_home: impl Fn() + 'static,
        on_notifications: impl Fn() + 'static,
        on_messages: impl Fn() + 'static,
        on_profile: impl Fn() + 'static,
        on_settings: impl Fn() + 'static,
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Horizontal, 0);

        let sidebar = GtkBox::new(gtk::Orientation::Vertical, 0);
        sidebar.set_width_request(240);
        sidebar.add_css_class("sidebar");
        sidebar.set_margin_top(16);
        sidebar.set_margin_bottom(16);
        sidebar.set_margin_start(16);

        let logo_label = Label::new(Some("Solian"));
        logo_label.add_css_class("title-1");
        logo_label.set_halign(Align::Start);
        logo_label.set_margin_bottom(24);

        let nav_box = GtkBox::new(gtk::Orientation::Vertical, 4);
        nav_box.set_hexpand(true);

        let home_btn = create_nav_button("Home", "home-symbolic", true);
        let notif_btn = create_nav_button("Notifications", "notification-symbolic", false);
        let messages_btn = create_nav_button("Messages", "chat-symbolic", false);
        let profile_btn = create_nav_button("Profile", "user-symbolic", false);
        let settings_btn = create_nav_button("Settings", "preferences-system-symbolic", false);

        home_btn.connect_clicked(move |_| on_home());
        notif_btn.connect_clicked(move |_| on_notifications());
        messages_btn.connect_clicked(move |_| on_messages());
        profile_btn.connect_clicked(move |_| on_profile());
        settings_btn.connect_clicked(move |_| on_settings());

        nav_box.append(&home_btn);
        nav_box.append(&notif_btn);
        nav_box.append(&messages_btn);
        nav_box.append(&profile_btn);
        nav_box.append(&settings_btn);

        let compose_btn = Button::with_label("Post");
        compose_btn.add_css_class("suggested-action");
        compose_btn.set_hexpand(true);
        compose_btn.set_size_request(-1, 44);
        compose_btn.set_margin_top(16);

        let user_box = GtkBox::new(gtk::Orientation::Horizontal, 8);
        user_box.set_margin_top(16);

        let avatar = Image::from_icon_name("avatar-default-symbolic");
        avatar.set_width_request(40);
        avatar.set_height_request(40);

        let user_info = GtkBox::new(gtk::Orientation::Vertical, 0);
        let user_name = Label::new(Some("Loading..."));
        user_name.add_css_class("subtitle");
        let user_handle = Label::new(Some("@user"));
        user_handle.add_css_class("subtitle");
        user_handle.set_opacity(0.6);
        user_info.append(&user_name);
        user_info.append(&user_handle);

        user_box.append(&avatar);
        user_box.append(&user_info);

        sidebar.append(&logo_label);
        sidebar.append(&nav_box);
        sidebar.append(&compose_btn);

        let main_area = GtkBox::new(gtk::Orientation::Vertical, 0);

        let header = GtkBox::new(gtk::Orientation::Horizontal, 0);
        header.add_css_class("titlebar");
        header.set_height_request(56);

        let tab_box = GtkBox::new(gtk::Orientation::Horizontal, 0);
        tab_box.set_hexpand(true);
        tab_box.set_halign(Align::Center);

        let home_tab = create_tab("Home", true);
        let local_tab = create_tab("Local", false);
        let federated_tab = create_tab("Federated", false);

        tab_box.append(&home_tab);
        tab_box.append(&local_tab);
        tab_box.append(&federated_tab);

        header.append(&tab_box);

        let feed_scroll = ScrolledWindow::new();
        feed_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        feed_scroll.set_vexpand(true);

        let feed_box = GtkBox::new(gtk::Orientation::Vertical, 0);
        feed_box.set_width_request(600);

        let compose_box = GtkBox::new(gtk::Orientation::Horizontal, 12);
        compose_box.set_margin_start(16);
        compose_box.set_margin_end(16);
        compose_box.set_margin_top(12);
        compose_box.set_margin_bottom(12);

        let compose_avatar = Image::from_icon_name("avatar-default-symbolic");
        compose_avatar.set_width_request(40);
        compose_avatar.set_height_request(40);

        let compose_input = Entry::new();
        compose_input.set_placeholder_text(Some("What's happening?"));
        compose_input.set_hexpand(true);
        compose_input.set_margin_start(8);

        compose_box.append(&compose_avatar);
        compose_box.append(&compose_input);

        feed_box.append(&compose_box);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        feed_box.append(&separator);

        let posts_box = GtkBox::new(gtk::Orientation::Vertical, 0);
        posts_box.set_margin_start(16);
        posts_box.set_margin_end(16);
        posts_box.set_margin_top(8);
        posts_box.set_margin_bottom(8);

        let placeholder_post = create_post_card(
            "Welcome to Solian",
            "Your Solar Network Desktop Client is now running. Start exploring the Fediverse!",
            "Just now",
            "0",
            "0",
        );
        posts_box.append(&placeholder_post);

        feed_box.append(&posts_box);

        feed_scroll.set_child(Some(&feed_box));

        main_area.append(&header);
        main_area.append(&feed_scroll);

        widget.append(&sidebar);
        widget.append(&main_area);

        TimelinePage {
            widget,
            posts_service,
            auth_service,
            current_filter: std::cell::Cell::new("home".to_string()),
        }
    }
}

fn create_nav_button(label: &str, icon: &str, active: bool) -> gtk::Button {
    let btn = gtk::Button::new();
    btn.set_halign(Align::Start);
    btn.set_hexpand(true);
    btn.set_size_request(200, 48);

    let icon_widget = gtk::Image::from_icon_name(icon);
    icon_widget.set_icon_size(gtk::IconSize::Large);

    let button_box = GtkBox::new(gtk::Orientation::Horizontal, 16);
    button_box.append(&icon_widget);

    let label_widget = Label::new(Some(label));
    label_widget.add_css_class("title-3");
    button_box.append(&label_widget);

    btn.set_child(Some(&button_box));

    if active {
        btn.add_css_class("active-nav");
    }

    btn
}

fn create_tab(label: &str, active: bool) -> gtk::Button {
    let btn = gtk::Button::new();
    btn.set_halign(Align::Center);
    btn.set_hexpand(true);

    let label_widget = Label::new(Some(label));
    label_widget.add_css_class("title-3");
    btn.set_child(Some(&label_widget));

    if active {
        btn.add_css_class("active-tab");
    }

    btn
}

fn create_post_card(author: &str, content: &str, time: &str, likes: &str, replies: &str) -> GtkBox {
    let card = GtkBox::new(gtk::Orientation::Vertical, 8);
    card.set_margin_top(8);
    card.set_margin_bottom(8);

    let header = GtkBox::new(gtk::Orientation::Horizontal, 8);

    let avatar = Image::from_icon_name("avatar-default-symbolic");
    avatar.set_width_request(40);
    avatar.set_height_request(40);

    let author_info = GtkBox::new(gtk::Orientation::Vertical, 0);

    let author_label = Label::new(Some(author));
    author_label.add_css_class("title-4");
    author_label.set_halign(Align::Start);

    let time_label = Label::new(Some(time));
    time_label.add_css_class("subtitle");
    time_label.set_opacity(0.6);
    time_label.set_halign(Align::Start);

    author_info.append(&author_label);
    author_info.append(&time_label);

    header.append(&avatar);
    header.append(&author_info);

    let content_label = Label::new(Some(content));
    content_label.set_wrap(true);
    content_label.set_xalign(0.0);
    content_label.add_css_class("body-text");

    let actions = GtkBox::new(gtk::Orientation::Horizontal, 32);
    actions.set_margin_top(8);

    let reply_btn = gtk::Button::from_icon_name("comment-symbolic");
    let reply_label = Label::new(Some(replies));
    reply_label.set_opacity(0.6);

    let repost_btn = gtk::Button::from_icon_name("repeat-symbolic");
    let repost_label = Label::new(Some("0"));
    repost_label.set_opacity(0.6);

    let like_btn = gtk::Button::from_icon_name("heart-symbolic");
    let like_label = Label::new(Some(likes));
    like_label.set_opacity(0.6);

    let share_btn = gtk::Button::from_icon_name("share-symbolic");

    actions.append(&reply_btn);
    actions.append(&reply_label);
    actions.append(&repost_btn);
    actions.append(&repost_label);
    actions.append(&like_btn);
    actions.append(&like_label);
    actions.append(&share_btn);

    card.append(&header);
    card.append(&content_label);
    card.append(&actions);

    card
}

fn create_post_card_from_snpost(post: &SnPost) -> GtkBox {
    let author_name = post
        .author
        .as_ref()
        .and_then(|a| a.display_name.clone().or_else(|| Some(a.name.clone())))
        .unwrap_or_else(|| "Unknown".to_string());

    let content = post.content.clone().unwrap_or_default();
    let time = post
        .created_at
        .clone()
        .unwrap_or_else(|| "Unknown".to_string());
    let likes = post.favourites_count.unwrap_or(0).to_string();
    let replies = post.replies_count.unwrap_or(0).to_string();

    create_post_card(&author_name, &content, &time, &likes, &replies)
}
