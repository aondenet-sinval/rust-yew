use yew::prelude::*;
mod pages;
use pages::menu::MenuBar;
use pages::main_page::MainPage;
#[function_component]
fn App() -> Html {

    html! {
        <>
            <MenuBar />
            <MainPage />
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
