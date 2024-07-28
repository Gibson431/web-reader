pub mod context;
pub mod pages;
pub mod utils;

use std::any::Any;
use std::collections::HashMap;

use crate::core::source::{self, *};
use crate::core::{self, Book, Chapter};
use crate::fl;
use cosmic::app::{message, Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, *};
use cosmic::{cosmic_theme, theme, ApplicationExt, Apply, Element};
use segmented_button::Selectable;

pub const REPOSITORY: &str = "https://github.com/Gibson431/web-reader";

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
    data_manager: core::data::DataManager,
    pages: HashMap<Page, Box<dyn crate::app::pages::page::Page>>,

    // Explore page
    explore_input: String,
    explore_results: Vec<String>,

    // Library page
    library_input: String,
    library_results: Vec<String>,

    // Book page
    book: Option<Book>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    /// Logs a message
    Log(LogMessage),
    /// Initialises the db and cache
    InitializeStorage,
    /// Clears the internal cache and hard database
    ClearStorage,
    /// Launches a website in default browser
    LaunchUrl(String),
    /// Toggles the context page
    ToggleContextPage(ContextPage),

    ReadChapter(Chapter),

    /// Navigate pages
    // PageMessage(crate::app::pages::Message),

    /// Triggers the search from all activated sources
    ExploreSearch(String),
    /// Callback for changing the explore text input field
    ExploreInputChanged(String),
    /// Result for scraping all activated sources for the search term
    ExploreResult(Vec<String>),

    // LibraryMessage(crate::app::pages::library::Message),
    LibraryLoad,
    /// Toggles to in_library flag for cache and db for book
    LibraryToggle(Book),
    /// Searches the library db (unused)
    LibrarySearch(String),
    /// Callback for changing the library text input field
    /// Is used for the library view's fuzzy search
    LibraryInputChanged(String),
    /// The result for the library search
    LibraryResult(Vec<String>),

    /// Adds book to cache, db, and triggers thumbnail scrape
    AddBook(Book),
    /// Result of thumbnail scrape, adds iamge to image cache
    AddThumbnail(Book, bytes::Bytes),
    /// Adds chapter to cache and db
    AddChapter(Chapter),

    /// Rescrapes book from url
    RefreshBook(String),
    /// Rescrapes chapter
    RefreshChapter(Chapter),
    /// Rescrapes tumbnail from book
    RefreshThumbnail(Book),

    /// Null op
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
    Chapter(core::Chapter),
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
            Self::About => fl!("about-context-title"),
            Self::Settings => fl!("settings-context-title"),
            Self::BookContext(_) => fl!("book-context-title"),
            // Self::BookContext(book) => book.name.clone(),
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
            .icon(icon::from_name("applications-science-symbolic"));
        // .activate();

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
            data_manager: core::data::DataManager::new(),
            ..Default::default()
        };

        let command = Command::batch([
            app.update_titles(),
            Command::perform(
                async move { message::app(Message::InitializeStorage) },
                |x| x,
            ),
        ]);

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
        dbg!(self.nav.active());
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
            Message::InitializeStorage => {
                let storage_path = dirs::data_local_dir().unwrap().join(App::APP_ID);

                let errors = self.data_manager.init(storage_path);
                if !errors.is_empty() {
                    let commands: Vec<cosmic::Command<message::Message<Message>>> = errors
                        .iter()
                        .map(|e| self.log_error(format!("{:?}", e)))
                        .collect();
                    return Command::batch(commands);
                }
            }
            Message::ClearStorage => {
                if let Err(e) = self.data_manager.clear_all() {
                    return self.log_error(format!("{:?}", e));
                }
                dbg!("Cleared storage");
            }
            Message::Log(log) => {
                match log {
                    LogMessage::Log(msg) => dbg!(&msg),
                    LogMessage::Error(msg) => dbg!(&msg),
                };
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
                    match self.data_manager.get_book(&url) {
                        // Skip book
                        Ok(Some(_)) => continue,
                        // Create command to scrape book info
                        Ok(_) => commands.push(Command::perform(
                            async move { message::app(Message::RefreshBook(url)) },
                            |x| x,
                        )),
                        // Log error
                        Err(e) => commands.push(self.log_error(format!("{:?}", e))),
                    };
                }
                return Command::batch(commands);
            }
            Message::LibrarySearch(_) => todo!("library search"),
            Message::LibraryResult(_) => todo!("library result"),

            Message::AddBook(book) => {
                if let Err(e) = self.data_manager.set_book(&book) {
                    return self.log_error(format!("{:?}", e));
                };

                return Command::perform(
                    async move { message::app(Message::RefreshThumbnail(book)) },
                    |x| x,
                );
            }
            Message::AddThumbnail(book, bytes) => {
                self.data_manager.set_image_as_bytes_to_cache(&book, bytes);
            }
            Message::AddChapter(chapter) => todo!(),
            Message::LibraryToggle(mut book) => {
                book.in_library = !book.in_library;

                if let Err(e) = self.data_manager.set_book(&book) {
                    return self.log_error(format!("{:?}", e));
                };

                if let Ok(Some(bytes)) = self.data_manager.get_image_as_bytes(&book) {
                    if book.in_library {
                        if let Err(e) = self.data_manager.set_image_as_bytes(&book, bytes) {
                            return self.log_error(format!("{:?}", e));
                        }
                    }
                }
            }
            Message::LibraryLoad => todo!(),
            Message::Ignore => (),
            Message::RefreshBook(book_url) => {
                return Command::perform(
                    async move {
                        // message::app(Message::RefreshBook(url))
                        if book_url.contains(&RoyalRoadSource::new().as_str()) {
                            let source = RoyalRoadSource::new();
                            let book = source.scrape_book(book_url.clone()).await;
                            match book {
                                Ok(b) => message::app(Message::AddBook(b)),
                                Err(e) => message::app(Message::Log(LogMessage::Error(format!(
                                    "{:?}",
                                    e
                                )))),
                            }
                        } else if false {
                            message::app(Message::Ignore)
                        } else {
                            message::app(Message::Ignore)
                        }
                    },
                    |x| x,
                );
            }
            Message::RefreshChapter(chapter) => todo!(),
            Message::RefreshThumbnail(book) => {
                let image_url = match book.image.clone() {
                    Some(url) => url,
                    None => return Command::none(),
                };

                return Command::perform(
                    async move {
                        match App::download_book_cover(image_url).await {
                            Ok(bytes) => message::app(Message::AddThumbnail(book.clone(), bytes)),
                            Err(e) => {
                                message::app(Message::Log(LogMessage::Error(format!("{:?}", e))))
                            }
                        }
                    },
                    |x| x,
                );
            }
            Message::ReadChapter(_) => {
                self.nav.deactivate();
            }
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
            ContextPage::BookContext(book) => self.book_context(book.clone()),
        })
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);
        // self.nav.close();

        self.update_titles()
    }
}

impl App {
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
