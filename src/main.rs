use leptos::*;
use leptos_router::*;
mod components;
use components::*;
fn main() {
    mount_to_body(|cx| {
        view! { cx,
             <Routes>
            <Route path="" view=|cx| view! { cx, <Home/> }/>
          </Routes>
        }
    })
}
