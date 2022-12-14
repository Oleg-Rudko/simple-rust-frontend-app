use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    let plus = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };

    let minus = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value - 1,
            })
        })
    };

    html! {
        <div>
            <button onclick={plus}>{ "+1" }</button>
            <p>{ state.value }</p>
            <button onclick={minus}>{ "-1" }</button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
