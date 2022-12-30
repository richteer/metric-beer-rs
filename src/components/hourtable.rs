use std::sync::Arc;
use yew::prelude::*;

use metric_beer_data::*;
use crate::common::*;

#[derive(Properties, PartialEq)]
pub struct HourTableProps {
    pub data: Arc<Box<BreweryData>>,
    pub ampm: bool,
    pub date: chrono::DateTime<chrono::Local>,
    pub dayorder: Arc<Vec<String>>,
}

#[function_component(HourTable)]
pub fn hour_table(props: &HourTableProps) -> Html {
    html! {
        <table role="grid">
            <thead>
            <tr>
                <td>{"Brewery"}</td>
                {
                    for props.dayorder.iter().map(|e| {
                        html! { <td>{e}</td> }
                    })
                }
            </tr>
            </thead>
            <tbody>
            {
                for props.data.breweries.iter().map(|brew| {

                    html! {
                        <tr>
                            <td>{&brew.name}</td>
                            {
                                for props.dayorder.iter().map(|day| {
                                    let Hours { open, close } = brew.hours.get(day).unwrap();
                                    html! { <td>{format_hours(open.to_today(&props.date), close.to_today(&props.date), props.ampm)}</td> }
                                })
                            }
                        </tr>
                    }
                })
            }
            </tbody>
        </table>
    }
}