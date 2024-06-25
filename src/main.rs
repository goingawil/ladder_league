use yew::prelude::*;
use serde::Deserialize;

const PARTICIPANTS: u32 = 16;

const JSON_DATA: &'static str = r#"
    {
        "weeks": [
            {
                "week": "6/23/24",
                "results": [
                    {
                        "winner1": "Valerie L.",
                        "winner2": "Isaac P.",
                        "loser1": "Adam W.",
                        "loser2": "Chris A."
                    },
                    {
                        "winner1": "Veronica K.",
                        "winner2": "Bharat G.",
                        "loser1": "Fei K.",
                        "loser2": "Luke B."
                    },
                    {
                        "winner1": "Ben J.",
                        "winner2": "Daniel P.",
                        "loser1": "Susan A.",
                        "loser2": "Sarah P."
                    },
                    {
                        "winner1": "Eric Y.",
                        "winner2": "Tim G.",
                        "loser1": "Mike M.",
                        "loser2": "Julian P."
                    }
                ]
            },
            {
                "week": "6/16/24",
                "results": [
                    {
                        "winner1": "Adam W.",
                        "winner2": "Chris A.",
                        "loser1": "Fei K.",
                        "loser2": "Luke B."
                    },
                    {
                        "winner1": "Isaac P.",
                        "winner2": "Valerie L.",
                        "loser1": "Susan A.",
                        "loser2": "Sarah P."
                    },
                    {
                        "winner1": "Veronica K.",
                        "winner2": "Bharat G.",
                        "loser1": "Mike M.",
                        "loser2": "Julian P."
                    },
                    {
                        "winner1": "Ben J.",
                        "winner2": "Daniel P.",
                        "loser1": "Eric Y.",
                        "loser2": "Tim G."
                    }
                ]
            }
        ]
    }

"#;

#[derive(Deserialize)]
struct WeekList {
    weeks: Vec<Week>,
}

#[derive(Deserialize)]
struct Week {
    week: String,
    results: Vec<Rung>
}

#[derive(Deserialize)]
struct Rung {
    winner1: String,
    winner2: String,
    loser1: String,
    loser2: String,
}

#[function_component(App)]
fn app() -> Html {

    //index into WeekList
    let week_state = use_state(|| 0);

    let week_list: WeekList = serde_json::from_str(JSON_DATA).unwrap();
    let weeks: Vec<Week> = week_list.weeks;

    for week in weeks {
        
    }


    html! {
        <h1 class={classes!("bg-red-300")}>{"Hello, world!"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
