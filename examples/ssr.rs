#![allow(non_upper_case_globals)]

use dioxus::prelude::*;
use dioxus::ssr;

fn main() {
    let mut vdom = VirtualDom::new(App);
    vdom.rebuild_in_place().expect("Rebuilding failed");
    println!("{}", ssr::render_vdom(&vdom, |c| c));
}

static App: FC<()> = |cx| {
    cx.render(rsx!(
        div {
            h1 { "Title" }
            p { "Body" }
        }
    ))
};
