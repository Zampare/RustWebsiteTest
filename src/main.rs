use std::path::Iter;

use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use gloo_net::http::{Request, FormData};
use gloo_console::log;
const SERVERHOST:&str = "192.168.1.38:8080";

#[derive(Serialize, Default, Debug)]
struct NewLift{
    pub lift: String,
    pub weight:i32,
    pub reps:i32,
    pub rpe:i32,
    pub time:chrono::DateTime<chrono::Utc>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Lift{
    id:i32,
    lift: String,
    weight:i32,
    reps:i32,
    rpe:i32,
    time:chrono::DateTime<chrono::Utc>
}

#[function_component(App)]
fn app() -> Html {
    let lifts = use_state(|| vec![]);
    let submitlift = use_state(|| NewLift { lift: "Bench".to_string(), ..Default::default()});
    {
        let lifts = lifts.clone();
        use_effect_with_deps(move |_|{
            wasm_bindgen_futures::spawn_local(async move{
                let fetchedlifts = get_lifts().await;
                lifts.set(fetchedlifts);
            })
        }, ());
    }

    
    let submitlift2 = submitlift.clone();
    let lifts2 = lifts.clone();
    let onsubmithandle = move |e:SubmitEvent| {
        e.prevent_default();
        
        
        let newlift = NewLift{
            lift: submitlift2.lift.clone(),
            weight: submitlift2.weight,
            reps: submitlift2.reps,
            rpe: submitlift2.rpe,
            time: chrono::offset::Utc::now()
        };
        log!(format!("{:?}", newlift));
        let lifts3 = lifts2.clone();
        wasm_bindgen_futures::spawn_local(async move{
            write_lift(&newlift).await;
            let fetchedlifts = get_lifts().await;
            lifts3.set(fetchedlifts);
        })
        

    };
    

    let submitlift3 = submitlift.clone();
    let onliftchangehandle = move|e:Event|{
        submitlift3.set(
            NewLift{
            lift: e.target().unwrap().dyn_ref::<HtmlSelectElement>().unwrap().value(),
            weight: submitlift3.weight,
            reps: submitlift3.reps,
            rpe: submitlift3.rpe,
            time: submitlift3.time
        });
    };

    let submitlift4 = submitlift.clone();
    let onweightchangehandle = move|e:Event|{
        submitlift4.set(
            NewLift{
            lift: submitlift4.lift.clone(),
            weight: e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value().parse().unwrap(),
            reps: submitlift4.reps,
            rpe: submitlift4.rpe,
            time: submitlift4.time
        });
    };

    let submitlift5 = submitlift.clone();
    let onrepschangehandle = move|e:Event|{
        submitlift5.set(
            NewLift{
            lift: submitlift5.lift.clone(),
            weight: submitlift5.weight,
            reps: e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value().parse().unwrap(),
            rpe: submitlift5.rpe,
            time: submitlift5.time
        });
    };
    
    let submitlift6 = submitlift.clone();
    let onrpechangehandle = move|e:Event|{
        submitlift6.set(
            NewLift{
            lift: submitlift6.lift.clone(),
            weight: submitlift6.weight,
            reps: submitlift6.reps,
            rpe: e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value().parse().unwrap(),
            time: submitlift6.time
        });
    };





    let liftsstring = lifts.iter().map(|lift| html! {
        <p key={lift.id}>{format!("id:{} lift:{} weight:{} reps:{} rpe:{} time:{}", lift.id, lift.lift, lift.weight, lift.reps, lift.rpe, lift.time)}</p>
    }).collect::<Html>();
    html! {
        <>
            <form onsubmit={onsubmithandle}>   
                <select name="lifts" id="lift select" onchange={onliftchangehandle}>
                    <option selected=true value="Bench">{"Bench"}</option>
                    <option value="Squat">{"Squat"}</option>
                    <option value="Deadlift">{"Deadlift"}</option>
                </select>
                <br/>
                <label>
                    {"weight: "}
                    <input type="integer" name="weight" onchange={onweightchangehandle}/>
                </label>
                <br/>
                <label>
                    {"reps: "}
                    <input type="integer" name="reps" onchange={onrepschangehandle}/>
                </label>
                <br/>
                <label>
                    {"rpe: "}
                    <input type="integer" name="rpe" onchange={onrpechangehandle}/>
                </label>
                <br/>
                <button>
                    {"Submit"}

                </button>
            </form>

            <div>
                {liftsstring}
            </div>
        </>
    }
}

async fn get_lifts() -> Vec<Lift> {
    let url = "/api/workout/lifts";

    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn write_lift(form_data: &NewLift) -> Lift {
    let url = "/api/workout/lifts";
    let json = serde_json::to_string(form_data).unwrap();

    Request::post(&url)
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

fn main() {
    yew::Renderer::<App>::new().render();
}

 