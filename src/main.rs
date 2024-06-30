use yew::prelude::*;
use serde::Deserialize;

// const PARTICIPANTS: u32 = 16;

const START_WEEK: &'static str = "6/9/24";

const JSON_DATA: &'static str = r#"
    {
        "weeks": [
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
            },
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
        <div class={classes!("px-4", "py-2", "w-fit", "justify-center")}>
            <div class={classes!("bg-gray-200", "h-fit", "font-bold", "text-2xl", "rounded-md", "px-2", "py-1", "border-2", "border-gray-700")}>
                {name}
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct WeekSelectorProps {
    week_state: UseStateHandle<usize>,
    history_state: UseStateHandle<WeekList>,
    show_weeks: UseStateHandle<bool>,
}

#[function_component(WeekSelector)]
fn week_selector(WeekSelectorProps {week_state, history_state, show_weeks}: &WeekSelectorProps) -> Html {

    let week_displayed: Week = (*(history_state.clone())).weeks[*(week_state.clone())].clone();

    let on_click = {
        let show_weeks = show_weeks.clone();
        Callback::from(move |_: MouseEvent| {
            show_weeks.set(!(*(show_weeks.clone())))
        })
    };

    html! {
        <div class={classes!("justify-center", "flex", "flex-row", "py-2", "bg-gray-200")}>
            <h1 class={classes!("text-3xl", "font-bold")}>{"Results for Week "}</h1>
            <div class={classes!("px-2")}>
                <button onclick={on_click.clone()} class={classes!("text-3xl", "font-bold", "px-2", "rounded-md", "bg-white", "border", "border-black")}>{week_displayed.week}</button>
            </div>
            <h1 class={classes!("text-xl", "font-bold", "self-center")}>{"(click to change)"}</h1>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct WeekMenuProps {
    week_state: UseStateHandle<usize>,
    history_state: UseStateHandle<WeekList>,
    show_weeks: UseStateHandle<bool>,
}

#[function_component(WeekMenu)]
fn week_menu(WeekMenuProps {week_state, history_state, show_weeks}: &WeekMenuProps) -> Html {

    html! {
        <div class={classes!("absolute", "inset-0","flex", "flex-row", "min-h-screen", "justify-center", "items-center", if *(show_weeks.clone()) {"visible"} else {"invisible"})}>
            <div class={classes!("z-20", "h-1/3", "w-1/3", "rounded-md", "bg-blue-100", "border-2", "border-black")}>
                <div class={classes!("max-h-full", "rounded-md", "overflow-y-auto", "p-2")}>
                    {
                        history_state.weeks.iter().enumerate().map(
                            move |(i, week)| {
                                let on_click = {
                                    let show_weeks = show_weeks.clone();
                                    let week_state = week_state.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        show_weeks.set(false);
                                        week_state.set(i);
                                    })
                                };
                                html! {
                                    <button onclick={on_click.clone()} class={classes!("flex", "justify-center", "h-fit", "w-full", "rounded-md", "border", "border-black", if i==*(week_state.clone()) {"bg-purple-200"} else {"bg-white"})}>
                                        <span class={classes!("font-bold", "text-2xl")}>{week.clone().week}</span>
                                    </button>
                                }
                            }
                        ).collect::<Html>()
                    }
                </div>
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
    let colors: Vec<&str> = vec!["bg-yellow-300", "bg-red-300", "bg-sky-300", "bg-lime-300"];
    let light_colors: Vec<&str> = vec!["bg-yellow-100", "bg-red-100", "bg-sky-100", "bg-lime-100"];
    let names: Vec<&str> = vec!["Champions Club", "Salty Spittoon", "Rivals Ring", "Pickle Pub"];

    let week_displayed: Week = (*(history_state.clone())).weeks[*(week_state.clone())].clone();

    html! {
        <div class={classes!("z-0")}>
            {
                week_displayed.results.iter().enumerate().map(
                    |(i, rung)| {
                        html! {
                            <div class={classes!("relative","rounded-md", colors[i], "grid", "h-64", "grid-cols-4", "content-center", "justify-items-center", "gap-4")}>
                                <div class={classes!("p-1", "absolute", "top-0")}>
                                    <div class={classes!("font-bold", "text-xl", "rounded-lg", light_colors[i], "p-1", "border", "border-black")}>{names[i]}</div>
                                </div>
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
    }
}

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
            results: Vec::<Rung>::new(),
        };

        let last_week_display = ladder_league_history.weeks[i].clone();
        // going through last week's rungs to calculate this week's rungs
        let len = last_week_results.results.len();
        let mut j = 0;
        while j < len {
            // let rung = last_week_results.results[j].clone();
            let last_display_rung = last_week_display.results[j].clone();
            this_week_display.results.push(Rung {
                winner1: if j == last_week_results.results.len() - 1 {last_display_rung.loser1} else {last_week_results.results[j+1].clone().winner1},
                winner2: if j == last_week_results.results.len() - 1 {last_display_rung.loser2} else {last_week_results.results[j+1].clone().winner2},
                loser1: if j == 0 {last_display_rung.winner1} else {last_week_results.results[j-1].clone().loser1},
                loser2: if j == 0 {last_display_rung.winner2} else {last_week_results.results[j-1].clone().loser2},
            });
            j += 1;
        }
        ladder_league_history.weeks.push(this_week_display);
    }

    let history_state = use_state(|| ladder_league_history.clone());
    //index into history
    let week_state = use_state(|| ladder_league_history.clone().weeks.len()-1);
    let show_weeks = use_state(|| false);

    let clickaway = {
        let show_weeks = show_weeks.clone();
        Callback::from(move |_: MouseEvent| {
            show_weeks.set(false)
        })
    };


    html! {
        <div class={classes!("relative")}>
            <WeekSelector week_state={week_state.clone()} history_state={history_state.clone()} show_weeks={show_weeks.clone()}/>
            <WeekDisplay week_state={week_state.clone()} history_state={history_state.clone()} />
            {if *show_weeks.clone() {
                html!{
                    <>
                    <WeekMenu week_state={week_state.clone()} history_state={history_state.clone()} show_weeks={show_weeks.clone()}/>
                    <div onclick={clickaway.clone()} class={classes!("absolute", "inset-0", "z-10")}></div>
                    </>
                }
            }
            else {
                html!{}
            }}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
