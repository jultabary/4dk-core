#[macro_use]
extern crate diesel;

use std::cell::RefCell;
use std::rc::Rc;
use actix_web::{App, HttpServer};
use dddk_core::Bus;
use dddk_core::dddk::command::command_handler::CommandHandlerInBus;
use dddk_core::dddk::event::event_handler::EventHandlerInBus;
use dddk_core::dddk::query::query_handler::QueryHandlerInBus;
use log::LevelFilter;
use crate::infrastructure::api::routes::{get_all_news_paper, post_one_news_paper};
use crate::infrastructure::database::database_repository::{establish_connection, NewsPaperRepositoryAdapter};
use crate::logger::SimpleLogger;
use crate::usecases::commands::open_news_paper_command_handler::OpenNewsPaperCommandHandler;
use crate::usecases::commands::publish_article_command_handler::PublishArticleCommandHandler;
use crate::usecases::queries::what_are_opened_news_papers_query_handler::WhatAreOpenedNewsPaperQueryHandler;

mod domain;
mod infrastructure;
mod usecases;
mod logger;
pub mod schema;

static LOGGER: SimpleLogger = SimpleLogger {};

pub struct Context {
    bus: Bus,
}

impl Context {
    pub fn new() -> Context {
        // clone a Rc smart pointer doesn't copy the value, it creates a new pointer. See Rc and Arc documentation for more detail
        let connection = Rc::new(establish_connection());
        let news_paper_repository = Rc::new(NewsPaperRepositoryAdapter::new(connection));

        let open_news_paper_command_handler = OpenNewsPaperCommandHandler::new(news_paper_repository.clone());
        let publish_article_command_handler = PublishArticleCommandHandler::new(news_paper_repository.clone());
        let mut command_handlers = Vec::new() as Vec<Box<dyn CommandHandlerInBus>>;
        command_handlers.push(Box::new(open_news_paper_command_handler));
        command_handlers.push(Box::new(publish_article_command_handler));

        let what_are_opened_news_paper_query_handler = WhatAreOpenedNewsPaperQueryHandler::new(news_paper_repository.clone());
        let mut query_handlers = Vec::new() as Vec<Box<dyn QueryHandlerInBus>>;
        query_handlers.push(Box::new(what_are_opened_news_paper_query_handler));

        let event_handlers = Vec::new() as Vec<Box<dyn EventHandlerInBus>>;

        let bus = Bus::new(command_handlers, event_handlers, query_handlers);
        Context {
            bus
        }
    }
    pub fn get_bus(&self) -> &Bus {
        &self.bus
    }
}

unsafe impl Sync for Context {}

unsafe impl Send for Context {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    // I prefer to copy middleware rather share all the bus between the two contexts (Actix and Kafka)
    // Bus is stateless and copy it does not cos a lot
    // In the other hand, i have shared the database context in both with an Arc.
    HttpServer::new(
        || {
            let context = RefCell::new(Context::new());
            App::new()
                .service(get_all_news_paper)
                .service(post_one_news_paper)
                .data(context)
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
