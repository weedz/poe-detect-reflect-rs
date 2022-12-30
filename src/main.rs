use std::io::{self, Cursor};
use notify_rust::Notification;
use rodio::Sink;

fn check_for_reflect(s: &str) -> Option<&str> {
    let m: Vec<_> = s.match_indices("monsters reflect ").collect();
    match m.get(0) {
        Some(x) => {
            let reflect_type = &s[x.0+17..];
            Some(reflect_type)
        }
        _ => None
    }
}

fn check_cannot_leech(s: &str) -> Option<bool> {
    if s.contains("cannot leech from monsters") {
        return Some(true)
    }
    None
}

fn show_notification(notification_summary: &str) -> Option<notify_rust::NotificationHandle> {
    Notification::new()
        .summary(notification_summary)
        .timeout(5000)
        .show().ok()
}


fn sound_alert(sink: &Sink) {
    if sink.empty() {
        let bytes = include_bytes!("assets/warning.wav");
        let source = rodio::Decoder::new(Cursor::new(bytes)).unwrap();
        sink.append(source);
    }
}

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.2);

    let lines = io::stdin().lines();

    for line in lines {
        let line_text = line.expect("Failed to read from STDIN").to_lowercase();

        match check_for_reflect(line_text.as_str()) {
            Some(reflect_type) => {
                println!("Found reflect: '{}'", reflect_type);
                show_notification(std::format!("Found reflect: '{}'", reflect_type).as_str());
                sound_alert(&sink);
            }
            None => {}
        }
        match check_cannot_leech(line_text.as_str()) {
            Some(_) => {
                println!("Found 'cannot leech'");
                show_notification("Found 'cannot leech'");
                sound_alert(&sink);
            }
            None => {}
        }
    }
    sink.sleep_until_end();
}
