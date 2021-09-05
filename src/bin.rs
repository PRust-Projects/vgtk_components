#![recursion_limit = "512"]
use std::cell::RefCell;

use lazy_static::lazy_static;
use vgtk::ext::*;
use vgtk::lib::gdk;
use vgtk::lib::gio::{ApplicationExt, ApplicationFlags};
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction};

use vgtk_components::checkbox::CheckBox;
use vgtk_components::date::{Date, DateInput};
use vgtk_components::dropdown::Dropdown;
use vgtk_components::file_chooser::FileChooser;
use vgtk_components::passwordbox::PasswordBox;
use vgtk_components::phone_number::PhoneNumber;
use vgtk_components::progress_bar::ProgressBar;
use vgtk_components::textbox::TextBox;

const STYLE: &str = "
progress, trough {
  min-height: 30px;
}
trough {
  min-width: 300px;
}
";

lazy_static! {
    static ref DROPDOWN_LIST: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("delta")
    ];
    static ref DATE: Date = Date {
        month: String::from("Jan"),
        day: String::from("01"),
        year: String::from("1990"),
    };
    static ref TEXTBOX_TEXT: String = String::from("Hello");
}

#[derive(Clone, Debug)]
struct Model {
    progress_fraction: f64,
    progress_text: RefCell<String>,
    text_buffer: RefCell<TextBuffer>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            progress_fraction: 0.0,
            progress_text: RefCell::new(String::from("0%")),
            text_buffer: RefCell::new(TextBuffer::new::<TextTagTable>(None)),
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    LoadCSS,
    PrintBoolean { value: bool },
    PrintString { value: String },
    StartProgressBar,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::LoadCSS => {
                let provider = CssProvider::new();
                provider
                    .load_from_data(STYLE.as_bytes())
                    .expect("Failed to load CSS");
                StyleContext::add_provider_for_screen(
                    &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                    &provider,
                    STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
                UpdateAction::Render
            }
            Message::PrintBoolean { value } => {
                println!("{}", value);
                UpdateAction::None
            }
            Message::PrintString { value } => {
                println!("{}", value);
                UpdateAction::None
            }
            Message::StartProgressBar => {
                if self.progress_fraction + 0.1 <= 1.0 {
                    self.progress_fraction += 0.1;
                    self.progress_text
                        .replace(format!("{}%", (self.progress_fraction * 100.0).to_string()));
                    UpdateAction::Render
                } else {
                    UpdateAction::None
                }
            }
        }
    }

    fn view(&self) -> vgtk::VNode<Self> {
        gtk! {
            <Application::new_unwrap(Some("org.hydra.contacts"), ApplicationFlags::empty()) on startup=|_| Message::LoadCSS>
                <Window default_width=400 default_height=300 border_width=20 on destroy=|_| Message::Exit>
                    <Box orientation=Orientation::Vertical spacing=20>
                        <@Dropdown label=Some(String::from("Test Dropdown:")) items=DROPDOWN_LIST.clone() full_width=true/>
                        <Box orientation=Orientation::Horizontal>
                            <@Dropdown label=Some(String::from("Test Dropdown:")) items=DROPDOWN_LIST.clone() orientation=Orientation::Vertical />
                        </Box>
                        <@Dropdown label=Some(String::from("Test Dropdown:")) items=DROPDOWN_LIST.clone() />
                        <@Dropdown items=DROPDOWN_LIST.clone() on_select=|value| Message::PrintString { value } />
                        <@DateInput editable=true orientation=Orientation::Vertical spacing=20 />
                        <@DateInput label=Some(String::from("Date:")) editable=true min_year=2000 max_year=2100 />
                        <@DateInput label=Some(String::from("Date:")) full_width=true date=DATE.clone() editable=false />
                        <@TextBox label=Some(String::from("Test Textbox:")) text=TEXTBOX_TEXT.clone() on_changed=|value| Message::PrintString { value } />
                        <@TextBox text=TEXTBOX_TEXT.clone() on_changed=|value| Message::PrintString { value } />
                        <@PasswordBox label=Some(String::from("Password")) full_width=true on_changed=|value| Message::PrintString { value } />
                        <@PhoneNumber label=Some(String::from("Phone Number:")) full_width=true on_changed=|value| Message::PrintString { value } />
                        <Box spacing=10>
                            <@ProgressBar progress_text=self.progress_text.borrow().clone() progress_fraction=self.progress_fraction />
                            <Button label="Start" on clicked=|_| Message::StartProgressBar />
                        </Box>
                        <@CheckBox label=Some(String::from("Test Checkbox")) checked=true on_toggled=|value| Message::PrintBoolean { value } />
                        <@CheckBox checked=false on_toggled=|value| Message::PrintBoolean { value } />
                        <@FileChooser label=Some(String::from("Test Filechooser")) dialog_action=FileChooserAction::Open show_path=true />
                        <@FileChooser label=Some(String::from("Test Filechooser")) dialog_action=FileChooserAction::SelectFolder show_path=true dialog_title="Select Folder" />
                        <Box Box::expand=true />
                    </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
