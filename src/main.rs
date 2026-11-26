use std::path::{Path, PathBuf};

use iced::{
    Alignment, Font, Length,
    widget::{Button, button, column, container, row, space, text},
};

pub use iced_fonts::codicon as icons;

const TITLE: &str = concat!("movui v", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    ChooseSource(PathBuf),
    ChooseDestination(PathBuf),
    StartSync,
}

pub struct App {
    source_path: PathBuf,
    dest_path: PathBuf,
}

fn choose_path() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Choose path")
        .pick_folder()
}

impl App {
    pub fn new() -> Self {
        Self {
            source_path: PathBuf::new(),
            dest_path: PathBuf::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        println!("message: {message:#?}");

        match message {
            Message::Nothing => {}
            Message::ChooseSource(source) => {
                self.source_path = source;
            }
            Message::ChooseDestination(destination) => {
                self.dest_path = destination;
            }
            _ => todo!("{message:?}"),
        }
    }

    fn path_button<'a>(
        path: &'a Path,
        text_if_empty: &'a str,
        msg: fn(PathBuf) -> Message,
    ) -> Button<'a, Message> {
        let btn_text = {
            if path.is_empty() {
                text_if_empty.to_string()
            } else {
                path.to_string_lossy().to_string()
            }
        };

        let btn_inside = container(text(btn_text).size(15).font(Font {
            family: iced::font::Family::Monospace,
            ..Default::default()
        }))
        .padding(5)
        .center_x(Length::Fill);
        button(btn_inside)
            .on_press_with(move || choose_path().map(msg).unwrap_or(Message::Nothing))
            .width(Length::Fill)
    }

    fn button_start(&self) -> Button<'_, Message> {
        let btn_text = text("start").size(25);

        let enabled = !self.source_path.is_empty() && !self.dest_path.is_empty();

        button(btn_text)
            .style(button::success)
            .on_press_maybe(enabled.then_some(Message::StartSync))
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let title = text("Movui")
            .size(50)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .center();

        let arrow = icons::arrow_down().size(35);

        let btn_a = Self::path_button(&self.source_path, "Choose Source", Message::ChooseSource);
        let btn_b = Self::path_button(
            &self.dest_path,
            "Choose Destination",
            Message::ChooseDestination,
        );

        let btn_start = self.button_start();

        container(
            column![title, btn_a, arrow, btn_b, btn_start]
                .align_x(Alignment::Center)
                .spacing(10),
        )
        .center_x(Length::Fill)
        .padding(25)
        .into()
    }
}

fn main() {
    println!("== opening window ==");
    if let Err(e) = iced::application(App::new, App::update, App::view)
        .font(iced_fonts::CODICON_FONT_BYTES)
        .resizable(true)
        .window_size((400, 400))
        .title(TITLE)
        .centered()
        .run()
    {
        eprintln!("ERROR: {e}");
    }
    println!("== closing window ==")
}
