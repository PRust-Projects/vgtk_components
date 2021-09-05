use vgtk::lib::glib::{Type, Value};
use vgtk::lib::gtk::prelude::*;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Callback, Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub struct Dropdown {
    // A list of items to be provided by parent component
    pub items: Vec<String>,
    pub label: Option<String>,
    pub on_select: Callback<String>,
    pub orientation: Orientation,
    pub spacing: i32,
    pub full_width: bool,
    // Internal list of items stored as a ListStore (upcasted to TreeModel)
    dropdown_items: TreeModel,
}

impl Default for Dropdown {
    fn default() -> Self {
        Self {
            items: vec![],
            label: None,
            on_select: Callback::default(),
            orientation: Orientation::Horizontal,
            spacing: 10,
            full_width: false,
            dropdown_items: ListStore::new(&[Type::String]).upcast::<TreeModel>(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DropdownMessage {
    SetValue { value: String },
}

impl Component for Dropdown {
    type Message = DropdownMessage;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        let mut component = props;

        let dropdown_items = ListStore::new(&[Type::String]);
        for item in &component.items {
            let iter = dropdown_items.append();
            dropdown_items.set_value(&iter, 0, &Value::from(&item));
        }
        component.dropdown_items = dropdown_items.upcast::<TreeModel>();

        component
    }

    fn change(&mut self, props: Self::Properties) -> UpdateAction<Self> {
        *self = props;

        // Populate the dropdown with the given items
        let dropdown_items = ListStore::new(&[Type::String]);
        for item in &self.items {
            let iter = dropdown_items.append();
            dropdown_items.set_value(&iter, 0, &Value::from(&item));
        }
        self.dropdown_items = dropdown_items.upcast::<TreeModel>();

        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            DropdownMessage::SetValue { value } => {
                self.on_select.send(value);
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=self.orientation spacing=self.spacing>
                {
                    gtk_if!(self.label.is_some() => {
                        <Label label=self.label.as_ref().unwrap().to_string() halign=Align::Start />
                    })
                }
            {
                if self.full_width {
                    gtk! {
                        <ComboBoxText model=Some(self.dropdown_items.clone()) Box::pack_type=PackType::End
                            on changed=|entry| {
                                DropdownMessage::SetValue {
                                    value: entry.get_active_text().unwrap().to_string(),
                                }
                            } />
                    }
                } else {
                    gtk! {
                        <ComboBoxText model=Some(self.dropdown_items.clone()) on changed=|entry| {
                            DropdownMessage::SetValue {
                                value: entry.get_active_text().unwrap().to_string(),
                            }
                        } />
                    }
                }
            }
            </Box>
        }
    }
}
