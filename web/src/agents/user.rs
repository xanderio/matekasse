use std::collections::{HashMap, HashSet};

use anyhow::Error;
use common::{Product, User};
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
    users: HashMap<i32, User>,
    subscribers: HashSet<HandlerId>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Response(Result<Vec<User>, Error>),
    UserUpdate(Result<User, Error>),
}

#[non_exhaustive]
pub enum Input {
    ChangeUser(User),
    Buy(Product),
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
            users: HashMap::new(),
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
                for user in &users {
                    self.users.insert(user.id, user.clone());
                }
                for id in self.subscribers.iter() {
                    self.link.respond(*id, Output::Update(users.clone()))
                }
            }
            Msg::Response(Err(e)) => {
                log::error!("{}", e);
            }
            Msg::UserUpdate(Ok(user)) => {
                log::debug!("received update for user: {} \n {:?}", user.id, &user);
                self.users.insert(user.id, user.clone());
                let users: Vec<User> = self.users.clone().into_values().collect();
                for id in self.subscribers.iter() {
                    self.link.respond(*id, Output::Update(users.clone()))
                }
                if self.current_user.as_ref().map(|u| u.id) == Some(user.id) {
                    log::debug!("received update for current user");
                    for id in self.subscribers.iter() {
                        self.link.respond(*id, Output::Current(Some(user.clone())))
                    }
                    self.current_user = Some(user);
                }
            }
            Msg::UserUpdate(Err(e)) => {
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
            Input::Buy(product) => {
                self.buy(product);
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        let users: Vec<User> = self.users.clone().into_values().collect();
        self.link.respond(id, Output::Update(users));
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

    fn buy(&mut self, product: Product) {
        if let Some(user) = &self.current_user {
            let request = Request::post(format!("/api/v3/users/{}/buy", user.id))
                .body(Ok(product.id.to_string()))
                .expect("Could build request");

            let task = FetchService::fetch(
                request,
                self.link
                    .callback(|resp: Response<Json<Result<User, Error>>>| {
                        let Json(data) = resp.into_body();
                        Msg::UserUpdate(data)
                    }),
            )
            .expect("unable to build fetch task");
            self.fetch_task = Some(task);
        }
    }
}
