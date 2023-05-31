use leptos::*;

#[component]
pub fn Button(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx,
      <button on:click=move |_| {
          set_count.update(|n| *n += 1);
        }
      >
        "Click me: "
        {count}
      </button>
    }
}
