use vgtk::lib::gtk::prelude::*;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Callback, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub struct TextBox {
    pub label: Option<String>,
    pub text: String,
    pub orientation: Orientation,
    pub spacing: i32,
    pub full_width: bool,
    pub widget_name: String,
    pub label_widget: String,
    pub on_changed: Callback<String>,
}

impl Default for TextBox {
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
pub enum TextBoxMessage {
    SetValue { value: String },
}

impl Component for TextBox {
    type Message = TextBoxMessage;
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
            TextBoxMessage::SetValue { value } => {
                self.on_changed.send(value);
                UpdateAction::None
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
                    if self.full_width {
                        gtk! {
                            <Entry text=self.text.clone() widget_name=self.widget_name.clone() Box::pack_type=PackType::End on changed=|entry| {
                                TextBoxMessage::SetValue {
                                    value: entry.get_text().to_string(),
                                }
                            } />
                        }
                    } else {
                        gtk! {
                            <Entry text=self.text.clone() widget_name=self.widget_name.clone() on changed=|entry| {
                                TextBoxMessage::SetValue {
                                    value: entry.get_text().to_string(),
                                }
                            } />
                        }
                    }
                }
            </Box>
        }
    }
}
