use dioxus::prelude::*;
use std::num::IntErrorKind;

const MAIN_CSS: Asset = asset!("/assets/main.css");

const MAX_MAP_SIZE: u8 = 32;
const MAX_CELL_SIZE: u8 = 128;

const BASE_MAP_SIZE: u8 = 16;
const BASE_CELL_SIZE: u8 = 32;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut map_size = use_signal(|| BASE_MAP_SIZE);
    let mut cell_size = use_signal(|| BASE_CELL_SIZE);

    use_context_provider(|| cell_size);

    let handle_map_size_input = move |ev: Event<FormData>| {
        match ev.value().parse::<u8>() {
            Ok(value) => {
                if value < MAX_MAP_SIZE {
                    map_size.set(value);
                } else {
                    map_size.set(BASE_MAP_SIZE);
                }
            }
            Err(e) => {
                match e.kind() {
                    IntErrorKind::Zero => map_size.set(1),
                    _ => map_size.set(BASE_MAP_SIZE),
                };
            }
        };
    };

    let handle_cell_size_input = move |ev: Event<FormData>| {
        match ev.value().parse::<u8>() {
            Ok(value) => {
                if value < MAX_CELL_SIZE {
                    cell_size.set(value);
                } else {
                    cell_size.set(BASE_CELL_SIZE);
                }
            }
            Err(e) => {
                match e.kind() {
                    IntErrorKind::Zero => cell_size.set(1),
                    _ => map_size.set(BASE_CELL_SIZE),
                };
            }
        };
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            id: "inputs",
            div {
                id: "size_input",
                h1 { "Размер клетки" }
                input {
                    value: "{cell_size}",
                    oninput: handle_cell_size_input
                }
            }
            div {
                id: "size_input",
                h1 { "Размер карты" }
                input {
                    value: "{map_size}",
                    oninput: handle_map_size_input
                }
            }
        }
        Map {
            map_size
        }
    }
}

#[component]
pub fn Map(map_size: ReadOnlySignal<u8>) -> Element {
    rsx! {
        for _x in 0..map_size() {
            div {
                for _y in 0..map_size() {
                    MapCell {  }
                }
            }
        }
    }
}

#[component]
pub fn MapCell() -> Element {
    let mut filled = use_signal(|| false);
    let id = use_memo(move || if filled() { "filled_cell" } else { "cell" });

    let cell_size = use_context::<Signal<u8>>();

    let handle_click = move |ev: MouseEvent| {
        ev.prevent_default();
        if filled() {
            filled.set(false);
        } else {
            filled.set(true);
        }
    };

    rsx! {
        button {
            id: "{id}",
            style: "width: {cell_size()}px; height: {cell_size()}px",
            onclick: handle_click
        }
    }
}
