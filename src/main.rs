use yew::prelude::*;

struct Model{
    value: i32
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    html! {
        <form>
            <TextInput name="username">
            <label for="lifts">Choose a lift:</label>
            <select name="lifts" id="lift select">
                <option value="Bench">Bench</option>
                <option value="Squat">Squat</option>
                <option value="Deadlift">Deadlift</option>
            </select>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

 