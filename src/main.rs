use std::sync::Arc;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::prelude::*;

mod components;
use components::*;

mod common;

mod data;
use data::*;


// TODO: Actually error handle, don't rely on unwraps
async fn get_data() -> Result<Arc<Box<BreweryData>>, ()> {
    Ok(Arc::new(Box::new(Request::get("https://richteer.github.io/metric-beer-data/beer.json")
        .send().await.unwrap()
        .json::<BreweryData>()
        .await.unwrap())))
}


#[function_component]
fn App() -> Html {
    let data = use_async(async move { get_data().await });
    let date = use_state(|| chrono::Local::now());
    let ampm = use_bool_toggle(false);

    // Yes, this is probably unnecessary.
    let dayorder = use_toggle(
        Arc::new(vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"].iter()
            .map(|e| e.to_string()).collect::<Vec<String>>()),
        Arc::new(vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"].iter()
            .map(|e| e.to_string()).collect())
    );

    {
        let data = data.clone();
        use_mount(move || { data.run() });
    }
    {
        let date = date.clone();
        use_interval(move || {
            date.set(chrono::Local::now());
        }, 1000);
    }

    let onampmclick = {
        let ampm = ampm.clone();
        Callback::from(move |_| ampm.toggle())
    };
    let onweekendclick = {
        let dayorder = dayorder.clone();
        Callback::from(move |_| dayorder.toggle())
    };

    html! {
        <body class="container">
        <div>
        {
            if let Some(data) = &data.data {
                html! {
                    <>
                    <div>
                        <OpenToday data={data} ampm={*ampm} date={*date}/>
                    </div>
                    <div>
                        <HourTable data={data} ampm={*ampm} date={*date} dayorder={(*dayorder).clone()}/>
                    </div>
                    </>
                }
            } else {
                html! {

                }
            }
        }
        </div>
        <div>
            <label for="ampm">
                <input type="checkbox" name="ampm" onclick={onampmclick}/>
                {"Use AM/PM"}
            </label>
            <label for="weekend">
                <input type="checkbox" name="weekend" onclick={onweekendclick}/>
                {"Weekend at End"}
            </label>
        </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}