use iced::color;
use iced::widget::{button, column, horizontal_space, stack, text};
use iced::Length::Fill;
use iced::{
    widget::{container, row},
    Element,
};

pub fn main() -> iced::Result {
    iced::program("Card View Test", TestApp::update, TestApp::view).run()
}

#[derive(Default)]
struct TestApp {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl TestApp {
    fn update(&mut self, _: Message) {}

    fn view(&self) -> Element<'_, Message> {
        stack([
            card_view(
                "Title 1",
                row![text("Subtitle 1"), horizontal_space(), button("Action 1")].into(),
            ),
            card_view(
                "Title 2",
                row![text("Subtitle 2"), horizontal_space(), button("Action 2")].into(),
            ),
        ])
        .width(Fill)
        .into()
    }
}

fn card_view<'a>(title: &str, content: Element<'a, Message>) -> Element<'a, Message> {
    container(column![text(title.to_string()).size(18), content])
        .style(|theme| container::transparent(theme).with_background(color!(246, 247, 250)))
        .padding(8)
        .into()
}
