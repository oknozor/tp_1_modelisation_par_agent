#![recursion_limit = "512"]

use particules::environment::Cell;
use particules::sma::Sma;
use std::time::Duration;
use yew::{
    InputData,
    html,
    services::{IntervalService, Task},
    Component, ComponentLink, Html, ShouldRender,
};

pub struct Model {
    link: ComponentLink<Self>,
    sma: Sma,
    height: u32,
    width: u32,
    error: String,
    active: bool,
    #[allow(unused)]
    job: Box<dyn Task>,
}

pub enum Msg {
    UpdateHeight(String),
    UpdateWidth(String),
    Update,
    Start,
    Stop,
    Tick,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Tick);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(200), callback);
        let sma = Sma::new(15, 15);

        Model {
            link,
            sma,
            error: "".into(),
            height: 10,
            width: 10,
            active: false,
            job: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Start => {
                self.sma.draw_all();
                self.active = true;
            }
            Msg::Stop => {
                self.active = false;
            }
            Msg::Tick => {
                if self.active {
                    self.sma.tick();
                }
            }
            Msg::UpdateHeight(value) => {
                match value.parse::<u32>() {
                    Ok(value) => self.height = value,
                    Err(e) => self.error = e.to_string()
                }
            }
            Msg::UpdateWidth(value) => {
                match value.parse::<u32>() {
                    Ok(value) => self.width = value,
                    Err(e) => self.error = e.to_string()
                }
            }
            Msg::Update => {
                self.active = false;
                self.sma = Sma::new(self.width, self.height)
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="environment-area">
                <div class="environment-form">
                    <input
                        height=&self.height
                            oninput=self.link.callback(|e: InputData| Msg::UpdateWidth(e.value))
                        placeholder="height">
                    </input>
                    <input
                        height=&self.height
                            oninput=self.link.callback(|e: InputData| Msg::UpdateHeight(e.value))
                        placeholder="width">
                    </input>
                    <button class="update-env" onclick=self.link.callback(|_| Msg::Update)>{ "Update" }</button>
                    <p class="error">{&self.error}</p>
                </div>
                <div class="game-buttons">
                    <p> {&self.height}</p>
                    <button class="game-button" onclick=self.link.callback(|_| Msg::Start)>{ "Start" }</button>
                    <button class="game-button" onclick=self.link.callback(|_| Msg::Stop)>{ "Stop" }</button>
                </div>
                <div class="particules">
                    {
                        for self.sma.get_state().iter().enumerate().map(|c| {
                            {self.view_cell(c)}
                        })
                    }
                </div>
            </section>
        }
    }
}

impl Model {
    fn view_cell(&self, (idx, cell): (usize, &Cell)) -> Html {
        let cell_status = match cell {
            Cell::Filled(agent) => {
                let agent = agent.borrow();
                let color = agent.color.as_str();
                format!("cell-{}", color)
            }
            Cell::Empty => "cell-empty".into(),
        };

        html! {
            <div>
            {
                if (idx + 1)  % (self.sma.width() as usize) == 0 {
                    html! {
                        <div class=("row")>
                            <div class=("cell", cell_status)>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class=("cell", cell_status)>
                        </div>
                    }
                }
            }
            </div>
        }
    }
}
