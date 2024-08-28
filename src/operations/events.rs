/// Поиск всех мероприятий.
pub struct EventsOperations {
    app_state: Application,
    query_params: QueryParams,
}

impl EventsOperations {}

/// Выдача данных по мериприятию.
pub struct ShowEventOperation {
}

impl ShowEventOperation {
    async fn search_events(&mut self) -> Result<(), Error> {
        let result = self.app_state.client_elastic
            .search(elasticsearch::SearchParts::Index(&["events"]))
            .from(self.calc_from() as i64)
            .size(self.calc_size() as i64)
            .body(json!({
                "query": {
                    "bool": {
                        "filter": {
                            "term": {
                                "publish": { "query": true }
                            }
                        },
                        // TODO: параметров нет тогда блока must тоже не должно быть
                        "must": {
                            "bool": {
                                "should": [
                                    {
                                        "multi_match": {
                                            "query": self.request_query(),
                                            "fields": ["title^2", "description"],
                                        }
                                    },
                                    {
                                        "range": {
                                            "start_at": {
                                                "gte": self.request_start_at,
                                                "lte": self.request_end_at
                                            }
                                        }
                                    },
                                ]
                            }
                        }
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
