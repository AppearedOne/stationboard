use iced::event::{self, Event};
use iced::time::every;
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule, horizontal_space, row,
    scrollable, stack, text, text_input, tooltip, vertical_rule, vertical_space,
};
use iced::{alignment, Alignment, Element, Font, Length, Padding, Subscription, Task, Theme};
use iced::{window, Color};
use serde::Deserialize;
use std::time::{Duration, Instant};
pub mod colors;
pub mod error;
pub mod time;
use error::*;
use time::*;
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
    fontsize: f32,
}
impl Settings {
    pub fn default() -> Self {
        Settings {
            filter: false,
            fontsize: 30.0,
        }
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
        every(Duration::from_secs(5)).map(Message::Send)
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
        let mut list = column![].width(Length::Fill).height(Length::Fill);

        for dep in &self.deps {
            let time_text = match &self.showDelta {
                true => text(get_delta_time(&dep.stop.departure))
                    .size(self.settings.fontsize)
                    .color(Color::WHITE),
                false => text(format_time(&dep.stop.departure))
                    .size(self.settings.fontsize)
                    .color(Color::WHITE),
            };
            let icon = container(
                text(dep.number.clone())
                    .color(iced::Color::BLACK)
                    .size(self.settings.fontsize),
            )
            .padding(20)
            .width(Length::Fixed(150.0))
            .style(|theme: &Theme| {
                container::dark(theme).background(colors::line_color(&dep.number))
            })
            .align_y(Alignment::Center)
            .align_x(Alignment::Center);

            let delaytime = match dep.stop.delay {
                Some(v) => v,
                None => 0,
            };
            let delay_t = text(format!("+{}", delaytime))
                .size(self.settings.fontsize)
                .color(Color::WHITE);
            let delay = if delaytime <= 2 {
                delay_t
            } else {
                delay_t.color(colors::from_rgb(255.0, 0.0, 0.0))
            };
            list = list.push(container(
                row![
                    icon,
                    horizontal_space(),
                    text(dep.to.clone())
                        .size(self.settings.fontsize)
                        .color(Color::WHITE),
                    horizontal_space(),
                    time_text,
                    horizontal_space().width(20.0),
                    delay,
                ]
                .padding(3)
                .align_y(Alignment::Center)
                .spacing(5),
            ));
        }

        list.into()
    }

    fn theme(&self) -> Theme {
        let mut default = Theme::Dark.palette();
        default.background = colors::from_rgb(7.0, 121.0, 204.0);
        Theme::custom("ZVV".to_string(), default)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().0
    }
}
