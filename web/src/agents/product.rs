use std::collections::HashSet;

use anyhow::Error;
use common::Product;
use yew::{
    format::{Json, Nothing},
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    worker::*,
};

#[derive(Debug)]
pub struct ProductStore {
    link: AgentLink<Self>,
    products: Vec<Product>,
    subscribers: HashSet<HandlerId>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Response(Result<Vec<Product>, Error>),
}

#[non_exhaustive]
pub enum Output {
    Update(Vec<Product>),
}

impl Agent for ProductStore {
    type Reach = Context<Self>;

    type Message = Msg;

    type Input = ();

    type Output = Output;

    fn create(link: AgentLink<Self>) -> Self {
        let mut agent = Self {
            link,
            products: Vec::new(),
            subscribers: HashSet::new(),
            fetch_task: None,
        };
        agent.fetch();
        agent
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Response(Ok(products)) => {
                self.products = products;
                for id in self.subscribers.iter() {
                    self.link
                        .respond(*id, Output::Update(self.products.clone()))
                }
            }
            Msg::Response(Err(e)) => {
                log::error!("{}", e);
            }
        }
    }

    fn handle_input(&mut self, _msg: Self::Input, _id: HandlerId) {}

    fn connected(&mut self, id: HandlerId) {
        if !self.products.is_empty() {
            self.link.respond(id, Output::Update(self.products.clone()))
        }
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl ProductStore {
    fn fetch(&mut self) {
        let request = Request::get("/api/v3/products")
            .body(Nothing)
            .expect("Could build request");

        let task = FetchService::fetch(
            request,
            self.link
                .callback(|resp: Response<Json<Result<Vec<Product>, Error>>>| {
                    let Json(data) = resp.into_body();
                    Msg::Response(data)
                }),
        )
        .expect("unable to build fetch task");
        self.fetch_task = Some(task);
    }
}
