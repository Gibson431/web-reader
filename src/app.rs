pub mod pages;
pub mod utils;

use std::collections::HashMap;
use std::path::Display;

use crate::core::book::Book;
use crate::core::source::{self, *};
use crate::fl;
use cosmic::app::{message, Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::window::Icon;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, icon, menu, nav_bar};
use cosmic::{cosmic_theme, theme, ApplicationExt, Apply, Element};

const REPOSITORY: &str = "https://github.com/Gibson431/web-reader";

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Default)]
pub struct App {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    /// A model that contains all of the pages assigned to the nav bar panel.
    nav: nav_bar::Model,

    // Data
    books: HashMap<String, Book>,               // url, book
    book_covers: HashMap<String, bytes::Bytes>, // book name, bytes

    // Explore page
    explore_input: String,
    explore_results: Vec<String>,

    // Library page
    library_input: String,
    library_results: Vec<String>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),

    ExploreSearch(String),
    ExploreInputChanged(String),
    ExploreResult(Vec<String>),

    LibrarySearch(String),
    LibraryInputChanged(String),
    LibraryResult(Vec<String>),

    // AddSearchBook(Book),
    AddBook(String, Book),
    AddThumbnail(String, bytes::Bytes),
    Log(LogMessage),
    Ignore,
}

#[derive(Debug, Clone)]
pub enum LogMessage {
    Log(String),
    Error(String),
}

/// Identifies a page in the application.
pub enum Page {
    Explore,
    Library,
    History,
}

/// Identifies a context page to display in the context drawer.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    Settings,
    BookContext(Book),
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
            Self::BookContext(book) => book.name.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    Settings,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::Settings => Message::ToggleContextPage(ContextPage::Settings),
        }
    }
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl cosmic::Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Instructs the cosmic runtime to use this model as the nav bar model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text("Explore")
            .data::<Page>(Page::Explore)
            .icon(icon::from_name("applications-science-symbolic"))
            .activate();

        nav.insert()
            .text("Library")
            .data::<Page>(Page::Library)
            .icon(icon::from_name("applications-system-symbolic"));

        nav.insert()
            .text("History")
            .data::<Page>(Page::History)
            .icon(icon::from_name("applications-games-symbolic"));

        let mut app = App {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            nav,
            ..Default::default()
        };

        let command = app.update_titles();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button(fl!("about"), MenuAction::About),
                    menu::Item::Button(fl!("settings"), MenuAction::Settings),
                ],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        let page_view = widget::responsive(|size| match self.nav.active_data::<Page>() {
            Some(Page::Explore) => self.view_explore(size),
            Some(Page::Library) => self.view_library(size),
            Some(Page::History) => self.view_history(size),
            _ => widget::text::title1(fl!("welcome"))
                .apply(widget::container)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
        });
        page_view.into()
    }

    /// Application messages are handled here. The application state can be modified based on
    /// what message was received. Commands may be returned for asynchronous execution on a
    /// background thread managed by the application's executor.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Log(log) => {
                dbg!(log);
            }
            Message::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }
            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page.clone();
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }
            Message::ExploreInputChanged(input) => {
                self.explore_input = input;
            }
            Message::LibraryInputChanged(input) => {
                self.library_input = input;
            }
            Message::ExploreSearch(term) => {
                let mut commands = vec![];
                // Royal Road search
                commands.push(Command::perform(
                    async move {
                        let source = source::RoyalRoadSource::new();
                        dbg!(format!("searching for: {}", &term));
                        let res = source.search(term).await;
                        match res {
                            Ok(content) => {
                                cosmic::app::message::app(Message::ExploreResult(content))
                            }
                            Err(e) => cosmic::app::message::app(Message::Log(LogMessage::Error(
                                e.to_string(),
                            ))),
                        }
                    },
                    |x| x,
                ));

                return Command::batch(commands);
            }
            Message::ExploreResult(res) => {
                self.explore_results = res;
                let mut commands = vec![];
                for url in self.explore_results.clone() {
                    // Skip already available books
                    if let Some(_) = self.books.get(&url) {
                        continue;
                    }

                    // Create command to scrape book info
                    commands.push(Command::perform(
                        async move {
                            let source = RoyalRoadSource::new();
                            let book = source.scrape_book(url.clone()).await.unwrap();
                            message::app(Message::AddBook(url, book))
                        },
                        |x| x,
                    ))
                }
                return Command::batch(commands);
            }
            Message::LibrarySearch(_) => todo!("library search"),
            Message::LibraryResult(_) => todo!("library result"),

            Message::AddBook(url, book) => {
                _ = self.books.insert(url.clone(), book.clone());

                let image_url = match book.image {
                    Some(url) => url,
                    None => return Command::none(),
                };

                if !self.book_covers.contains_key(&book.name) {
                    return Command::perform(
                        async move {
                            match App::download_book_cover(image_url).await {
                                Ok(content) => {
                                    message::app(Message::AddThumbnail(book.name.clone(), content))
                                }
                                Err(e) => {
                                    dbg!(e);
                                    message::none()
                                }
                            }
                        },
                        |x| x,
                    );
                }
            }
            Message::AddThumbnail(name, content) => {
                self.book_covers.insert(name, content);
            }
            Message::Ignore => (),
        }
        Command::none()
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match &self.context_page {
            ContextPage::About => self.about_context(),
            ContextPage::Settings => self.settings_context(),
            ContextPage::BookContext(book) => self.book_context(&book),
        })
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);

        self.update_titles()
    }
}

impl App {
    /// The about page for this app.
    pub fn about_context(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon_style = cosmic::iced::widget::svg::Appearance {
            color: Some(cosmic::iced::Color::WHITE),
        };
        // let icon_style = cosmic::iced_widget::Theme::Light;
        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/48x48/apps/settings-svgrepo-com.svg")[..],
        ))
        .height(48)
        .width(48);

        let title = widget::text::title3(fl!("app-title"));

        let title_row = widget::row()
            .push(icon)
            .push(title)
            .spacing(space_xxs)
            .width(Length::Shrink)
            .align_items(Alignment::Center);

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            // .push(icon)
            // .push(title)
            .push(title_row)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .width(Length::Fill)
            .into()
    }

    /// The settings page for this app.
    pub fn settings_context(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/48x48/apps/settings-svgrepo-com.svg")[..],
        ))
        .height(64);

        let display_options = widget::column()
            .push(widget::row().push(widget::text("Display")))
            .align_items(Alignment::Center);
        // let title = widget::text::title3(fl!("app-title"));
        let contact_info = widget::column()
            .push(
                widget::button::link(REPOSITORY)
                    .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
                    .padding(0),
            )
            .align_items(Alignment::Center);

        widget::column()
            .push(icon)
            .push(widget::divider::horizontal::default())
            .push(display_options)
            .push(widget::divider::horizontal::default())
            // .push(title)
            .push(contact_info)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    // the book context page
    pub fn book_context(&self, book: &Book) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/24x24/apps/book-svgrepo-com.svg")[..],
        ))
        .height(48);

        let title = widget::text::title3(fl!("app-title"));

        let title_row = widget::row().push(icon).push(title).spacing(space_xxs);

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            // .push(icon)
            // .push(title)
            .push(title_row)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_titles(&mut self) -> Command<Message> {
        let mut window_title = fl!("app-title");
        let mut header_title = String::new();

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
            header_title.push_str(page);
        }

        self.set_header_title(header_title);
        // winit
        self.set_window_title(window_title)
        // wayland
        // self.set_window_title(window_title, cosmic::iced::window::Id::MAIN)
    }
}
