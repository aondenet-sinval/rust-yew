use yew::prelude::*;
use yew::{classes};
mod pages;
use pages::menu::MenuBar;
use pages::main_page::MainPage;
use pages::main_sidebar::MySidebar;
use pages::main_footer::MyFooter;
#[function_component]
fn App() -> Html {

    html! {
        <>
            <div class={classes!("geral")}>
                <div class={classes!("menu-bar")}><MenuBar /></div>
                <div class={classes!("page-principal")}><MainPage /></div>
                <div class={classes!("sidebar-left")}><MySidebar /></div>
                <div class={classes!("my-footer")}><MyFooter /></div>
            </div>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
