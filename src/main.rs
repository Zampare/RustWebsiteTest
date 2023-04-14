use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <form>
            <label>
                <input type="text" name="username"/>
            </label>
            <select name="lifts" id="lift select">
                <option value="Bench">{"Bench"}</option>
                <option value="Squat">{"Squat"}</option>
                <option value="Deadlift">{"Deadlift"}</option>
            </select>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

 