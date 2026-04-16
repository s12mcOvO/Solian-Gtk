use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Button, Label, ScrolledWindow};

pub struct WalletsPage {
    pub widget: GtkBox,
}

impl WalletsPage {
    pub fn new(on_back: impl Fn() + 'static) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 0);

        let title_bar = GtkBox::new(gtk::Orientation::Horizontal, 0);
        title_bar.add_css_class("titlebar");

        let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
        back_btn.connect_clicked(move |_| {
            on_back();
        });

        let title_label = Label::new(Some("Wallets"));
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
        content_box.set_margin_top(16);
        content_box.set_margin_bottom(16);
        content_box.set_margin_start(16);
        content_box.set_margin_end(16);
        content_box.set_halign(Align::Center);
        content_box.set_width_request(600);

        let balance_card = create_balance_card("1,234.56", "≈ $1,234.56 USD");
        content_box.append(&balance_card);

        let actions = GtkBox::new(gtk::Orientation::Horizontal, 12);
        actions.set_hexpand(true);

        let send_btn = gtk::Button::with_label("Send");
        send_btn.add_css_class("suggested-action");
        send_btn.set_hexpand(true);

        let receive_btn = gtk::Button::with_label("Receive");
        receive_btn.set_hexpand(true);

        let swap_btn = gtk::Button::with_label("Swap");
        swap_btn.set_hexpand(true);

        actions.append(&send_btn);
        actions.append(&receive_btn);
        actions.append(&swap_btn);
        content_box.append(&actions);

        let wallets_label = Label::new(Some("Your Wallets"));
        wallets_label.add_css_class("title-3");
        wallets_label.set_halign(Align::Start);
        wallets_label.set_margin_top(16);

        let wallets = vec![
            ("Main Wallet", "SN", "1,000.00", "+5.2%"),
            ("Savings", "SN", "234.56", "+3.1%"),
            ("Business", "SN", "0.00", "0%"),
        ];

        for (name, symbol, balance, change) in wallets {
            let wallet_card = create_wallet_card(name, symbol, balance, change);
            content_box.append(&wallet_card);
        }

        main_content.set_child(Some(&content_box));

        widget.append(&title_bar);
        widget.append(&main_content);

        WalletsPage { widget }
    }
}

fn create_balance_card(balance: &str, usd_value: &str) -> GtkBox {
    let card = GtkBox::new(gtk::Orientation::Vertical, 12);
    card.add_css_class("balance-card");
    card.set_margin_start(24);
    card.set_margin_end(24);
    card.set_margin_top(24);
    card.set_margin_bottom(24);

    let label = Label::new(Some("Total Balance"));
    label.add_css_class("balance-label");
    label.set_opacity(0.7);
    label.set_halign(Align::Center);

    let balance_label = Label::new(Some(balance));
    balance_label.add_css_class("balance-amount");
    balance_label.set_halign(Align::Center);

    let usd_label = Label::new(Some(usd_value));
    usd_label.add_css_class("balance-usd");
    usd_label.set_opacity(0.7);
    usd_label.set_halign(Align::Center);

    card.append(&label);
    card.append(&balance_label);
    card.append(&usd_label);

    card
}

fn create_wallet_card(name: &str, symbol: &str, balance: &str, change: &str) -> GtkBox {
    let card = GtkBox::new(gtk::Orientation::Horizontal, 12);
    card.add_css_class("wallet-card");
    card.set_margin_start(16);
    card.set_margin_end(16);
    card.set_margin_top(12);
    card.set_margin_bottom(12);

    let icon = gtk::Image::from_icon_name("wallet-symbolic");
    icon.set_width_request(40);
    icon.set_height_request(40);

    let info = GtkBox::new(gtk::Orientation::Vertical, 4);
    info.set_hexpand(true);

    let name_label = Label::new(Some(name));
    name_label.add_css_class("wallet-name");
    name_label.set_halign(Align::Start);

    let symbol_label = Label::new(Some(symbol));
    symbol_label.add_css_class("wallet-symbol");
    symbol_label.set_opacity(0.6);
    symbol_label.set_halign(Align::Start);

    info.append(&name_label);
    info.append(&symbol_label);

    let balance_box = GtkBox::new(gtk::Orientation::Vertical, 2);
    balance_box.set_halign(Align::End);

    let balance_label = Label::new(Some(balance));
    balance_label.add_css_class("wallet-balance");
    balance_label.set_halign(Align::End);

    let change_label = Label::new(Some(change));
    change_label.add_css_class(if change.starts_with('+') {
        "wallet-change-positive"
    } else {
        "wallet-change"
    });
    change_label.set_halign(Align::End);

    balance_box.append(&balance_label);
    balance_box.append(&change_label);

    card.append(&icon);
    card.append(&info);
    card.append(&balance_box);

    card
}
