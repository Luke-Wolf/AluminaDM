use iced::{
    executor,
    widget::{button, column, container, row, text, text_input},
    Application, Command, Length, Settings, Theme,
};
fn main() -> iced::Result {
    GreeterApp::run(Settings::default())
}

#[derive(Default)]
struct GreeterApp {
    user_name: String,
    password: String,
}

#[derive(Debug, Clone)]
enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    AttemptLogin,
}

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
        match message {
            Message::UsernameChanged(user_name) => self.user_name = user_name,
            Message::PasswordChanged(password) => self.password = password,
            Message::AttemptLogin => todo!(),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = column![
            "Login",
            row![
                "username",
                text_input("username", &self.user_name, Message::UsernameChanged)
                    .on_submit(Message::UsernameChanged("changed".into()))
            ],
            row![
                text("password"),
                text_input("password", &self.password, Message::PasswordChanged)
                    .password()
                    .on_submit(Message::PasswordChanged("changed".into()))
            ],
            button("Login").on_press(Message::AttemptLogin)
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
