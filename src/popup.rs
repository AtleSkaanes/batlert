use std::{cell::RefCell, rc::Rc};

use crate::cli;
use gtk4::{glib, prelude::*, Application, ApplicationWindow, Window};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

pub fn create_popup(cli_args: &cli::CliArgs) -> Option<Choice> {
    let application = Application::builder()
        .application_id("com.github.AtleSkaanes.batlert")
        .build();

    let answer_buf = Rc::new(RefCell::new(None));
    application.connect_activate(build_ui_curry(answer_buf.clone()));
    application.run_with_args::<String>(cli_args.gtk_args.as_ref().unwrap_or(&vec![]));

    let answer = answer_buf.borrow().clone();
    answer
}

fn build_ui_curry(answer_buf: Rc<RefCell<Option<Choice>>>) -> impl Fn(&Application) + 'static {
    move |application: &Application| {
        let window = ApplicationWindow::builder()
            .application(application)
            .decorated(false)
            .visible(false)
            .build();

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_namespace("batlert");
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        window.set_exclusive_zone(-1); // makes sure that it is above waybar...

        window.error_bell();
        glib::MainContext::default().spawn_local(dialog(window.clone(), answer_buf.clone()));

        window.connect_close_request(move |closing_window| {
            if let Some(application) = closing_window.application() {
                application.remove_window(closing_window);
            }
            glib::Propagation::Proceed
        });
    }
}

async fn dialog<W: IsA<Window>>(window: W, answer_buf: Rc<RefCell<Option<Choice>>>) {
    let question_dialog = gtk4::AlertDialog::builder()
        .modal(true)
        .message("Critical battery level")
        .detail("Computer will soon run out of battery")
        .buttons(["Sleep", "Ok", "Shutdown"])
        .default_button(1)
        .build();

    let answer = question_dialog.choose_future(Some(&window)).await;

    *answer_buf.borrow_mut() = match answer {
        Ok(idx) => Some(Choice::try_from(idx).unwrap()),
        Err(_) => None,
    };
    window.destroy();
}

#[derive(Clone, Debug)]
pub enum Choice {
    Shutdown,
    Ok,
    Suspend,
}

impl TryFrom<i32> for Choice {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Suspend),
            1 => Ok(Self::Ok),
            2 => Ok(Self::Shutdown),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Suspend => write!(f, "Suspend"),
            Self::Ok => write!(f, "Ok"),
            Self::Shutdown => write!(f, "Shutdown"),
        }
    }
}
