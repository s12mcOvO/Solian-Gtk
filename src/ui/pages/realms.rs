use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Label, ListBox, ListBoxRow, ScrolledWindow};

pub struct RealmsPage {
    pub widget: GtkBox,
}

impl RealmsPage {
    pub fn new(on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Realms"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let search_btn = gtk::Button::from_icon_name("system-search-symbolic");

        title_bar.append(&back_btn);
        title_bar.append(&title_label);
        title_bar.append(&search_btn);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let content_box = GtkBox::new(gtk::Orientation::Vertical, 16);
        content_box.set_margin_top(16);
        content_box.set_margin_bottom(16);
        content_box.set_margin_start(16);
        content_box.set_margin_end(16);
        content_box.set_halign(Align::Center);
        content_box.set_width_request(600);

        let my_realms_label = Label::new(Some("My Realms"));
        my_realms_label.add_css_class("title-3");
        my_realms_label.set_halign(Align::Start);

        let realms_list = ListBox::new();
        realms_list.add_css_class("realms-list");

        let my_realms = vec![
            (
                "Developers",
                "A community for developers",
                "24 members",
                true,
            ),
            ("Designers", "Design community", "18 members", true),
        ];

        for (name, desc, members, is_admin) in my_realms {
            let row = create_realm_row(name, desc, members, is_admin);
            realms_list.append(&row);
        }

        let discover_label = Label::new(Some("Discover"));
        discover_label.add_css_class("title-3");
        discover_label.set_halign(Align::Start);
        discover_label.set_margin_top(24);

        let discover_list = ListBox::new();
        discover_list.add_css_class("realms-list");

        let discover_realms = vec![
            (
                "Tech News",
                "Latest tech news and updates",
                "1.2K members",
                false,
            ),
            (
                "Open Source",
                "Open source projects and discussions",
                "856 members",
                false,
            ),
            ("Gaming", "Gaming community", "2.3K members", false),
            ("Music", "Music lovers community", "543 members", false),
        ];

        for (name, desc, members, is_admin) in discover_realms {
            let row = create_realm_row(name, desc, members, is_admin);
            discover_list.append(&row);
        }

        content_box.append(&my_realms_label);
        content_box.append(&realms_list);
        content_box.append(&discover_label);
        content_box.append(&discover_list);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        RealmsPage { widget }
    }
}

fn create_realm_row(name: &str, desc: &str, members: &str, is_admin: bool) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("realm-row");

    let container = GtkBox::new(gtk::Orientation::Horizontal, 12);
    container.set_margin_start(12);
    container.set_margin_end(12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);

    let avatar = gtk::Image::from_icon_name("folder-symbolic");
    avatar.set_width_request(48);
    avatar.set_height_request(48);

    let info = GtkBox::new(gtk::Orientation::Vertical, 4);
    info.set_hexpand(true);

    let name_box = GtkBox::new(gtk::Orientation::Horizontal, 8);

    let name_label = Label::new(Some(name));
    name_label.add_css_class("realm-name");
    name_label.set_halign(Align::Start);

    if is_admin {
        let admin_badge = gtk::Label::new(Some("Admin"));
        admin_badge.add_css_class("admin-badge");
        name_box.append(&name_label);
        name_box.append(&admin_badge);
    } else {
        name_box.append(&name_label);
    }

    name_box.set_hexpand(true);

    let desc_label = Label::new(Some(desc));
    desc_label.add_css_class("realm-desc");
    desc_label.set_opacity(0.7);
    desc_label.set_halign(Align::Start);
    desc_label.set_ellipsize(pango::EllipsizeMode::End);

    let members_label = Label::new(Some(members));
    members_label.add_css_class("realm-members");
    members_label.set_opacity(0.5);
    members_label.set_halign(Align::Start);

    info.append(&name_box);
    info.append(&desc_label);
    info.append(&members_label);

    let join_btn = gtk::Button::with_label(if is_admin { "Settings" } else { "Join" });
    if !is_admin {
        join_btn.add_css_class("suggested-action");
    }

    container.append(&avatar);
    container.append(&info);
    container.append(&join_btn);

    row.set_child(Some(&container));
    row
}
