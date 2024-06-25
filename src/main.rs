use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class={classes!("bg-red-300")}>{"Hello, world!"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
