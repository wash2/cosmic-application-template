use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use relm4::{gtk::{self, traits::GtkWindowExt}, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus};

use crate::fl;

pub struct Example {
    counter: u8,
}

#[derive(Debug)]
pub enum ExampleMsg {
    Increment,
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for Example {
    type Init = u8;
    type Input = ExampleMsg;
    type Output = ();
    // ExampleWidgets is generated by the macro
    type Widgets = ExampleWidgets;

   
    view! {
        gtk::Window {
            set_title: Some(&fl!("app-name")),
            set_decorated: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "+",
                    connect_clicked[sender] => move |_| {
                        sender.input(ExampleMsg::Increment);
                    }
                },

                gtk::Button {
                    set_label: "-",
                    connect_clicked[sender] => move |_| {
                        sender.input(ExampleMsg::Decrement);
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("{}", model.counter),
                    set_margin_all: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Example { counter };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ExampleMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            ExampleMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}