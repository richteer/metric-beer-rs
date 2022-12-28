use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::prelude::*;

mod components;
use components::*;

mod common;

mod data;
use data::*;


// TODO: Actually error handle, don't rely on unwraps
async fn get_data() -> Result<BreweryData, ()> {
    Ok(Request::get("https://richteer.github.io/metric-beer-data/beer.json")
        .send().await.unwrap()
        .json::<BreweryData>()
        .await.unwrap())
}


#[function_component]
fn App() -> Html {
    let data = use_async(async move { get_data().await });
    let date = use_state(|| chrono::Local::now());
    let ampm = use_bool_toggle(false);

    // TODO: State this, let the user set weekend type
    let dayorder: Vec<String> = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"].iter()
        .map(|e| e.to_string()).collect();

    // TODO: probably put BreweryData into some kind of shareable Box to reduce clones

    {
        let data = data.clone();
        use_mount(move || { data.run() });
    }
    {
        let date = date.clone();
        use_interval(move || {
            // TODO: is getting current time every second expensive?
            date.set(chrono::Local::now());
        }, 1000);
    }

    let onampmclick = {
        let ampm = ampm.clone();
        Callback::from(move |_| ampm.toggle())
    };

    html! {
        <body class="container">
        <div>
        {
            if let Some(data) = &data.data {
                html! {
                    <>
                    <div>
                        <OpenToday data={data.clone()} ampm={*ampm} date={*date}/>
                    </div>
                    <div>
                        <HourTable data={data.clone()} ampm={*ampm} date={*date} dayorder={dayorder.clone()}/>
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
        </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}