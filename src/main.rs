use std::sync::Arc;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::prelude::*;

mod components;
use components::*;

mod common;

//mod data;
//use data::*;
use metric_beer_data::*;


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

    // Yes, this is probably unnecessary.
    let dayorder = use_toggle(
        Arc::new(vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"].iter()
            .map(|e| e.to_string()).collect::<Vec<String>>()),
        Arc::new(vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"].iter()
            .map(|e| e.to_string()).collect())
    );

    let ampm_storage = use_local_storage::<bool>("ampm".to_string());
    let monfirst_storage = use_local_storage::<bool>("monfirst".to_string());

    let ampm = use_bool_toggle(ampm_storage.unwrap_or_else(|| false));

    {
        let data = data.clone();
        let dayorder = dayorder.clone();
        let monfirst_storage = monfirst_storage.clone();
        use_mount(move || {
            match *monfirst_storage {
                Some(true) => dayorder.set_right(),
                Some(false) => dayorder.set_left(),
                None => dayorder.set_left(),
            };
            data.run()
        });
    }
    {
        let date = date.clone();
        use_interval(move || {
            date.set(chrono::Local::now());
        }, 1000);
    }

    let onampmclick = {
        let ampm = ampm.clone();
        let ampm_storage = ampm_storage.clone();
        Callback::from(move |_| {
            ampm_storage.set(!*ampm);
            ampm.toggle();
        })
    };
    let onmonfirstclick = {
        let dayorder = dayorder.clone();
        let monfirst_storage = monfirst_storage.clone();
        Callback::from(move |_| {
            monfirst_storage.set(!monfirst_storage.unwrap_or_default());
            dayorder.toggle()
        })
    };

    html! {
        <body class="container">
        <div>
        {
            if let Some(data) = &data.data {
                html! {
                    <>
                    <div>
                        {format!("{} hours", date.format("%A"))} // TODO: Probably style this better
                        <OpenToday data={data} ampm={*ampm} date={*date}/>
                    </div>
                    <div>
                        {"Schedule"}
                        <figure>
                            <HourTable data={data} ampm={*ampm} date={*date} dayorder={(*dayorder).clone()}/>
                        </figure>
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
                <input type="checkbox" name="ampm" onclick={onampmclick} checked={*ampm}/>
                {"Use AM/PM"}
            </label>
            <label for="weekend">
                <input type="checkbox" name="weekend" onclick={onmonfirstclick} checked={monfirst_storage.unwrap_or_default()}/>
                {"Monday First"}
            </label>
        </div>
        </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}