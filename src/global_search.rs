use serde::{Serialize};

use crate::handler::QuerySearch;

const PER_PAGE_SIZE: u32 = 15;

#[derive(Default, Serialize)]
pub struct Results {
    events: Box<[String]>,
    informations: Box<[String]>,
}

#[derive(Default, Serialize)]
pub struct Response {
    total_events: u64,
    total_inforamtions: u64,
    results: Results,
}

pub struct GlobalSearchOperation {
    app_state: Option<String>,
    query_params: QuerySearch,
}

impl GlobalSearchOperation {
    pub fn init(app_state: String, conditions: QuerySearch) -> GlobalSearchOperation {
        GlobalSearchOperation {
            app_state: Some(app_state),
            query_params: conditions
        }
    }

    pub fn call(&self) -> Result<Response, Response>{
        let mut response = Response::default();

        match self.search_by_events() {
            Ok(events) => {
                response.total_events = 0; // events.hits.total.value.clone();
                response.results.events = Box::new([String::from("123")]); // events.hits.hits.iter().map( |hit| hit._source).collect().clone();
            },
            Err(e) => (),
        };
        match self.search_by_info() {
            Ok(info) => {
                response.total_events = 0; // events.hits.total.value.clone();
                response.results.events = Box::new([String::from("123")]); // events.hits.hits.iter().map( |hit| hit._source).collect().clone();
            },
            Err(e) => (),
        }

        Ok(response)
    }

    fn search_by_events(&self) -> Result<String, String>{
        // self.app_state.client_elastic
        //     .search(SearchParts::Index(&["events"]))
        //     .from(calc_from)
        //     .size(calc_size)
        //     .body(json!({
        //         "query": {
        //             {
        //                 "multi_match": {
        //                     "query": query_params.q.as_ref().unwrap_or(""),
        //                     "fields": ["title^2", "description"],
        //                 }
        //             }
        //         }
        //     }))
        //     .send()
        //     .and_then( |response| Ok(response.json()) )
        //     .or_else( Err() );
        Ok(String::from(""))
    }

    fn search_by_info(&self) -> Result<String, String>{
        // self.app_state.client_elastic
        //     .search(SearchParts::Index(&["informations"]))
        //     .from(calc_from)
        //     .size(calc_size)
        //     .body(json!({
        //         "query": {
        //             "bool": {
        //                 "should": [
        //                     {
        //                         "mathc": {
        //                             "title": {
        //                                 "query": query_params.q.as_ref().unwrap_or("")
        //                             }
        //                         }
        //                     },
        //                     {
        //                         "nested": {
        //                             "path": "contents",
        //                             "query": {
        //                                 "multi_match": {
        //                                     "query": query_params.q.as_ref().unwrap_or(""),
        //                                     "fields": ["info.content"],
        //                                 }
        //                             }
        //                         }
        //                     },
        //                 ]
        //             }
        //         }
        //     }))
        //     .send()
        //     .and_then( |response| Ok(response.json()) )
        //     .or_else(Err());
        Ok(String::from(""))
    }

    fn calc_from(&self) -> u32 {
        match self.query_params.page {
            Some(page) => page * (PER_PAGE_SIZE - 1),
            None => 0,
        }
    }

    fn calc_size(&self) -> u32 {
        match self.query_params.page {
            Some(page) => page * PER_PAGE_SIZE,
            None => PER_PAGE_SIZE,
        }
    }
}
