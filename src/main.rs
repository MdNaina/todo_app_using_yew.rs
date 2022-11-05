use yew::prelude::*;


enum Msg {
    AddOne,
    InputChange
}

struct Model {
    value: i64,
    name: String
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            name: "Hi!!!".to_string()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let name = self.name.clone();

        html! {
            <>
            <input value={name} on_change={|e| dbg!(e)} />
            <h1>{"Hello World !!!"}</h1>
            </>
        }

    }
}


fn main() {
    println!("Hello, world!");
    yew::start_app::<Model>();
}
