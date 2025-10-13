use super::{ProviderBase, ProviderConfig};
use crate::scraper::{
    EpisodeMetadata, ExternalIds, MediaDetails, MediaSearchResult, MetadataProvider, Result,
    ScraperError, TvMetadata, TvSearchResult,
};
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;

const TVDB_API_URL: &str = "https://api4.thetvdb.com/v4";

/// TVDB Provider
pub struct TvdbProvider {
    base: ProviderBase,
    api_key: String,
    token: parking_lot::RwLock<Option<String>>,
}

impl TvdbProvider {
    /// Create a new TVDB provider
    pub fn new(api_key: impl Into<String>, cache: Arc<crate::scraper::ScraperCache>) -> Self {
        let api_key = api_key.into();
        let config = ProviderConfig::new(TVDB_API_URL)
            .with_api_key(api_key.clone())
            .with_cache_ttl(86400); // 24 hours

        Self {
            base: ProviderBase::new(config, cache),
            api_key,
            token: parking_lot::RwLock::new(None),
        }
    }

    /// Get authentication token
    async fn get_token(&self) -> Result<String> {
        // Check if token already exists
        {
            let token = self.token.read();
            if let Some(ref t) = *token {
                return Ok(t.clone());
            }
        }

        // Login to get new token
        let login_url = format!("{TVDB_API_URL}/login");
        let body = serde_json::json!({
            "apikey": self.api_key
        });

        let response = self
            .base
            .client
            .post(&login_url)
            .json(&body)
            .send()
            .await
            .map_err(ScraperError::Network)?;

        if !response.status().is_success() {
            return Err(ScraperError::Api {
                status: response.status().as_u16(),
                message: "Failed to authenticate with TVDB".to_string(),
            });
        }

        let login_response: TvdbLoginResponse = response.json().await.map_err(|e| {
            ScraperError::Parse(format!("Failed to parse TVDB login response: {e}"))
        })?;

        let token = login_response.data.token;
        *self.token.write() = Some(token.clone());

        Ok(token)
    }

    /// Execute TVDB API request
    async fn request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let token = self.get_token().await?;
        let url = format!("{TVDB_API_URL}{endpoint}");

        let response = self
            .base
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(ScraperError::Network)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(ScraperError::Api {
                status,
                message: text,
            });
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ScraperError::Parse(format!("Failed to parse TVDB response: {e}")))
    }

    // Private helper methods
    async fn search_tv_internal(
        &self,
        query: &str,
        _year: Option<i32>,
    ) -> Result<Vec<TvSearchResult>> {
        let encoded_query = urlencoding::encode(query);
        let endpoint = format!("/search?query={encoded_query}&type=series");

        let response: TvdbSearchResponse = self.request(&endpoint).await?;

        Ok(response
            .data
            .into_iter()
            .map(|series| TvSearchResult {
                id: series.tvdb_id.to_string(),
                name: series.name,
                original_name: series.original_name,
                first_air_date: series.first_aired,
                poster_path: series.image_url,
                overview: series.overview,
                vote_average: None,
                provider: "tvdb".to_string(),
            })
            .collect())
    }

    async fn get_tv_details_internal(&self, id: &str) -> Result<TvMetadata> {
        let endpoint = format!("/series/{id}/extended");
        let response: TvdbSeriesResponse = self.request(&endpoint).await?;
        let series = response.data;

        Ok(TvMetadata {
            id: series.id.to_string(),
            name: series.name,
            original_name: None,
            first_air_date: series.first_aired,
            last_air_date: series.last_aired,
            overview: series.overview,
            poster_path: series.image,
            backdrop_path: None,
            vote_average: series.score.map(f64::from),
            vote_count: None,
            genres: series
                .genres
                .unwrap_or_default()
                .into_iter()
                .map(|g| g.name)
                .collect(),
            number_of_seasons: None,
            number_of_episodes: None,
            episode_run_time: vec![],
            status: Some(series.status.name),
            original_language: series.original_language,
            production_companies: vec![],
            provider: "tvdb".to_string(),
            external_ids: ExternalIds {
                tvdb_id: Some(series.id.to_string()),
                ..Default::default()
            },
        })
    }
}

#[async_trait]
impl MetadataProvider for TvdbProvider {
    fn name(&self) -> &'static str {
        "tvdb"
    }

    fn requires_api_key(&self) -> bool {
        true
    }

    async fn search(&self, query: &str, year: Option<i32>) -> Result<Vec<MediaSearchResult>> {
        // TVDB only supports TV show searches
        let tv_shows = self.search_tv_internal(query, year).await?;
        Ok(tv_shows.into_iter().map(MediaSearchResult::Tv).collect())
    }

    async fn get_details(&self, result: &MediaSearchResult) -> Result<MediaDetails> {
        match result {
            MediaSearchResult::Tv(t) => self
                .get_tv_details_internal(&t.id)
                .await
                .map(MediaDetails::Tv),
            MediaSearchResult::Movie(_) => Err(ScraperError::Config(
                "TVDB does not support movies".to_string(),
            )),
            MediaSearchResult::Anime(_) => Err(ScraperError::Config(
                "TVDB does not support anime".to_string(),
            )),
        }
    }

    async fn get_episode_details(
        &self,
        series_id: &str,
        season: i32,
        episode: i32,
    ) -> Result<EpisodeMetadata> {
        // TVDB API v4 requires getting season ID first, then episode
        let season_endpoint = format!("/series/{series_id}/episodes/default?season={season}");
        let season_response: TvdbEpisodesResponse = self.request(&season_endpoint).await?;

        let ep = season_response
            .data
            .episodes
            .into_iter()
            .find(|e| e.number == episode)
            .ok_or_else(|| {
                ScraperError::NotFound(format!("Episode {episode} not found in season {season}"))
            })?;

        Ok(EpisodeMetadata {
            id: ep.id.to_string(),
            name: ep.name,
            season_number: ep.season_number,
            episode_number: ep.number,
            air_date: ep.aired,
            overview: ep.overview,
            still_path: ep.image,
            runtime: ep.runtime,
            vote_average: None,
            provider: "tvdb".to_string(),
        })
    }
}

// TVDB API Response Types
#[derive(Debug, Deserialize)]
struct TvdbLoginResponse {
    data: TvdbTokenData,
}

#[derive(Debug, Deserialize)]
struct TvdbTokenData {
    token: String,
}

#[derive(Debug, Deserialize)]
struct TvdbSearchResponse {
    data: Vec<TvdbSearchResult>,
}

#[derive(Debug, Deserialize)]
struct TvdbSearchResult {
    tvdb_id: String,
    name: String,
    original_name: Option<String>,
    first_aired: Option<String>,
    image_url: Option<String>,
    overview: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TvdbSeriesResponse {
    data: TvdbSeriesDetails,
}

#[derive(Debug, Deserialize)]
struct TvdbSeriesDetails {
    id: i64,
    name: String,
    overview: Option<String>,
    first_aired: Option<String>,
    last_aired: Option<String>,
    image: Option<String>,
    score: Option<f32>,
    status: TvdbStatus,
    original_language: Option<String>,
    genres: Option<Vec<TvdbGenre>>,
}

#[derive(Debug, Deserialize)]
struct TvdbStatus {
    name: String,
}

#[derive(Debug, Deserialize)]
struct TvdbGenre {
    name: String,
}

#[derive(Debug, Deserialize)]
struct TvdbEpisodesResponse {
    data: TvdbSeasonData,
}

#[derive(Debug, Deserialize)]
struct TvdbSeasonData {
    episodes: Vec<TvdbEpisode>,
}

#[derive(Debug, Deserialize)]
struct TvdbEpisode {
    id: i64,
    name: String,
    #[serde(rename = "seasonNumber")]
    season_number: i32,
    number: i32,
    aired: Option<String>,
    overview: Option<String>,
    image: Option<String>,
    runtime: Option<i32>,
}
