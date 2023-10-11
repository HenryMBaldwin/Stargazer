use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            style: "display: flex;",

            div {
                style: "background-color: black; width: 200px; height: 100vh;",
            },

            div {
                style: "flex-grow: 1; background-color: grey;",
            },
        }
    })
}
