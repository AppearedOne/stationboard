use std::time::{Duration, Instant};

use iced::advanced::graphics::futures::backend::default::time;
use iced::event::{self, Event};
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, row,
    scrollable, stack, text, text_input, tooltip, vertical_rule, vertical_space,
};
use iced::window;
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};
use serde::Deserialize;
pub mod error;
use chrono::{Local, NaiveTime};
use error::*;

fn format_time(iso_time: &str) -> String {
    let datetime: iso8601::DateTime = iso8601::datetime(iso_time).unwrap();
    let mut h = datetime.time.hour.to_string();
    let mut m = datetime.time.minute.to_string();

    if h.len() == 1 {
        h.insert(0, '0');
    }
    if m.len() == 1 {
        m.insert(0, '0');
    }

    let s = String::from(format!("{}:{}", h, m));
    s
}

fn get_delta_time(iso_time: &str) -> String {
    let dep_time =
        NaiveTime::parse_from_str(&format_time(iso_time), "%H:%M").expect("invalid format");
    let now = Local::now().time();
    let delta_minutes = if now < dep_time {
        dep_time.signed_duration_since(now).num_minutes()
    } else {
        now.signed_duration_since(dep_time).num_minutes()
    };
    return delta_minutes.to_string();
}
#[tokio::main]
async fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .exit_on_close_request(false)
        .run_with(|| {
            (
                App::new().0,
                Task::perform(fetch_data(true), Message::Recieve),
            )
        })
}

#[derive(Debug, Deserialize, Clone)]
struct Departure {
    number: String,
    stop: Stop,
    to: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Stop {
    departure: String,
    #[serde(default)]
    delay: Option<i64>,
}

async fn fetch_data(filter_endpoint: bool) -> Result<Vec<Departure>, Error> {
    let resp_text = reqwest::get(
        "http://transport.opendata.ch/v1/stationboard?station=Zürich%20Zentrum%20Witikon&limit=50",
    )
    .await?
    .text()
    .await?;

    let terminals = ["Klusplatz", "Hermetsc"];
    let json_value: serde_json::Value = serde_json::from_str(&resp_text)?;
    let stationboard = &json_value["stationboard"];
    let mut departures: Vec<Departure> = Vec::new();
    for departure_json in stationboard.as_array().unwrap() {
        let departure: Departure = serde_json::from_value(departure_json.clone())?;
        if filter_endpoint {
            for t in terminals {
                if departure.to.contains(t) {
                    departures.push(departure);
                    break;
                }
            }
        } else {
            departures.push(departure);
        }
    }

    Ok(departures)
}

#[derive(Debug, Clone)]
enum Message {
    Send(Instant),
    Recieve(Result<Vec<Departure>, Error>),
}

enum Status {
    Working,
    Error,
}

#[derive(Debug, Deserialize)]
struct Settings {
    filter: bool,
}
impl Settings {
    pub fn default() -> Self {
        Settings { filter: false }
    }
}

struct App {
    status: Status,
    deps: Vec<Departure>,
    settings: Settings,
    showDelta: bool,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                status: Status::Working,
                deps: Vec::new(),
                settings: Settings::default(),
                showDelta: false,
            },
            Task::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Application")
    }
    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(5)).map(Message::Send)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Send(_) => {
                self.showDelta = !self.showDelta;
                return Task::perform(fetch_data(true), Message::Recieve);
            }
            Message::Recieve(val) => match val {
                Err(_) => self.status = Status::Error,
                Ok(data) => self.deps = data,
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let mut list = column![]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        for dep in &self.deps {
            let time_text = match &self.showDelta {
                true => text(get_delta_time(&dep.stop.departure)),
                false => text(format_time(&dep.stop.departure)),
            };
            list = list.push(container(
                row![
                    text(dep.number.clone()),
                    horizontal_space(),
                    text(dep.to.clone()),
                    horizontal_space(),
                    time_text
                ]
                .align_y(Alignment::Center)
                .padding(10)
                .spacing(5),
            ));
        }

        list.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().0
    }
}
