/* window.rs
 *
 * Copyright 2024 Arpan Biswas
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

// use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, prelude::ActionMapExtManual};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/firstApp/experimental/window.ui")]
    pub struct ExperimentalWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child(id = "carousel")]
        pub carousel: TemplateChild<adw::Carousel>,
        // #[template_child]
        // pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExperimentalWindow {
        const NAME: &'static str = "ExperimentalWindow";
        type Type = super::ExperimentalWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExperimentalWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
        }
    }
    impl WidgetImpl for ExperimentalWindow {}
    impl WindowImpl for ExperimentalWindow {}
    impl ApplicationWindowImpl for ExperimentalWindow {}
    impl AdwApplicationWindowImpl for ExperimentalWindow {}
}

glib::wrapper! {
    pub struct ExperimentalWindow(ObjectSubclass<imp::ExperimentalWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl ExperimentalWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn setup_gactions(&self) {
        let carousel_return_action = gio::ActionEntry::builder("return")
            .activate(move |app: &Self, _, _| app.carousel_return())
            .build();
        self.add_action_entries([carousel_return_action]);
    }

    fn carousel_return(&self) {
        let carousel = &self.imp().carousel;
        let nth_page = carousel.nth_page(0);
        
        carousel.scroll_to(&nth_page, true);
    }
}
