use std::sync::Arc;
use yew::prelude::*;
use crate::common::*;

use crate::data::*;

#[derive(Properties, PartialEq)]
pub struct OpenTodayProps {
    pub data: Arc<Box<BreweryData>>,
    pub ampm: bool,
    pub date: chrono::DateTime<chrono::Local>,
}


#[function_component(OpenToday)]
pub fn open_today(props: &OpenTodayProps) -> Html {
    let brewdata = &props.data;
    let today = props.date.format("%A").to_string();

    // Convert the list of breweries into rows of data for the table
    let map_func = |e: &Brewery| {
        let hours = e.hours.get(&today).unwrap();
        let open = hours.open.to_today(&props.date);
        let close = hours.close.to_today(&props.date);
        (
            e.name.to_string(), // Probably unnecessary clone here, but makes the code below less of an indented mess.
            format_time(open, props.ampm),
            format_time(close, props.ampm),
            format_upcoming(&props.date, open.as_ref(), close.as_ref()),
        )
    };

    html! {
        <table role="grid">
            <thead>
            <tr>
                <td>{"Brewery"}</td>
                <td>{"Open"}</td>
                <td>{"Close"}</td>
                <td>{"Status"}</td>
            </tr>
            </thead>
            <tbody>
            {
                for brewdata.breweries.iter()
                    .map(map_func)
                    .map(|e| {
                    html! {
                        <tr>
                            <td>{e.0}</td>
                            <td>{e.1}</td>
                            <td>{e.2}</td>
                            <td>{e.3}</td>
                        </tr>
                    }
                })
            }
            </tbody>
        </table>
    }
}