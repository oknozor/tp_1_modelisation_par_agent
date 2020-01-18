use particules::agent::Color as AgentColor;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct CellComponent(ComponentLink<Self>);

impl Component for CellComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CellComponent(link)
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
