use vgtk::lib::gtk::prelude::*;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Callback, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub struct PhoneNumber {
    pub label: Option<String>,
    pub text: String,
    pub orientation: Orientation,
    pub spacing: i32,
    pub full_width: bool,
    pub widget_name: String,
    pub label_widget: String,
    pub on_changed: Callback<String>,
}

impl Default for PhoneNumber {
    fn default() -> Self {
        Self {
            label: None,
            text: String::new(),
            orientation: Orientation::Horizontal,
            spacing: 10,
            full_width: false,
            widget_name: String::new(),
            label_widget: String::new(),
            on_changed: Callback::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PhoneNumberMessage {
    SetValue { value: String },
}

impl Component for PhoneNumber {
    type Message = PhoneNumberMessage;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            PhoneNumberMessage::SetValue { value } => {
                self.on_changed.send(value);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=self.orientation spacing=self.spacing>
                {
                    gtk_if!(self.label.is_some() => {
                        <Label label=self.label.as_ref().unwrap().to_string() widget_name=self.label_widget.clone()
                            halign=Align::Start />
                    })
                }
                {
                    let pack_type = if self.full_width { PackType::End } else { PackType::Start };
                    gtk! {
                        <Entry widget_name=self.widget_name.clone() Box::pack_type=pack_type max_length=14 on insert_text=|entry, text, position| {
                            let mut raw_phone_number = entry.get_text().to_string();
                            raw_phone_number.push_str(text);

                            let phone_number = unpretty_format_phone_number(&raw_phone_number);
                            let pretty_phone_number = pretty_format_phone_number(&phone_number);
                            if raw_phone_number != pretty_phone_number {
                                // ignore the last char as that would be added by user input
                                entry.set_text(&pretty_format_phone_number(&phone_number[..phone_number.len()-1]));
                                *position = pretty_phone_number.len() as i32;
                            }
                            PhoneNumberMessage::SetValue {
                                value: phone_number,
                            }
                        } />
                    }
                }
            </Box>
        }
    }

}

fn pretty_format_phone_number(phone_number: &str) -> String {
    let mut pretty_phone_number = String::from("(");
    if phone_number.len() < 3 {
        pretty_phone_number.push_str(phone_number);
    } else if phone_number.len() < 6 {
        pretty_phone_number.push_str(&phone_number[..3]);
        pretty_phone_number.push_str(") ");
        pretty_phone_number.push_str(&phone_number[3..]);
    } else {
        pretty_phone_number.push_str(&phone_number[..3]);
        pretty_phone_number.push_str(") ");
        pretty_phone_number.push_str(&phone_number[3..6]);
        pretty_phone_number.push('-');
        pretty_phone_number.push_str(&phone_number[6..]);
    }

    pretty_phone_number
}

fn unpretty_format_phone_number(pretty_phone_number: &str) -> String {
    pretty_phone_number.chars()
        .filter(|c| *c != '(' && *c != ')' && *c != ' ' && *c != '-')
        .collect()
}
