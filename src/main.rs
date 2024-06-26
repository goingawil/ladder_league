use yew::prelude::*;
use serde::Deserialize;

const PARTICIPANTS: u32 = 16;

const START_WEEK: &'static str = "6/9/24";

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

#[derive(Deserialize, Clone, PartialEq)]
struct WeekList {
    weeks: Vec<Week>,
}

#[derive(Deserialize, Clone, PartialEq)]
struct Week {
    week: String,
    results: Vec<Rung>
}

#[derive(Deserialize, Clone, PartialEq)]
struct Rung {
    winner1: String,
    winner2: String,
    loser1: String,
    loser2: String,
}
// ladder is updated every Sunday
// let's say ladder is updated Sunday, 6/30
// the site will then show the ladder after incorporating the results from week 6/23
// so the week for a given results will actually be a week ahead, i.e. *these are the results we want to show starting the given week*
// Ladder for 6/30/24
// {ladder after results from week of 6/23/24}
#[function_component(App)]
fn app() -> Html {

    //index into WeekList
    let _week_state = use_state(|| 0);

    let ladder_league_results: WeekList = serde_json::from_str(JSON_DATA).unwrap();
    let weeks: Vec<Week> = ladder_league_results.weeks.clone();

    let ladder_league_start: Week = Week {
        week: String::from(START_WEEK),
        results: weeks[0].results.clone(),
    };

    //vector of ladder leagues, representing history of the league
    let mut ladder_league_history: WeekList = WeekList {
        weeks: Vec::<Week>::new()
    };

    ladder_league_history.weeks.push(ladder_league_start);

    // going through last week's results to calculate this week's display
    for (i, last_week_results) in ladder_league_results.weeks.iter().enumerate() {
        let this_week_str = &last_week_results.week;

        let mut this_week_display: Week = Week {
            week: this_week_str.clone(),
            results: Vec::<Rung>::with_capacity((PARTICIPANTS/4) as usize),
        };

        let last_week_display = ladder_league_history.weeks[i].clone();
        // going through last week's rungs to calculate this week's rungs
        for (j, rung) in last_week_results.results.iter().enumerate() {
            // this_week_display.results[j] = Rung {
            //     winner1: if ,
            //     winner2: ,
            //     loser1: ,
            //     loser2: ,
            // };
        }
        ladder_league_history.weeks.push(this_week_display);
    }


    html! {
        <h1 class={classes!("bg-red-300")}>{"Hello, world!"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
