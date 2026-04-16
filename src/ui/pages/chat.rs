use adw::prelude::*;
use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Entry, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::models::SnChatRoom;
use crate::core::services::ChatService;

pub struct ChatPage {
    pub widget: GtkBox,
    #[allow(dead_code)]
    chat_service: Arc<ChatService>,
}

impl ChatPage {
    pub fn new(
        chat_service: Arc<ChatService>,
        on_back: impl Fn() + 'static,
        _on_room_selected: fn(String),
    ) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let main_split = GtkBox::new(gtk::Orientation::Horizontal, 0);

        let sidebar = GtkBox::new(gtk::Orientation::Vertical, 0);
        sidebar.set_width_request(280);
        sidebar.add_css_class("sidebar");

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let title_label = Label::new(Some("Messages"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let new_chat_btn = gtk::Button::from_icon_name("list-add-symbolic");

        title_bar.append(&title_label);
        title_bar.append(&new_chat_btn);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Search conversations..."));
        search_entry.set_icon_from_icon_name(
            gtk::EntryIconPosition::Primary,
            Some("system-search-symbolic"),
        );

        let rooms_list = ListBox::new();
        rooms_list.add_css_class("chat-rooms-list");

        let rooms_scroll = ScrolledWindow::new();
        rooms_scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        rooms_scroll.set_vexpand(true);
        rooms_scroll.set_child(Some(&rooms_list));

        let placeholder_rooms = vec![("Loading...", "Fetching chat rooms...", "")];

        for (name, preview, _time) in placeholder_rooms {
            let row = create_placeholder_row(name, preview);
            rooms_list.append(&row);
        }

        sidebar.append(&title_bar);
        sidebar.append(&search_entry);
        sidebar.append(&rooms_scroll);

        let main_content = GtkBox::new(gtk::Orientation::Vertical, 0);
        main_content.set_hexpand(true);

        let chat_title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        chat_title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let chat_title = Label::new(Some("Select a conversation"));
        chat_title.add_css_class("title");
        chat_title.set_halign(Align::Center);
        chat_title.set_hexpand(true);

        let more_btn = gtk::Button::from_icon_name("view-more-symbolic");

        chat_title_bar.append(&back_btn);
        chat_title_bar.append(&chat_title);
        chat_title_bar.append(&more_btn);

        let messages_scroll = ScrolledWindow::new();
        messages_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        messages_scroll.set_vexpand(true);

        let messages_box = GtkBox::new(gtk::Orientation::Vertical, 0);
        messages_box.set_valign(Align::End);
        messages_box.set_margin_start(16);
        messages_box.set_margin_end(16);
        messages_box.set_margin_top(16);
        messages_box.set_margin_bottom(16);
        messages_box.set_spacing(8);

        let placeholder_label = Label::new(Some("Select a conversation to start chatting"));
        placeholder_label.set_opacity(0.5);
        messages_box.append(&placeholder_label);

        messages_scroll.set_child(Some(&messages_box));

        let input_box = GtkBox::new(gtk::Orientation::Horizontal, 8);
        input_box.add_css_class("chat-input-box");

        let message_entry = Entry::new();
        message_entry.set_placeholder_text(Some("Type a message..."));
        message_entry.set_hexpand(true);

        let send_btn = gtk::Button::from_icon_name("paper-plane-symbolic");
        send_btn.add_css_class("suggested-action");
        send_btn.set_sensitive(false);

        let messages_box_clone = messages_box.clone();
        let message_entry_clone = message_entry.clone();
        let send_btn_for_click = send_btn.clone();
        let chat_service_for_send = chat_service.clone();
        let current_room_id: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);

        send_btn.connect_clicked(move |_| {
            let text = message_entry_clone.text().to_string();
            if !text.is_empty() {
                let bubble = create_message_bubble(&text, true, "You");
                messages_box_clone.append(&bubble);
                message_entry_clone.set_text("");
            }
        });

        let send_btn_for_change = send_btn.clone();
        message_entry.connect_activate(move |_| {
            send_btn_for_click.emit_clicked();
        });

        message_entry.connect_changed(move |entry| {
            send_btn_for_change.set_sensitive(!entry.text().is_empty());
        });

        input_box.append(&message_entry);
        input_box.append(&send_btn);

        let chat_area = GtkBox::new(gtk::Orientation::Vertical, 0);
        chat_area.append(&chat_title_bar);
        chat_area.append(&messages_scroll);
        chat_area.append(&input_box);

        main_split.append(&sidebar);
        main_split.append(&chat_area);
        main_split.set_hexpand(true);

        widget.append(&main_split);

        ChatPage {
            widget,
            chat_service,
        }
    }
}

fn create_placeholder_row(name: &str, preview: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("chat-room-row");

    let container = GtkBox::new(gtk::Orientation::Horizontal, 12);
    container.set_margin_start(12);
    container.set_margin_end(12);
    container.set_margin_top(8);
    container.set_margin_bottom(8);

    let avatar = gtk::Image::from_icon_name("avatar-default-symbolic");
    avatar.add_css_class("chat-avatar");
    avatar.set_width_request(40);
    avatar.set_height_request(40);

    let info_box = GtkBox::new(gtk::Orientation::Vertical, 2);
    info_box.set_hexpand(true);

    let name_label = Label::new(Some(name));
    name_label.add_css_class("chat-room-name");
    name_label.set_halign(Align::Start);
    name_label.set_ellipsize(pango::EllipsizeMode::End);

    let preview_label = Label::new(Some(preview));
    preview_label.add_css_class("chat-room-preview");
    preview_label.set_opacity(0.6);
    preview_label.set_halign(Align::Start);
    preview_label.set_ellipsize(pango::EllipsizeMode::End);

    info_box.append(&name_label);
    info_box.append(&preview_label);

    container.append(&avatar);
    container.append(&info_box);

    row.set_child(Some(&container));
    row
}

fn create_room_row(_room_id: &str, room: &SnChatRoom) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("chat-room-row");

    let container = GtkBox::new(gtk::Orientation::Horizontal, 12);
    container.set_margin_start(12);
    container.set_margin_end(12);
    container.set_margin_top(8);
    container.set_margin_bottom(8);

    let icon_name = "avatar-default-symbolic";
    let avatar = gtk::Image::from_icon_name(icon_name);
    avatar.add_css_class("chat-avatar");
    avatar.set_width_request(40);
    avatar.set_height_request(40);

    let info_box = GtkBox::new(gtk::Orientation::Vertical, 2);
    info_box.set_hexpand(true);

    let name = room.name.clone().unwrap_or_else(|| "Unnamed".to_string());
    let name_label = Label::new(Some(&name));
    name_label.add_css_class("chat-room-name");
    name_label.set_halign(Align::Start);
    name_label.set_ellipsize(pango::EllipsizeMode::End);

    let preview = room
        .last_message
        .as_ref()
        .and_then(|m| m.content.clone())
        .unwrap_or_else(|| "No messages".to_string());
    let preview_label = Label::new(Some(&preview));
    preview_label.add_css_class("chat-room-preview");
    preview_label.set_opacity(0.6);
    preview_label.set_halign(Align::Start);
    preview_label.set_ellipsize(pango::EllipsizeMode::End);

    info_box.append(&name_label);
    info_box.append(&preview_label);

    let time_label = Label::new(Some(""));
    time_label.add_css_class("chat-room-time");
    time_label.set_opacity(0.5);

    let unread = room.unread_count.unwrap_or(0);
    let unread_badge = if unread > 0 {
        let badge = gtk::Label::new(Some(unread.to_string().as_str()));
        badge.add_css_class("unread-badge");
        Some(badge)
    } else {
        None
    };

    container.append(&avatar);
    container.append(&info_box);
    container.append(&time_label);
    if let Some(badge) = unread_badge {
        container.append(&badge);
    }

    row.set_child(Some(&container));
    row
}

fn create_message_bubble(content: &str, is_me: bool, sender: &str) -> GtkBox {
    let container = GtkBox::new(gtk::Orientation::Horizontal, 0);
    container.set_margin_top(4);
    container.set_margin_bottom(4);
    container.set_hexpand(true);

    let bubble = GtkBox::new(gtk::Orientation::Vertical, 4);
    bubble.set_margin_start(8);
    bubble.set_margin_end(8);
    bubble.set_margin_top(4);
    bubble.set_margin_bottom(4);
    bubble.set_hexpand(true);

    if is_me {
        container.set_halign(Align::End);
    } else {
        container.set_halign(Align::Start);
    }

    if !is_me {
        let sender_label = Label::new(Some(sender));
        sender_label.add_css_class("message-sender");
        sender_label.set_opacity(0.7);
        sender_label.set_halign(Align::Start);
        bubble.append(&sender_label);
    }

    let content_label = Label::new(Some(content));
    content_label.set_wrap(true);
    content_label.set_xalign(if is_me { 1.0 } else { 0.0 });
    content_label.set_vexpand(true);

    let time_label = Label::new(Some("10:30"));
    time_label.add_css_class("message-time");
    time_label.set_opacity(0.5);
    time_label.set_halign(if is_me { Align::End } else { Align::Start });

    bubble.append(&content_label);
    bubble.append(&time_label);

    container.append(&bubble);
    container
}
