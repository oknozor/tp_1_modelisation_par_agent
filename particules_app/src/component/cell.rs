use particules::agent::Color as AgentColor;
use yew::{html, macros::Properties, Component, ComponentLink, Html, ShouldRender};

pub struct CellComponent {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub x: u32,
    pub y: u32,
}

impl Component for CellComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CellComponent { link, props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<div class=("cell", Color::None.as_str())></div>}
    }
}

#[derive(Clone)]
pub enum Color {
    Red,
    Black,
    None,
}

impl Color {
    pub fn from(color: AgentColor) -> Self {
        match color {
            AgentColor::Black => Color::Black,
            AgentColor::Red => Color::Red,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Color::Black => "cell-black",
            Color::Red => "cell-red",
            Color::None => "cell-empty",
        }
    }
}
