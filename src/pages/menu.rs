use yew::{function_component, html, Html, classes};

#[function_component]
pub fn MenuBar() -> Html {
    html! {
        <>
            <nav class={classes!("w3-bar", "w3-black")}>
                <a href="#" class={classes!("w3-bar-item", "w3-button")}>{"Home"}</a>
                <a href="#" class={classes!("w3-bar-item", "w3-button")}>{"Link 1"}</a>
                <a href="#" class={classes!("w3-bar-item", "w3-button")}>{"Link 2"}</a>
                <a href="#" class={classes!("w3-bar-item", "w3-button")}>{"Link 3"}</a>
            </nav>
        </>
    }
}
