extern crate news_contract;
extern crate news_dao;

use news_contract::News;

pub async fn list_news() -> Option<Vec<News>> {
  return news_dao::list_news().await;
}