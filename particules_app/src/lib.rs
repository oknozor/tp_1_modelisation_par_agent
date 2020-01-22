#![recursion_limit = "1024"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
mod component;
use component::grid::Grid;

pub struct Model {
    link: ComponentLink<Self>,
    height: i32,
    width: i32,
    error: String,
    redraw: bool,
}

pub enum Msg {
    Update,
    UpdateHeight(String),
    UpdateWidth(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            height: 20,
            width: 20,
            redraw: false,
            error: "".into(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHeight(value) => {
                match value.parse::<i32>() {
                    Ok(value) => {
                        self.error = "".into();
                        self.height = value;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                false
            }
            Msg::UpdateWidth(value) => {
                match value.parse::<i32>() {
                    Ok(value) => {
                        self.error = "".into();
                        self.width = value;
                    }
                    Err(e) => self.error = e.to_string(),
                }
                false
            }
            Msg::Update => {
                self.redraw = !self.redraw;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <div class="environment-form row">
                <input height=&self.height oninput=self.link.callback(|e: InputData| Msg::UpdateWidth(e.value)) placeholder="height"> </input>
                        <input height=&self.height oninput=self.link.callback(|e: InputData| Msg::UpdateHeight(e.value)) placeholder="width"></input>
                        <button class="game-button" onclick=self.link.callback(|_| Msg::Update)>{ if self.redraw {{"Create"}} else { {"Update"} }  }</button>
                        <p> {"height : " } {&self.height}</p>
                        <p> {"widht : " } {self.width}</p>
            </div>
                <p color="red"> {"error : " } {&self.error}</p>
                {
                    if !self.redraw {
                       html! { <Grid height={self.height}, width = {self.width}/> }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}