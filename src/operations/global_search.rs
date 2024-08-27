use std::sync::Arc;

use serde::Serialize;
use serde_json::{json, Value};
use elasticsearch::Error;
use elasticsearch::http::response::Response as ElasticResponse;

use crate::{config::application::Application, handlers::global_search::QuerySearch};

#[derive(Default, Serialize, Clone)]
pub struct Results {
    events: Vec<Value>,
    informations: Vec<Value>,
}

#[derive(Default, Serialize, Clone)]
pub struct Response {
    total_events: i64,
    total_inforamtions: i64,
    results: Results,
}

pub struct GlobalSearchOperation {
    app_state: Arc<Application>,
    query_params: QuerySearch,
    result: Response
}

impl GlobalSearchOperation {
    pub fn init(app_state: Arc<Application>, query_params: QuerySearch) -> GlobalSearchOperation {
        GlobalSearchOperation {
            app_state,
            query_params,
            result: Response::default(),
        }
    }

    pub async fn call(&mut self) -> Result<Response, Response>{
        self.search_by_events().await;
        self.search_by_info().await;

        Ok(self.result.clone())
    }

    async fn parse_response(&self, response: ElasticResponse) -> Result<(i64, Vec<Value>), Error> { // -> Result<Vec<(u64, Value)>, String> {
        let response_body = response.json::<Value>().await?;

        let took = response_body["hits"]["total"]["value"].as_i64().unwrap_or(0);
        let hits = response_body["hits"]["hits"]
            .as_array().unwrap_or(&vec![])
            .into_iter().map( |hit| hit["_source"].clone()).collect();

        Ok(
            (took, hits)
        )
    }

    async fn search_by_events(&mut self) -> Result<(), Error> {
        let result = self.app_state.client_elastic
            .search(elasticsearch::SearchParts::Index(&["events"]))
            .from(self.calc_from() as i64)
            .size(self.calc_size() as i64)
            .body(json!({
                "query": {
                    "multi_match": {
                        "query": self.request_query(),
                        "fields": ["title^2", "description"],
                    }
                }
            }))
            .send()
            .await?;

        match self.parse_response(result).await {
            Ok((total, values)) => {
                self.result.total_events = total.clone();
                self.result.results.events = values.clone();
            },
            Err(_) => (),
        };

        return Ok(());
    }

    async fn search_by_info(&mut self) -> Result<(), Error> {
        let result = self.app_state.client_elastic
            .search(elasticsearch::SearchParts::Index(&["informations"]))
            .from(self.calc_from() as i64)
            .size(self.calc_size() as i64)
            .body(json!({
                "query": {
                    "bool": {
                        "should": [
                            {
                                "mathc": {
                                    "title": {
                                        "query": self.request_query()
                                    }
                                }
                            },
                            {
                                "nested": {
                                    "path": "contents",
                                    "query": {
                                        "multi_match": {
                                            "query": self.request_query(),
                                            "fields": ["info.content"],
                                        }
                                    }
                                }
                            },
                        ]
                    }
                }
            }))
            .send()
            .await?;

        match self.parse_response(result).await {
            Ok((total, values)) => {
                self.result.total_inforamtions = total.clone();
                self.result.results.informations = values.clone();
            },
            Err(_) => (),
        };

        return Ok(());
    }

    fn calc_from(&self) -> u32 {
        match self.query_params.page.clone() {
            Some(page) => page * (self.per_page_size() - 1),
            None => 0,
        }
    }

    fn calc_size(&self) -> u32 {
        match self.query_params.page.clone() {
            Some(page) => page * self.per_page_size(),
            None => self.per_page_size(),
        }
    }

    fn request_query(&self) -> String {
        match self.query_params.q.clone() {
            Some(q) => q,
            None => String::from("")
        }
    }

    fn per_page_size(&self) -> u32 {
        return 15;
    }
}
