use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Label, ScrolledWindow};

pub struct ThoughtsPage {
    pub widget: GtkBox,
}

impl ThoughtsPage {
    pub fn new(on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Thoughts"));
        title_label.add_css_class("title");
        title_label.set_halign(Align::Center);
        title_label.set_hexpand(true);

        let ai_btn = gtk::Button::from_icon_name("lightbulb-symbolic");

        title_bar.append(&back_btn);
        title_bar.append(&title_label);
        title_bar.append(&ai_btn);

        let main_content = ScrolledWindow::new();
        main_content.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        main_content.set_vexpand(true);

        let content_box = GtkBox::new(gtk::Orientation::Vertical, 24);
        content_box.set_margin_top(16);
        content_box.set_margin_bottom(16);
        content_box.set_margin_start(16);
        content_box.set_margin_end(16);
        content_box.set_halign(Align::Center);
        content_box.set_width_request(600);

        let header = Label::new(Some("AI Assistant"));
        header.add_css_class("title-2");
        header.set_halign(Align::Center);

        let desc = Label::new(Some("Ask questions and get AI-powered responses"));
        desc.add_css_class("subtitle");
        desc.set_opacity(0.7);
        desc.set_halign(Align::Center);

        let examples_label = Label::new(Some("Example Questions"));
        examples_label.add_css_class("title-3");
        examples_label.set_halign(Align::Start);
        examples_label.set_margin_top(16);

        let examples = vec![
            "Explain quantum computing in simple terms",
            "What are the best practices for Rust programming?",
            "How does blockchain technology work?",
            "Write a function to sort a list in Python",
        ];

        for example in examples {
            let btn = gtk::Button::new();
            btn.set_halign(Align::Start);
            btn.set_hexpand(true);

            let btn_box = GtkBox::new(gtk::Orientation::Horizontal, 8);
            btn_box.append(&gtk::Image::from_icon_name("lightbulb-symbolic"));
            btn_box.append(&Label::new(Some(example)));

            btn.set_child(Some(&btn_box));
            content_box.append(&btn);
        }

        let tips_label = Label::new(Some("Tips"));
        tips_label.add_css_class("title-3");
        tips_label.set_halign(Align::Start);
        tips_label.set_margin_top(24);

        let tips = Label::new(Some("• Be specific in your questions\n• Use context from your conversations\n• Try different phrasings if needed"));
        tips.set_opacity(0.7);
        tips.set_halign(Align::Start);
        tips.set_margin_top(8);

        content_box.append(&header);
        content_box.append(&desc);
        content_box.append(&examples_label);
        content_box.append(&tips_label);
        content_box.append(&tips);

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        ThoughtsPage { widget }
    }
}
