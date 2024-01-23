use css_in_rs::{make_styles, Classes};
// toggle_hidden!
use dioxus::prelude::*;
use mui_dioxus::theme::MuiTheme;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
// toggle_hidden!

make_styles! {
    (theme: MuiTheme) -> CssClasses {
        container {
            display: "flex",
            justify_content: "center",
            padding: "16px",
        },
        ".container > .item" {
            background_color: "#e2deff",
            box_shadow: &theme.shadows[3],
            cursor: "pointer",
            padding: "16px",
            user_select: "none",
            animation: "shake 0.25s",
            animation_iteration_count: "1",
        },
        "@keyframes shake" {
            "0%" { transform: "translate(1px, 1px) rotate(0deg)", },
            "10%" { transform: "translate(-1px, -2px) rotate(-1deg)", },
            "20%" { transform: "translate(-3px, 0px) rotate(1deg)", },
            "30%" { transform: "translate(3px, 2px) rotate(0deg)", },
            "40%" { transform: "translate(1px, -1px) rotate(1deg)", },
            "50%" { transform: "translate(-1px, 2px) rotate(-1deg)", },
            "60%" { transform: "translate(-3px, 1px) rotate(0deg)", },
            "70%" { transform: "translate(3px, 1px) rotate(-1deg)", },
            "80%" { transform: "translate(-1px, -1px) rotate(1deg)", },
            "90%" { transform: "translate(1px, 2px) rotate(0deg)", },
            "100%" { transform: "translate(1px, -2px) rotate(-1deg)", },
          }
    }
}

fn shake(elem: &web_sys::Element, classes: &CssClasses) {
    elem.set_class_name("");
    elem.client_width();
    elem.set_class_name(&classes.item);
}

#[allow(non_snake_case)]
pub fn Sample(cx: Scope) -> Element {
    let classes = CssClasses::use_style(cx);

    let elem_ref = use_ref(cx, || None::<web_sys::Element>);

    render!(
        div {
            class: &classes.container as &str,
            div {
                class: &classes.item as &str,
                onclick: move |_| shake(elem_ref.read().as_ref().unwrap(), classes),
                onmounted: move |ev| {
                    let elem = ev.inner().get_raw_element().unwrap();
                    let elem = elem.downcast_ref::<web_sys::Element>().unwrap();
                    elem_ref.set(Some(elem.clone()));
                },
                "Click me"
            }
        }
    )
}
// toggle_hidden!

fn App(cx: Scope) -> Element {
    css_in_rs::use_style_provider_quickstart(cx, MuiTheme::default);
    CssClasses::use_style(cx);

    render!(Sample {})
}
