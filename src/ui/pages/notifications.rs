use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Image, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::models::SnNotification;
use crate::core::services::NotificationService;

pub struct NotificationsPage {
    pub widget: GtkBox,
    notification_service: Arc<NotificationService>,
}

impl NotificationsPage {
    pub fn new(
        notification_service: Arc<NotificationService>,
        on_back: impl Fn() + 'static,
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Notifications"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let mark_read_btn = gtk::Button::from_icon_name("checkmark-all-symbolic");

        title_bar.append(&back_btn);
        title_bar.append(&title_label);
        title_bar.append(&mark_read_btn);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let notifications_list = ListBox::new();
        notifications_list.add_css_class("notifications-list");

        let placeholder_label = Label::new(Some("No notifications yet"));
        placeholder_label.set_opacity(0.6);
        placeholder_label.set_halign(Align::Center);
        placeholder_label.set_margin_top(48);
        notifications_list.append(&placeholder_label);

        main_content.set_child(Some(&notifications_list));

        widget.append(&title_bar);
        widget.append(&main_content);

        NotificationsPage {
            widget,
            notification_service,
        }
    }
}

fn create_notification_row(notification: &SnNotification) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("notification-row");

    let container = GtkBox::new(gtk::Orientation::Horizontal, 12);
    container.set_margin_start(12);
    container.set_margin_end(12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);

    let icon = match notification.notification_type.as_str() {
        "follow" => "user-new-symbolic",
        "like" => "heart-symbolic",
        "reblog" => "repeat-symbolic",
        "reply" => "reply-symbolic",
        "mention" => "at-symbolic",
        _ => "notification-symbolic",
    };

    let avatar = Image::from_icon_name(icon);
    avatar.set_width_request(32);
    avatar.set_height_request(32);

    let info_box = GtkBox::new(gtk::Orientation::Vertical, 4);
    info_box.set_hexpand(true);

    let actor_name = notification
        .account
        .as_ref()
        .and_then(|a| a.display_name.clone())
        .unwrap_or_else(|| "Someone".to_string());
    let content = format!(
        "{} {}",
        actor_name,
        match notification.notification_type.as_str() {
            "follow" => "followed you",
            "like" => "liked your post",
            "reblog" => "boosted your post",
            "reply" => "replied to you",
            "mention" => "mentioned you",
            _ => "interacted with you",
        }
    );
    let content_label = Label::new(Some(&content));
    content_label.set_wrap(true);
    content_label.set_xalign(0.0);
    content_label.set_ellipsize(pango::EllipsizeMode::End);

    let time_label = Label::new(Some(&notification.created_at.clone().unwrap_or_default()));
    time_label.add_css_class("notification-time");
    time_label.set_opacity(0.6);

    info_box.append(&content_label);
    info_box.append(&time_label);

    container.append(&avatar);
    container.append(&info_box);

    row.set_child(Some(&container));
    row
}
