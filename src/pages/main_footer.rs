use yew::{function_component, html, Html, classes};

#[function_component]
pub fn MyFooter() -> Html {
    html! {
        <>
            <div class={classes!("w3-container", "w3-black")}>
                <p>{ "foooter..." }</p>
            </div>
        </>
    }
}
