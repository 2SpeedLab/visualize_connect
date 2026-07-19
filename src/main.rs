use gtk::cairo;
use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, DrawingArea, DropDown,
    Frame, HeaderBar, Label, ListBox, ListBoxRow, Orientation, Paned, PolicyType, ScrolledWindow,
    SearchEntry, Separator, StyleContext, TextBuffer, TextView,
};
use gtk4 as gtk;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "com.example.NetworkVisualizer";

#[derive(Clone, Copy)]
struct Packet {
    time: &'static str,
    protocol: &'static str,
    source: &'static str,
    destination: &'static str,
    length: u16,
    summary: &'static str,
}

const PACKETS: [Packet; 6] = [
    Packet { time: "10:42:01.104", protocol: "UDP", source: "192.168.1.24:5353", destination: "224.0.0.251:5353", length: 186, summary: "mDNS standard query" },
    Packet { time: "10:42:01.227", protocol: "TCP", source: "192.168.1.24:51432", destination: "142.250.196.14:443", length: 66, summary: "ACK" },
    Packet { time: "10:42:01.390", protocol: "TCP", source: "142.250.196.14:443", destination: "192.168.1.24:51432", length: 1514, summary: "TLS application data" },
    Packet { time: "10:42:01.642", protocol: "ARP", source: "192.168.1.1", destination: "192.168.1.24", length: 42, summary: "Who has 192.168.1.24?" },
    Packet { time: "10:42:01.811", protocol: "ICMP", source: "192.168.1.24", destination: "1.1.1.1", length: 98, summary: "Echo request" },
    Packet { time: "10:42:02.001", protocol: "UDP", source: "192.168.1.24:61245", destination: "8.8.8.8:53", length: 82, summary: "DNS query: api.example.com" },
];

fn set_margins(widget: &impl IsA<gtk::Widget>, amount: i32) {
    widget.set_margin_top(amount);
    widget.set_margin_bottom(amount);
    widget.set_margin_start(amount);
    widget.set_margin_end(amount);
}

fn label(text: &str, class: Option<&str>) -> Label {
    let label = Label::new(Some(text));
    label.set_xalign(0.0);
    if let Some(class) = class {
        label.add_css_class(class);
    }
    label
}

fn stat_card(name: &str, value: &str) -> (Frame, Label) {
    let card = Frame::new(None);
    card.add_css_class("stat-card");
    card.set_hexpand(true);
    let box_ = GtkBox::new(Orientation::Vertical, 4);
    set_margins(&box_, 12);
    box_.append(&label(name, Some("muted")));
    let value_label = label(value, Some("metric-value"));
    box_.append(&value_label);
    card.set_child(Some(&box_));
    (card, value_label)
}

fn protocol_card(protocol: &str, count: &str, color: &str) -> Frame {
    let card = Frame::new(None);
    card.add_css_class("protocol-card");
    let row = GtkBox::new(Orientation::Horizontal, 8);
    set_margins(&row, 10);
    let dot = Label::new(Some("●"));
    dot.set_markup(&format!("<span foreground=\"{color}\">●</span>"));
    row.append(&dot);
    let name = label(protocol, None);
    name.set_hexpand(true);
    row.append(&name);
    row.append(&label(count, Some("muted")));
    card.set_child(Some(&row));
    card
}

fn packet_row(packet: Packet) -> ListBoxRow {
    let row = ListBoxRow::new();
    let content = GtkBox::new(Orientation::Horizontal, 12);
    set_margins(&content, 6);
    let fields = [
        (packet.time, 125),
        (packet.protocol, 70),
        (packet.source, 190),
        (packet.destination, 205),
        (&format!("{} B", packet.length), 80),
        (packet.summary, 0),
    ];
    for (text, width) in fields {
        let value = label(text, None);
        if width > 0 {
            value.set_width_request(width);
        } else {
            value.set_hexpand(true);
        }
        value.set_ellipsize(gtk::pango::EllipsizeMode::End);
        content.append(&value);
    }
    row.set_widget_name(&format!(
        "{} {} {} {} {}",
        packet.protocol, packet.source, packet.destination, packet.summary, packet.time
    ));
    row.set_tooltip_text(Some(&format!(
        "Frame details\n\nTime: {}\nProtocol: {}\nSource: {}\nDestination: {}\nLength: {} bytes\nInfo: {}\n\nPayload preview\n45 00 00 52 1a 2b 40 00 40 11 a1 b4 ...",
        packet.time, packet.protocol, packet.source, packet.destination, packet.length, packet.summary
    )));
    row.set_child(Some(&content));
    row
}

fn chart() -> DrawingArea {
    let chart = DrawingArea::new();
    chart.set_content_width(600);
    chart.set_content_height(150);
    chart.set_hexpand(true);
    chart.set_draw_func(|_, cr: &cairo::Context, width, height| {
        let width = f64::from(width);
        let height = f64::from(height);
        cr.set_source_rgb(0.09, 0.13, 0.22);
        let _ = cr.paint();
        cr.set_line_width(1.0);
        cr.set_source_rgba(0.45, 0.57, 0.72, 0.2);
        for step in 1..4 {
            let y = height * f64::from(step) / 4.0;
            cr.move_to(0.0, y);
            cr.line_to(width, y);
        }
        let _ = cr.stroke();
        let points = [0.76, 0.62, 0.72, 0.39, 0.56, 0.30, 0.50, 0.18, 0.35, 0.22];
        cr.set_source_rgb(0.22, 0.74, 0.95);
        cr.set_line_width(2.5);
        for (i, value) in points.iter().enumerate() {
            let x = (i as f64) * width / ((points.len() - 1) as f64);
            let y = height * value;
            if i == 0 { cr.move_to(x, y); } else { cr.line_to(x, y); }
        }
        let _ = cr.stroke();
    });
    chart
}

fn build_ui(app: &Application) {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../gui/app.css"));
    StyleContext::add_provider_for_display(
        &gdk::Display::default().expect("A display is required"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Network Visualizer")
        .default_width(1320)
        .default_height(820)
        .build();

    let header = HeaderBar::new();
    header.set_title_widget(Some(&Label::new(Some("Network Visualizer"))));
    let capture = Button::with_label("●  Capture");
    capture.add_css_class("suggested-action");
    header.pack_end(&capture);
    window.set_titlebar(Some(&header));

    let root = GtkBox::new(Orientation::Vertical, 0);
    let toolbar = GtkBox::new(Orientation::Horizontal, 10);
    toolbar.add_css_class("toolbar");
    set_margins(&toolbar, 10);
    toolbar.append(&label("Interface", Some("muted")));
    toolbar.append(&DropDown::from_strings(&["en0 · Wi-Fi", "lo0 · Loopback"]));
    let filter = SearchEntry::new();
    filter.set_placeholder_text(Some("Filter packets: IP, port, protocol…"));
    filter.set_hexpand(true);
    toolbar.append(&filter);
    toolbar.append(&Button::with_label("Clear"));
    root.append(&toolbar);

    let layout = Paned::new(Orientation::Horizontal);
    layout.set_wide_handle(true);
    layout.set_vexpand(true);

    let sidebar = GtkBox::new(Orientation::Vertical, 10);
    sidebar.add_css_class("sidebar");
    set_margins(&sidebar, 12);
    sidebar.append(&label("PROTOCOLS", Some("column-header")));
    sidebar.append(&protocol_card("All traffic", "2,112", "#7dd3fc"));
    sidebar.append(&protocol_card("TCP", "1,240", "#60a5fa"));
    sidebar.append(&protocol_card("UDP", "860", "#34d399"));
    sidebar.append(&protocol_card("ICMP", "12", "#fbbf24"));
    sidebar.append(&protocol_card("ARP", "8", "#c084fc"));
    sidebar.append(&Separator::new(Orientation::Horizontal));
    sidebar.append(&label("CAPTURE", Some("column-header")));
    sidebar.append(&label("Status  •  Running", None));
    sidebar.append(&label("Retention  •  250 packets", Some("muted")));
    layout.set_start_child(Some(&sidebar));
    layout.set_resize_start_child(false);
    layout.set_shrink_start_child(false);

    let content = GtkBox::new(Orientation::Vertical, 12);
    set_margins(&content, 12);
    let stats = GtkBox::new(Orientation::Horizontal, 10);
    let (pps_card, pps) = stat_card("PACKETS / SECOND", "248");
    let (throughput_card, throughput) = stat_card("THROUGHPUT", "1.84 MB/s");
    let (total_card, total) = stat_card("PACKETS CAPTURED", "2,112");
    let (drop_card, _) = stat_card("DROPPED", "0");
    stats.append(&pps_card);
    stats.append(&throughput_card);
    stats.append(&total_card);
    stats.append(&drop_card);
    content.append(&stats);

    let chart_panel = Frame::new(None);
    chart_panel.add_css_class("chart-panel");
    let chart_box = GtkBox::new(Orientation::Vertical, 6);
    set_margins(&chart_box, 12);
    chart_box.append(&label("TRAFFIC OVER TIME", Some("column-header")));
    chart_box.append(&chart());
    chart_panel.set_child(Some(&chart_box));
    content.append(&chart_panel);

    let packet_panel = Frame::new(None);
    packet_panel.add_css_class("packet-panel");
    packet_panel.set_vexpand(true);
    let packet_box = GtkBox::new(Orientation::Vertical, 0);
    let column_names = GtkBox::new(Orientation::Horizontal, 12);
    set_margins(&column_names, 10);
    for (title, width) in [("TIME", 125), ("PROTOCOL", 70), ("SOURCE", 190), ("DESTINATION", 205), ("LENGTH", 80), ("INFO", 0)] {
        let header = label(title, Some("column-header"));
        if width > 0 { header.set_width_request(width); } else { header.set_hexpand(true); }
        column_names.append(&header);
    }
    packet_box.append(&column_names);
    packet_box.append(&Separator::new(Orientation::Horizontal));
    let packet_list = ListBox::new();
    packet_list.set_selection_mode(gtk::SelectionMode::Single);
    for packet in PACKETS { packet_list.append(&packet_row(packet)); }
    let scroll = ScrolledWindow::new();
    scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_child(Some(&packet_list));
    packet_box.append(&scroll);
    packet_panel.set_child(Some(&packet_box));
    content.append(&packet_panel);

    let detail_panel = Frame::new(None);
    detail_panel.add_css_class("detail-panel");
    let detail_box = GtkBox::new(Orientation::Vertical, 6);
    set_margins(&detail_box, 10);
    detail_box.append(&label("SELECTED PACKET", Some("column-header")));
    let detail_buffer = TextBuffer::new(None);
    detail_buffer.set_text("Select a packet to inspect its decoded headers and payload preview.");
    let detail = TextView::with_buffer(&detail_buffer);
    detail.set_editable(false);
    detail.set_monospace(true);
    detail.set_cursor_visible(false);
    detail.set_size_request(-1, 125);
    detail_box.append(&detail);
    detail_panel.set_child(Some(&detail_box));
    content.append(&detail_panel);

    packet_list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            if let Some(text) = row.tooltip_text() { detail_buffer.set_text(&text); }
        }
    });
    let filter_list = packet_list.clone();
    filter.connect_search_changed(move |search| {
        let query = search.text().to_lowercase();
        let mut child = filter_list.first_child();
        while let Some(widget) = child {
            let next = widget.next_sibling();
            widget.set_visible(widget.widget_name().to_lowercase().contains(query.as_str()));
            child = next;
        }
    });

    let tick = Rc::new(Cell::new(0usize));
    let tick_list = packet_list.clone();
    glib::timeout_add_local(Duration::from_millis(900), move || {
        let index = tick.get() % PACKETS.len();
        tick.set(tick.get() + 1);
        tick_list.prepend(&packet_row(PACKETS[index]));
        while tick_list.observe_children().n_items() > 250 {
            if let Some(last) = tick_list.last_child() { tick_list.remove(&last); }
        }
        pps.set_text(&format!("{}", 230 + (tick.get() % 45)));
        throughput.set_text(&format!("{:.2} MB/s", 1.65 + (tick.get() % 30) as f64 / 100.0));
        total.set_text(&format!("{}", 2_112 + tick.get()));
        glib::ControlFlow::Continue
    });

    layout.set_end_child(Some(&content));
    root.append(&layout);
    window.set_child(Some(&root));
    window.present();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}
