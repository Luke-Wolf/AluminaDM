use iced::{
    executor,
    widget::{button, column, row},
    Application, Command, Settings, Theme,
};
fn main() -> iced::Result {
    GreeterApp::run(Settings::default())
}

#[derive(Default)]
struct GreeterApp {
    user_name: String,
    password: String,
}

enum Message {}

impl Application for GreeterApp {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("AluminaDM")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match Message {}
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = column![
            "Login",
            //row!["username", text_input("username", &self.user_name)],
            //row!["password", text_input("password", &self.password)],
            button("Login")
        ];
    }
}
