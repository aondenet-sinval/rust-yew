use yew::{function_component, html, Html, classes};

#[function_component]
pub fn MySidebar() -> Html {
    html! {
        <>
            <div class={classes!("form-align")}>
                <h3>{"Contato:"}</h3>
                <form>
                <label>{"Nome: "}</label>
                <input class={classes!("w3-input")} type="text" id="nome" value={"seu nome"} /><br />
                <label>{"Telefone: "}</label>
                <input class={classes!("w3-input")} type="number" id="telefone" step="0" /><br />
                <label>{"Endereço: "}</label>
                <input class={classes!("w3-input")} type="text" id="endereco" value={"seu endereço"} /><br />
                <label>{"Email: "}</label>
                <input class={classes!("w3-input")} type="email" id="email" value={"Email"} /><br />
                <input type="submit" value={"Enviar"} />
                </form>
            </div>
        </>
    }
}
