use particules::environment::Cell;
use particules::sma::Sma;
use std::time::Duration;
use yew::{
    html,
    services::{IntervalService, Task},
    Component, ComponentLink, Html, ShouldRender,
};

pub struct Model {
    link: ComponentLink<Self>,
    sma: Sma,
    active: bool,
    cellules_width: usize,
    cellules_height: usize,
    #[allow(unused)]
    job: Box<dyn Task>,
}

pub enum Msg {
    Start,
    Tick,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Tick);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(200), callback);
        let sma = Sma::new();
        let height = sma.height() as usize;
        let width = sma.width() as usize;
        Model {
            link,
            sma,
            active: false,
            cellules_width: width,
            cellules_height: height,
            job: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Start => {
                self.sma.draw_all();
            }
            Msg::Tick => {
                self.sma.tick();
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="environment-area">
                <p> {self.sma.width()} </p>
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
            Cell::Filled => "cell-filled",
            Cell::Empty => "cell-empty",
        };

        html! {
            <div>
            {
                if (idx + 1)  % (self.sma.width() as usize) == 0 {
                    html! {
                        <div class=("row")>
                            <div class=("cell", cell_status)>
                                {idx}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class=("cell", cell_status)>
                            {idx}
                        </div>
                    }
                }
            }
            </div>
        }
    }
}
