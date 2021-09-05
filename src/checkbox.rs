use vgtk::lib::gtk::prelude::*;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Callback, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub struct CheckBox {
    pub label: Option<String>,
    pub checked: bool,
    pub spacing: i32,
    pub widget_name: String,
    pub label_widget: String,
    pub on_toggled: Callback<bool>,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            label: None,
            checked: false,
            spacing: 10,
            widget_name: String::new(),
            label_widget: String::new(),
            on_toggled: Callback::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CheckBoxMessage {
    Toggle,
}

impl Component for CheckBox {
    type Message = CheckBoxMessage;
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
            CheckBoxMessage::Toggle => {
                self.checked = !self.checked;
                self.on_toggled.send(self.checked);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=Orientation::Horizontal spacing=self.spacing>
                <CheckButton active=self.checked widget_name=self.widget_name.clone() on toggled=|_| CheckBoxMessage::Toggle />
                {
                    gtk_if!(self.label.is_some() => {
                        <Label label=self.label.as_ref().unwrap().to_string() widget_name=self.label_widget.clone() />
                    })
                }
            </Box>
        }
    }
}
