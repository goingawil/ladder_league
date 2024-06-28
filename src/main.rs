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

#[derive(Properties, Clone, PartialEq)]
struct PlayerProps {
    name: String
}

#[function_component(Player)]
fn player(PlayerProps {name}: &PlayerProps) -> Html {
    html! {
        <div class={classes!("px-4", "py-2")}>
            <div class={classes!("bg-violet-200", "h-full", "w-full", "font-bold", "text-xl", "rounded-md", "px-2", "py-1")}>
                {name}
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct WeekDisplayProps {
    week_state: UseStateHandle<usize>,
    history_state: UseStateHandle<WeekList>,
}

#[function_component(WeekDisplay)]
fn week_display(WeekDisplayProps {week_state, history_state}: &WeekDisplayProps) -> Html {
    let colors: Vec<&str> = vec!["bg-yellow-300", "bg-red-500", "bg-sky-400", "bg-lime-500"];
    let names: Vec<&str> = vec!["Champions Club", "Salty Spittoon", "Rivals Ring", "Pickle Pub"];

    let week_displayed: Week = (*(history_state.clone())).weeks[*(week_state.clone())].clone();

    html! {
        <>
            <div class={classes!("flex", "flex-col")}>
                <div class={classes!("justify-center", "flex", "flex-row", "py-2")} style="height:10%;">
                    <h1 class={classes!("text-3xl", "font-bold")}>{format!("Results for Week {}", week_displayed.week)}</h1>
                </div>
                <div>
                {
                    week_displayed.results.iter().map(
                        |rung| {
                            html! {
                                <div class={classes!("flex", "flex-row")} style="height:18%;">
                                    <Player name={rung.clone().winner1} />
                                    <Player name={rung.clone().winner2} />
                                    <Player name={rung.clone().loser1} />
                                    <Player name={rung.clone().loser2} />
                                </div>
                            }
                        }
                    ).collect::<Html>()
                }
                </div>
            </div>
        </>
    }
}

// ladder is updated every Sunday
// let's say ladder is updated Sunday, 6/30
// the site will then show the ladder after incorporating the results from week 6/23
// so the week for a given results will actually be a week ahead, i.e. *these are the results we want to show starting the given week*
// Ladder for 6/30/24
// {ladder after results from week of 6/23/24}
#[function_component(App)]
fn app() -> Html {

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

        // let last_week_display = ladder_league_history.weeks[i].clone();
        // going through last week's rungs to calculate this week's rungs
        let len = last_week_results.results.len();
        let mut j = 0;
        while j < len {
            let rung = last_week_results.results[j].clone();
            this_week_display.results.push(Rung {
                winner1: if j < last_week_results.results.len() {rung.winner1} else {last_week_results.results[j+1].clone().winner1},
                winner2: if j < last_week_results.results.len() {rung.winner2} else {last_week_results.results[j+1].clone().winner2},
                loser1: if j == 0 {rung.loser1} else {last_week_results.results[j-1].clone().loser1},
                loser2: if j == 0 {rung.loser2} else {last_week_results.results[j-1].clone().loser1},
            });
            j += 1;
        }

        ladder_league_history.weeks.push(this_week_display);
    }

    let history_state = use_state(|| ladder_league_history.clone());
    //index into history
    let week_state = use_state(|| ladder_league_history.clone().weeks.len()-1);

    html! {
        <WeekDisplay week_state={week_state.clone()} history_state={history_state.clone()} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
