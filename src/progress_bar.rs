use vgtk::lib::gtk::prelude::*;
use vgtk::lib::gtk::ProgressBar as GtkProgressBar;
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct ProgressBar {
    pub progress_fraction: f64,
    pub progress_text: String,
    pub show_text: bool,
    pub widget_name: String,
}

#[derive(Clone, Debug)]
pub enum ProgressBarMessage {}

impl Component for ProgressBar {
    type Message = ProgressBarMessage;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, _msg: Self::Message) -> UpdateAction<Self> {
        UpdateAction::None
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <GtkProgressBar fraction=self.progress_fraction text=self.progress_text.clone()
                show_text=self.show_text widget_name=self.widget_name.clone() />
        }
    }
}
