use yew::prelude::*;
use crate::data::loaders::load_links;

#[function_component(App)]
pub fn app() -> Html {
    load_links(20i64, 0i64);
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class={classes!("container")}>
            <div class="row">
                <div class="col-2">
                    <button {onclick}>{ "+1" }</button>
                    <p>{ *counter }</p>
                </div>
            </div>
        </div>
    }
}
