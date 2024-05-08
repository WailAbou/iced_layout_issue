use flexi_logger::{Duplicate, Logger};
use iced::widget::{button, column, horizontal_space, scrollable, stack, text};
use iced::{alignment, Background, Border, Color, Settings};
use iced::{
    widget::{container, row},
    Element,
};
use iced::{Alignment, Length};

pub fn main() -> iced::Result {
    // std::env::set_var("ICED_BACKEND", "tiny-skia");
    Logger::try_with_env_or_str("Info")
        .unwrap()
        // .log_to_file(flexi_logger::FileSpec::default().basename("Demo"))
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();

    iced::program("Card View Test", TestApp::update, TestApp::view)
        .settings(Settings {
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct TestApp {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl TestApp {
    fn update(&mut self, _: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let content: Element<_> = container(
            column![
                text("Title").size(50),
                card_view(
                    "Title 1",
                    column![row![text("Test1"), horizontal_space(), button("Action")]].into()
                ),
                card_view(
                    "Title 2",
                    column![row![text("Test2"), horizontal_space(), button("Action")]].into()
                ),
                card_view(
                    "Title 3",
                    column![row![text("Test3"), horizontal_space(), button("Action")]].into()
                ),
                stack_test(),
            ]
            .spacing(20)
            .width(Length::Fill)
            .padding(20)
            .align_items(Alignment::Center),
        )
        .into();
        scrollable(content).into()
    }
}

fn stack_test<'a>() -> Element<'a, Message> {
    stack([
        card_view(
            "Title 4",
            column![row![text("Test4"), horizontal_space(), button("Action")]].into(),
        ),
        card_view(
            "Title 5",
            column![row![text("Test5"), horizontal_space(), button("Action")]].into(),
        ),
    ])
    .into()
}

fn card_view<'a>(title: &str, content: Element<'a, Message>) -> Element<'a, Message> {
    container(
        column![
            row![text(title.to_string())
                .size(18)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Top),]
            .width(Length::Fill)
            .spacing(8),
            content
        ]
        .spacing(8),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(246, 247, 250))),
        border: Border::rounded(8),
        ..Default::default()
    })
    .padding(20)
    .into()
}
