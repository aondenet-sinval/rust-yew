use yew::{function_component, html, Html};

#[function_component]
pub fn MyFooter() -> Html {
    html! {
        <>
            <div>
                <p>{ "foooter..." }</p>
            </div>
        </>
    }
}
