use std::collections::HashSet;

use anyhow::Error;
use common::User;
use yew::{
    format::{Json, Nothing},
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    worker::*,
};

#[derive(Debug)]
pub struct UserStore {
    link: AgentLink<Self>,
    current_user: Option<User>,
    users: Vec<User>,
    subscribers: HashSet<HandlerId>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Response(Result<Vec<User>, Error>),
}

#[non_exhaustive]
pub enum Input {
    ChangeUser(User),
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Output {
    Current(Option<User>),
    Update(Vec<User>),
}

impl Agent for UserStore {
    type Reach = Context<Self>;

    type Message = Msg;

    type Input = Input;

    type Output = Output;

    fn create(link: AgentLink<Self>) -> Self {
        let mut agent = Self {
            link,
            users: Vec::new(),
            current_user: None,
            subscribers: HashSet::new(),
            fetch_task: None,
        };
        agent.fetch();
        agent
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Response(Ok(users)) => {
                self.users = users;
                for id in self.subscribers.iter() {
                    self.link.respond(*id, Output::Update(self.users.clone()))
                }
            }
            Msg::Response(Err(e)) => {
                log::error!("{}", e);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Input::ChangeUser(user) => {
                self.current_user = Some(user);
                for id in self.subscribers.iter() {
                    self.link
                        .respond(*id, Output::Current(self.current_user.clone()))
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link.respond(id, Output::Update(self.users.clone()));
        self.link
            .respond(id, Output::Current(self.current_user.clone()));
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl UserStore {
    fn fetch(&mut self) {
        let request = Request::get("/api/v3/users")
            .body(Nothing)
            .expect("Could build request");

        let task = FetchService::fetch(
            request,
            self.link
                .callback(|resp: Response<Json<Result<Vec<User>, Error>>>| {
                    let Json(data) = resp.into_body();
                    Msg::Response(data)
                }),
        )
        .expect("unable to build fetch task");
        self.fetch_task = Some(task);
    }
}
