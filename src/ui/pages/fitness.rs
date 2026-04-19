use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Label, ListBox, ListBoxRow, ScrolledWindow};
use std::sync::Arc;

use crate::core::services::FitnessService;

pub struct FitnessPage {
    pub widget: GtkBox,
    fitness_service: Arc<FitnessService>,
}

impl FitnessPage {
    pub fn new(fitness_service: Arc<FitnessService>, on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Fitness & Health"));
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

        let stats_card = GtkBox::new(gtk::Orientation::Vertical, 12);
        stats_card.add_css_class("card");
        stats_card.set_margin_bottom(16);

        let stats_title = Label::new(Some("Today's Stats"));
        stats_title.add_css_class("title-3");
        stats_title.set_halign(Align::Start);

        let stats_box = GtkBox::new(gtk::Orientation::Horizontal, 24);
        stats_box.set_margin_top(8);

        let steps_stat = create_stat_item("0", "Steps");
        let calories_stat = create_stat_item("0", "Calories");
        let distance_stat = create_stat_item("0.0 km", "Distance");
        let time_stat = create_stat_item("0 min", "Active");

        stats_box.append(&steps_stat.0);
        stats_box.append(&calories_stat.0);
        stats_box.append(&distance_stat.0);
        stats_box.append(&time_stat.0);

        stats_card.append(&stats_title);
        stats_card.append(&stats_box);

        let activities_label = Label::new(Some("Recent Activities"));
        activities_label.add_css_class("title-3");
        activities_label.set_halign(Align::Start);
        activities_label.set_margin_top(16);

        let activities_list = ListBox::new();

        let placeholder = Label::new(Some("No activities yet. Start tracking!"));
        placeholder.set_opacity(0.6);
        placeholder.set_halign(Align::Center);
        placeholder.set_margin_top(24);
        activities_list.append(&placeholder);

        content_box.append(&stats_card);
        content_box.append(&activities_label);
        content_box.append(&activities_list);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        FitnessPage {
            widget,
            fitness_service,
        }
    }
}

fn create_stat_item(value: &str, label_text: &str) -> (gtk::Box, Label) {
    let stat_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
    stat_box.set_halign(Align::Center);

    let value_label = Label::new(Some(value));
    value_label.add_css_class("title-2");
    value_label.set_halign(Align::Center);

    let label = Label::new(Some(label_text));
    label.add_css_class("subtitle");
    label.set_opacity(0.7);
    label.set_halign(Align::Center);

    stat_box.append(&value_label);
    stat_box.append(&label);

    (stat_box, value_label)
}
