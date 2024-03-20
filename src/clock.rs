use chrono::Local;

#[yew::prelude::function_component(Clock)]
pub fn clock() -> yew::prelude::Html {
    let time = yew::prelude::use_state(|| Local::now().format("%H:%M:%S").to_string());
    let time_clone = time.clone();

    yew::prelude::use_effect(move || {
        let handle = gloo_timers::callback::Interval::new(1_000, move || {
            time.set(Local::now().format("%H:%M:%S").to_string());
        });
        handle.forget();
        || {}
    });

    let x = format!("{:?}", *time_clone);

    yew::prelude::html! {
        <div class="clock">
            {
            x.get(1..x.len().checked_sub(1).unwrap_or(1)).unwrap_or_default().to_owned()}
        </div>
    }
}
