use duckduckgo::{browser::BrowserBuilder, response::LiteSearchResult, user_agents::get};

/// Search the web using DuckDuckGo
///
/// # Arguments
/// * `query` - The search query string
///
/// # Returns
/// A Result containing the search results or an error
pub async fn search(query: &str) -> Result<Vec<LiteSearchResult>, Box<dyn std::error::Error>> {
   assert!(!query.is_empty());
   let browser = BrowserBuilder::default();
   let browser = browser.build().expect("Failed to build browser");

   let limit = None;

   let results = browser
      .lite_search(query, "us", limit, get("firefox").unwrap())
      .await
      .expect("Failed to search");

   Ok(results.into_iter().collect())
}
