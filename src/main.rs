use chrono;
use chrono::DurationRound;
use std::process::Command;
use serde_json;
use serde::Deserialize;
use libnotify;

#[derive(Deserialize, Debug)]
struct Event
{
    title: String,
    start: String,
    end: String,
}


fn get_search_time() -> chrono::NaiveTime {
    let now = chrono::Local::now()
        .duration_trunc(chrono::TimeDelta::minutes(1)).unwrap()
        .time();
    let offset = chrono::TimeDelta::minutes(15);
    now + offset
}

fn notify(x : Event) {
    let n = libnotify::Notification::new(
        x.title.as_str(),
        Some(format!("Start Time: {}\nEnd Time: {}", x.start, x.end).as_str()),
        None);
    n.show().unwrap();
}

fn main() {
    let search_time = get_search_time();
    let output = Command::new("/usr/bin/env").arg("khal")
        .arg("at").arg(format!("{}", search_time))
        .arg("--notstarted")
        .arg("-df").arg("")
        .arg("--format").arg("{{\"title\": \"{title}\",\"start\": \"{start-time}\",\"end\": \"{end-time}\"}}")
        .output().unwrap();

    if !output.status.success() {
        println!("Failed! {}", String::from_utf8(output.stderr).unwrap());
        ()
    }

    libnotify::init("Khal Notifications").unwrap();

    String::from_utf8(output.stdout).unwrap()
        .lines()
        .for_each(|x| match serde_json::from_str(x) {
            Ok(x) => notify(x),
            Err(e) => println!("{}: {}", e, x),
        });

    libnotify::uninit();
}
