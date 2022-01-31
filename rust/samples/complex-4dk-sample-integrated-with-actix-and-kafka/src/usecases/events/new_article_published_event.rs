use chrono::{DateTime, Utc};
use dddk_macro::Event;
use dddk_core::dddk::event::event::Event;
use std::any::Any;
use crate::domain::article::Article;

#[derive(Event, Debug)]
pub struct NewArticleSubmittedEvent {
    title: String,
    date_time: DateTime<Utc>,
}

impl NewArticleSubmittedEvent {
    pub fn new(article: &Article) -> NewArticleSubmittedEvent {
        NewArticleSubmittedEvent {
            title: article.get_title().clone(),
            date_time: Utc::now(),
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_date_time(&self) -> &DateTime<Utc> {
        &self.date_time
    }
}
