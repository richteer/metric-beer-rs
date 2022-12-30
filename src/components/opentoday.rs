use std::sync::Arc;
use std::cmp::Ordering;
use yew::prelude::*;
use chrono::prelude::*;

use crate::common::*;
use metric_beer_data::*;

fn format_upcoming(now: &DateTime<Local>, open: Option<&DateTime<Local>>, close: Option<&DateTime<Local>>) -> OpenStatus {
    match (open, close) {
        (Some(o), Some(c)) => {
            match (now.cmp(o), now.cmp(c)) {
                (Ordering::Less, _) => OpenStatus::OpenLater(format!("{} min", (*o - *now).num_minutes())), // TODO: consider an display option for hours/mins?
                (_, Ordering::Less) | (_, Ordering::Equal)  => OpenStatus::Open,
                (_, Ordering::Greater) => OpenStatus::Closed,
            }
        }
        (None, None) => OpenStatus::Closed,
        (Some(_), None) | (None, Some(_)) => {
            log::error!("data is probably wrong, open and close aren't both -1 (closed). open = {open:?}, close = {close:?}");
            OpenStatus::Error
        },
    }
}


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
            e.name.to_string(),
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
                            <td style={format!("color: {}", match &e.3 {
                                OpenStatus::Open => "green",
                                OpenStatus::Closed => "red",
                                OpenStatus::OpenLater(_) => "yellow",
                                OpenStatus::Error => "gray",
                            })}>{e.3}</td>
                        </tr>
                    }
                })
            }
            </tbody>
        </table>
    }
}