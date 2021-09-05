use std::path::PathBuf;

use pango::EllipsizeMode;
use vgtk::lib::gtk::prelude::*;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Callback, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub struct FileChooser {
    pub label: Option<String>,
    pub button_label: String,
    pub dialog_title: &'static str,
    pub dialog_yes_text: &'static str,
    pub dialog_no_text: &'static str,
    pub dialog_action: FileChooserAction,
    pub show_path: bool,
    pub spacing: i32,
    pub on_selected: Callback<Option<PathBuf>>,
    path: Option<PathBuf>,
}

impl Default for FileChooser {
    fn default() -> Self {
        Self {
            label: None,
            button_label: String::from("..."),
            dialog_title: "Select File",
            dialog_yes_text: "_Select",
            dialog_no_text: "_Cancel",
            dialog_action: FileChooserAction::Open,
            show_path: false,
            spacing: 10,
            on_selected: Callback::default(),
            path: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileChooserMessage {
    SetPath { path: Option<PathBuf> },
}

impl Component for FileChooser {
    type Message = FileChooserMessage;
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
            FileChooserMessage::SetPath { path } => {
                self.path = path.clone();
                self.on_selected.send(path);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        let dialog_title = self.dialog_title.clone();
        let dialog_yes_text = self.dialog_yes_text.clone();
        let dialog_no_text = self.dialog_no_text.clone();
        let dialog_action = self.dialog_action;
        gtk! {
            <Box spacing=self.spacing>
                {
                    gtk_if!(self.label.is_some() => {
                        <Label label=self.label.clone().unwrap() />
                    })
                }
                <Button Box::pack_type=PackType::End label=self.button_label.clone() on clicked=|_| {
                    let dialog = FileChooserNative::new(Some(&dialog_title), vgtk::current_window().as_ref(), dialog_action, Some(&dialog_yes_text), Some(&dialog_no_text));
                    dialog.run();
                    FileChooserMessage::SetPath {
                        path: dialog.get_filename(),
                    }
                }/>
            {
                gtk_if!(self.show_path && self.path.is_some() => {
                    <Label Box::pack_type=PackType::End label=self.path.clone().unwrap().display().to_string() ellipsize=EllipsizeMode::Middle />
                })
            }
            {
                gtk_if!(self.show_path && self.path.is_none() => {
                    <Label Box::pack_type=PackType::End label="" ellipsize=EllipsizeMode::Middle />
                })
            }
            </Box>
        }
    }
}
