use particules::sma::Sma;
use std::time::Duration;
use yew::{
    html,
    macros::Properties,
    services::{IntervalService, Task},
    Component, ComponentLink, Html, NodeRef, ShouldRender,
};

use super::cell::CellComponent;
use super::cell::Color;
use log::trace;
use stdweb::web::Element;
use stdweb::web::IElement;

pub struct Grid {
    link: ComponentLink<Self>,
    props: Props,
    sma: Sma,
    active: bool,
    refs: Vec<NodeRef>,
    #[allow(unused)]
    job: Box<dyn Task>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub width: u32,
    pub height: u32,
}

pub enum Msg {
    Start,
    Stop,
    Step,
    Tick,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Tick);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(200), callback);
        let sma = Sma::new(props.width, props.height);
        let refs = Self::init_refs(&sma);

        Grid {
            link,
            sma,
            props,
            refs,
            active: false,
            job: Box::new(handle),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.sma = Sma::new(self.props.width, self.props.height);
        self.refs = Self::init_refs(&self.sma);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Start => {
                self.active = true;
            }
            Msg::Stop => {
                self.active = false;
            }
            Msg::Step => {
                if self.active {
                    self.active = false;
                }
                self.clear_filled_cells();
                self.sma.tick();
                self.draw_agents();
            }
            Msg::Tick => {
                if self.active {
                    self.clear_filled_cells();
                    self.sma.tick();
                    self.draw_agents();
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="environment-area">
                <div class="game-buttons">
                    <button class="game-button" onclick=self.link.callback(|_| Msg::Start)>{ "Start" }</button>
                    <button class="game-button" onclick=self.link.callback(|_| Msg::Stop)>{ "Stop" }</button>
                    <button class="game-button" onclick=self.link.callback(|_| Msg::Step)>{ "Step" }</button>
                </div>

                <div class="particules">
                    {(0..self.props.width).map(|row| self.view_row(row)).collect::<Html>()}
                </div>
            </section>
        }
    }
}

impl Grid {
    fn view_row(&self, row: u32) -> Html {
        html! {
            <div class=("row")>
                {for (0..self.props.height).map(|column| {
                    self.view_cell(row, column)
                })}
            </div>
        }
    }

    fn view_cell(&self, x: u32, y: u32) -> Html {
        let idx = self.sma.get_index(x, y);
        html! {
            <CellComponent ref=self.refs[idx].clone()/>
        }
    }

    pub fn init_refs(sma: &Sma) -> Vec<NodeRef> {
        let mut refs = vec![];
        for _ in sma.get_state() {
            refs.push(NodeRef::default())
        }

        refs
    }

    fn clear_filled_cells(&mut self) {
        self.sma.agents.iter().for_each(|agent| {
            if let Some(cell) =
                self.refs[self.sma.get_index(agent.x, agent.y)].try_into::<Element>()
            {
                cell.set_attribute("class", &format!("cell {}", Color::None.as_str()))
                    .expect(":(");
            }
        });
    }

    fn draw_agents(&mut self) {
        self.sma.agents.iter().for_each(|agent| {
            let color = Color::from(agent.color);

            let idx = self.sma.get_index(agent.x, agent.y);

            if let Some(cell) = self.refs[idx].try_into::<Element>() {
                trace!("INFO : Agent {} {} {:?}", agent.x, agent.y, agent.color);
                cell.set_attribute("class", &format!("cell {}", color.as_str()))
                    .expect(":(");
            } else {
                trace!("ERROR : Agent {} {} {:?}", agent.x, agent.y, agent.color);
            }
        });
    }
}
