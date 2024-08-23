// Глобавльный поиск по всему контенту приложения.
pub async fn global_search(
    Query(conditions): Query<QuerySearch>
) -> impl IntoResponse {
    let result = ResultSearch {
        total: 2,
        results: vec![
            (
                String::from("https://example.com/article1"),
                String::from("Example Article 1"),
                String::from("This is an example article."),
            ),
            (
                String::from("https://example.com/article2"),
                String::from("Example Article 2"),
                String::from("This is another example article."),
            ),
        ],
    };
    Json(result)
}
