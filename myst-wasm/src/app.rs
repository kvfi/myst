use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
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
