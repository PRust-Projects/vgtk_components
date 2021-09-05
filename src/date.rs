use std::fmt;

use chrono::{Datelike, Utc};
use lazy_static::lazy_static;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Callback, Component, UpdateAction, VNode};

use crate::dropdown::Dropdown;

lazy_static! {
    static ref CURRENT_YEAR: usize = {
        let now = Utc::now();
        let (_, year) = now.year_ce();
        year as usize
    };
    static ref MONTHS: Vec<String> =
        vec!["Jan", "Feb", "Mar", "Apr", "May", "June", "July", "Aug", "Sept", "Oct", "Nov", "Dec"]
            .iter()
            .map(ToString::to_string)
            .collect();
    static ref DAYS: Vec<String> = vec![
        "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
        "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
        "31",
    ]
    .iter()
    .map(ToString::to_string)
    .collect();
}

#[derive(Clone, Default, Debug)]
pub struct Date {
    pub month: String,
    pub day: String,
    pub year: String,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.month.is_empty() || self.day.is_empty() || self.year.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{} {}, {}", self.month.to_string(), self.day, self.year)
        }
    }
}

#[derive(Clone, Debug)]
pub struct DateInput {
    pub label: Option<String>,
    pub date: Date,
    pub editable: bool,
    pub min_year: usize,
    pub max_year: usize,
    pub orientation: Orientation,
    pub spacing: i32,
    pub full_width: bool,
    pub element_spacing: i32,
    pub year_label: String,
    pub month_label: String,
    pub day_label: String,
    pub widget_name: String,
    pub year_widget: String,
    pub month_widget: String,
    pub day_widget: String,
    pub label_widget: String,
    pub on_update: Callback<Date>,
    years: Vec<String>,
}

impl Default for DateInput {
    fn default() -> Self {
        Self {
            label: None,
            date: Date::default(),
            editable: true,
            min_year: *CURRENT_YEAR - 120,
            max_year: *CURRENT_YEAR,
            orientation: Orientation::Horizontal,
            spacing: 10,
            full_width: false,
            element_spacing: 10,
            year_label: String::from("Year:"),
            month_label: String::from("Month:"),
            day_label: String::from("Day:"),
            widget_name: String::from("date"),
            year_widget: String::from("year"),
            month_widget: String::from("month"),
            day_widget: String::from("day"),
            label_widget: String::from("label"),
            on_update: Callback::default(),
            years: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum DateInputMessage {
    SetMonth { month: String },
    SetDay { day: String },
    SetYear { year: String },
}

impl Component for DateInput {
    type Message = DateInputMessage;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        let mut component = props;

        let years = (component.min_year..=component.max_year)
            .rev()
            .collect::<Vec<usize>>()
            .iter()
            .map(ToString::to_string)
            .collect();
        component.years = years;

        component
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        *self = props;

        // Populate the years
        let years = (self.min_year..=self.max_year)
            .rev()
            .collect::<Vec<usize>>()
            .iter()
            .map(ToString::to_string)
            .collect();
        self.years = years;

        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            DateInputMessage::SetMonth { month } => {
                self.date.month = month;
                self.on_update.send(self.date.clone());
                UpdateAction::None
            }
            DateInputMessage::SetDay { day } => {
                self.date.day = day;
                self.on_update.send(self.date.clone());
                UpdateAction::None
            }
            DateInputMessage::SetYear { year } => {
                self.date.year = year;
                self.on_update.send(self.date.clone());
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=Orientation::Horizontal spacing=self.spacing widget_name=self.widget_name.clone()>
            {
                gtk_if!(self.label.is_some() => {
                    <Label label=self.label.as_ref().unwrap().to_string() widget_name=self.label_widget.clone()
                        halign=Align::Start />
                })
            }
            {
                let pack_type = if self.full_width { PackType::End } else { PackType::Start };
                if self.editable {
                    gtk! {
                        <Box orientation=Orientation::Horizontal spacing=self.spacing widget_name=self.widget_name.clone() Box::pack_type=pack_type>
                            <Box orientation=self.orientation spacing=self.element_spacing widget_name=self.month_widget.clone()>
                                <Label label=self.month_label.clone() halign=Align::Start />
                                <@Dropdown items=MONTHS.clone() on_select=|month| DateInputMessage::SetMonth { month } />
                            </Box>
                            <Box orientation=self.orientation spacing=self.element_spacing widget_name=self.day_widget.clone()>
                                <Label label=self.day_label.clone() halign=Align::Start />
                                <@Dropdown items=DAYS.clone() on_select=|day| DateInputMessage::SetDay { day } />
                            </Box>
                            <Box orientation=self.orientation spacing=self.element_spacing widget_name=self.year_widget.clone()>
                                <Label label=self.year_label.clone() halign=Align::Start />
                                <@Dropdown items=self.years.clone() on_select=|year| DateInputMessage::SetYear { year } />
                            </Box>
                        </Box>
                    }
                } else {
                    gtk! {
                        <Box orientation=Orientation::Horizontal spacing=10 Box::pack_type=pack_type>
                            <Label label=self.date.to_string() widget_name=self.widget_name.clone() />
                        </Box>
                    }
                }
            }
            </Box>
        }
    }
}
