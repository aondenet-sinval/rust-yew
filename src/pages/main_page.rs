use yew::{function_component, html, Html, classes};

#[function_component]
pub fn MainPage() -> Html {
    html! {
        <>
            <h1 class={classes!("w3-green")}>{ "Rust + w3css" }</h1>
            <div>
                <h3>{"Desenvolvimento com rust e w3css"}</h3>
                <p style={"background-color: yellow"}>{ "Site desenvolvido para treinamento e demonstração." }</p>
                <p>{ "Rust é flexível e adaptável. Usei classes nesse projeto para incluir o leve w3css." }</p>
                <p>{ "Dois caras rápidos... UAU!!!" }</p>                
            </div>
        </>
    }
}
