use yew::{html, macros::Properties, Callback, Component, ComponentLink, Html, ShouldRender};

pub struct CellComponent {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub x: i32,
    #[props(required)]
    pub y: i32,
    #[props(required)]
    pub on_click: Callback<(i32, i32)>,
}

pub enum Msg {
    CreateAgent,
}

impl Component for CellComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CellComponent { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreateAgent => {
                self.props.on_click.emit((self.props.x, self.props.y));
            }
        }
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<div onclick=self.link.callback(move|_| Msg::CreateAgent) class=("cell", Color::None.as_str())></div>}
    }
}

#[derive(Clone)]
pub enum Color {
    Red,
    Black,
    None,
}

impl Color {
    pub fn from(collision: bool) -> Self {
        if collision {
            Color::Red
        } else {
            Color::Black
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
