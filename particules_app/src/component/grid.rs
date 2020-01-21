use log::trace;
use particules::agent::Agent;
use particules::sma::Sma;
use particules::AgentRef;
use particules::Direction;
use particules::HDirection;
use particules::Point;
use particules::VDirection;

use std::time::Duration;
use yew::{
    html,
    macros::Properties,
    services::{IntervalService, Task},
    Component, ComponentLink, Html, NodeRef, ShouldRender,
};

use super::cell::CellComponent;
use super::cell::Color;
use stdweb::web::Element;
use stdweb::web::IElement;

pub struct Grid {
    link: ComponentLink<Self>,
    props: Props,
    sma: Sma,
    direction: Direction,
    active: bool,
    borderless: bool,
    error: String,
    refs: Vec<NodeRef>,
    #[allow(unused)]
    job: Box<dyn Task>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub width: i32,
    pub height: i32,
    pub agents: Vec<AgentRef>,
}

pub enum Msg {
    Play,
    AddAgent((i32, i32)),
    Clear,
    Step,
    Tick,
    Borderless,
    ChangeDir(Direction),
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Tick);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(70), callback);
        let sma = Sma::new(props.width, props.height);
        let refs = Self::init_refs(&sma);
        let direction = Direction::new(HDirection::Right, VDirection::None);

        Grid {
            link,
            sma,
            props,
            direction,
            error: "".into(),
            refs,
            active: false,
            borderless: false,
            job: Box::new(handle),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.draw_agents();
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddAgent((x, y)) => {
                if self.direction != Direction::new(HDirection::None, VDirection::None) {
                    let coordinate = Point { x, y };

                    self.sma.add_agent(coordinate, self.direction);
                    self.draw_agents();
                } else {
                    self.error = "Please chose a direction".into()
                }
                return false;
            }
            Msg::Play => {
                self.active = !self.active;
                return true;
            }
            Msg::Clear => {
                self.active = false;
                self.clear_filled_cells();
                self.sma = Sma::new(self.props.width, self.props.height);
                return true;
            }
            Msg::Step => {
                if self.active {
                    self.active = false;
                }
                self.clear_filled_cells();
                self.sma.tick();
                self.draw_agents();
                return true;
            }
            Msg::Tick => {
                if self.active {
                    self.clear_filled_cells();
                    self.sma.tick();
                    self.draw_agents();
                }
            }
            Msg::ChangeDir(dir) => {
                self.direction = dir;
                return true;
            }
            Msg::Borderless => {
                self.sma.set_borderless(!self.borderless);
                return true;
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="environment-area">
            <div class="menu">
                <div class="game-buttons">
                        <button class="game-button" onclick=self.link.callback(|_| Msg::Play)>{ if !self.active {{"Play"}} else {{"Pause"}} }</button>
                        <button class="game-button" onclick=self.link.callback(|_| Msg::Clear)>{ "Clear" }</button>
                        <button class="game-button" onclick=self.link.callback(|_| Msg::Step)>{ "Step" }</button>
                        <button class="game-button" onclick=self.link.callback(|_| Msg::Borderless)>{ "Borderless" }</button>
                </div>
                <div class="">
                    <div class="row">
                        <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::Left, VDirection::Up)))><i class="left-up"></i></button>
                        <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::None, VDirection::Up)))><i class="up"></i></button>
                        <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::Right, VDirection::Up)))><i class="right-up"></i></button>
                    </div>
                    <div class="row">
                        <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::Left, VDirection::None)))><i class="left"></i></button>
                        <span class="dot"></span>
                        <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::Right, VDirection::None)))><i class="right"></i></button>
                    </div>
                    <div class="row">
                    <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::Left, VDirection::Down)))><i class="left-down"></i></button>
                    <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::None, VDirection::Down)))><i class="down"></i></button>
                    <button class="" onclick=self.link.callback(|_| Msg::ChangeDir(Direction::new(HDirection::Right, VDirection::Down)))><i class="right-down"></i></button>
                    </div>
                </div>
            </div>
            <div>
                <i class={self.dir_to_arrow(self.direction)}> </i>
            </div>
            <div class="particules">
                {(0..self.props.width).map(|row| self.view_row(row)).collect::<Html>()}
            </div>
            <div>
                {self.view_debug()}
            </div>
            </section>
        }
    }
}

impl Grid {
    fn view_row(&self, y: i32) -> Html {
        html! {
            <div class=("row")>
                {for (0..self.props.width).map(|x| {
                    self.view_cell(Point {x, y})
                })}
            </div>
        }
    }

    fn view_cell(&self, point: Point) -> Html {
        let idx = self.sma.get_index(point);
        html! {
            <CellComponent x={point.x} y ={point.y} ref=self.refs[idx].clone() on_click=self.link.callback(Msg::AddAgent)/>
        }
    }

    fn view_debug(&self) -> Html {
        html! {
            <div>
            {
                self.sma.agents.iter().enumerate().map(|(idx, agent)| {
                    {self.agent_info((idx, agent))}
                }).collect::<Html>()
            }
            </div>
        }
    }

    fn agent_info(&self, (idx, agent): (usize, &AgentRef)) -> Html {
        html! {
            <div class ="row">
                {idx}
                {"| \tx : "} {agent.coordinate().x}
                {"\ty : "} {agent.coordinate().y}
                {"\tcollsion : "} {agent.collision()}
                {"\t direction : "} <i class={self.dir_to_arrow(agent.direction())}></i>
            </div>
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
        self.refs.iter().for_each(|cell_ref| {
            if let Some(cell) = cell_ref.try_into::<Element>() {
                cell.set_attribute("class", &format!("cell {}", Color::None.as_str()))
                    .unwrap_or_else(|_| {
                        trace!("Something went wrong updating html refs, please refresh the page")
                    })
            }
        });
    }

    fn draw_agents(&mut self) {
        self.sma.agents.iter().for_each(|agent| {
            let color = Color::from(agent.collision());

            let idx = self.sma.get_index(agent.coordinate());

            if let Some(cell) = self.refs[idx].try_into::<Element>() {
                cell.set_attribute("class", &format!("cell {}", color.as_str()))
                    .expect(":(");
            }
        });
    }

    fn dir_to_arrow(&self, direction: Direction) -> &str {
        match (direction.x, direction.y) {
            (HDirection::None, VDirection::Up) => "up",
            (HDirection::None, VDirection::Down) => "down",
            (HDirection::Right, VDirection::None) => "right",
            (HDirection::Left, VDirection::None) => "left",
            (HDirection::Right, VDirection::Down) => "right-down",
            (HDirection::Right, VDirection::Up) => "right-up",
            (HDirection::Left, VDirection::Down) => "left-down",
            (HDirection::Left, VDirection::Up) => "left-up",
            (_, _) => "dot",
        }
    }
}
