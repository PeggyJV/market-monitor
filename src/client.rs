use anyhow::{anyhow, Result};
use graphql_client::{GraphQLQuery, Response};
use log::{debug, error, info, warn};
use reqwest::{header, Client as HttpClient};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use url::Url;

/// A client for interacting with The Graph API
#[derive(Debug, Clone)]
pub struct GraphClient {
    endpoint: Url,
    http: HttpClient,
}

impl GraphClient {
    /// Create a new Graph client for the given endpoint
    pub fn new(endpoint: Url) -> Result<Self> {
        info!("Creating GraphQL client for endpoint: {}", endpoint);

        // Add the API key as a header if available
        let mut headers = header::HeaderMap::new();
        if let Ok(api_key) = std::env::var("THE_GRAPH_API_KEY") {
            let auth_value = format!("Bearer {}", api_key);
            let header_value = header::HeaderValue::from_str(&auth_value)
                .map_err(|e| anyhow!("Invalid API key format: {}", e))?;
            headers.insert("Authorization", header_value);
            info!("Added API key to request headers");
        } else {
            warn!("No API key found, requests may be rate limited");
        }

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| {
                error!("Failed to build HTTP client: {}", e);
                anyhow!("Failed to build HTTP client: {}", e)
            })?;

        Ok(GraphClient { endpoint, http })
    }

    /// Execute a GraphQL query against the endpoint
    pub async fn query<Q: GraphQLQuery>(&self, variables: Q::Variables) -> Result<Q::ResponseData> {
        let body = Q::build_query(variables);
        debug!(
            "Sending GraphQL query: {}",
            serde_json::to_string(&body).unwrap_or_default()
        );

        let res = self
            .http
            .post(self.endpoint.clone())
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "GraphQL request failed with status: {}",
                res.status()
            ));
        }

        let response_body: Response<Q::ResponseData> = res
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        if let Some(errors) = response_body.errors {
            if !errors.is_empty() {
                return Err(anyhow!("GraphQL errors: {:?}", errors));
            }
        }

        response_body
            .data
            .ok_or_else(|| anyhow!("No data in GraphQL response"))
    }

    /// Execute a raw GraphQL query with the given query string and variables
    pub async fn query_raw<T: DeserializeOwned, V: Serialize>(
        &self,
        query: &str,
        variables: V,
    ) -> Result<T> {
        let body = serde_json::json!({
            "query": query,
            "variables": variables,
        });

        info!("Sending GraphQL request to: {}", self.endpoint);
        debug!(
            "Request body: {}",
            serde_json::to_string(&body).unwrap_or_default()
        );

        let res = self
            .http
            .post(self.endpoint.clone())
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send GraphQL request: {}", e);
                anyhow!("Failed to send request: {}", e)
            })?;

        info!("Received response with status: {}", res.status());
        if !res.status().is_success() {
            error!("Request failed with status code: {}", res.status());
            return Err(anyhow!(
                "GraphQL request failed with status: {}",
                res.status()
            ));
        }

        let response_text = res.text().await.map_err(|e| {
            error!("Failed to get response text: {}", e);
            anyhow!("Failed to get response text: {}", e)
        })?;

        debug!("Response body: {}", response_text);

        let response_value: Value = serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse response JSON: {}", e);
            anyhow!("Failed to parse response JSON: {}", e)
        })?;

        // Check for errors in the response
        if let Some(errors) = response_value.get("errors") {
            if errors.is_array() && !errors.as_array().unwrap().is_empty() {
                error!("GraphQL errors returned: {:?}", errors);
                return Err(anyhow!("GraphQL errors: {:?}", errors));
            }
        }

        // Try to get data from the response, if it exists
        if let Some(data) = response_value.get("data") {
            info!("Found 'data' field in response");
            match serde_json::from_value::<T>(data.clone()) {
                Ok(parsed) => {
                    info!("Successfully parsed response data");
                    return Ok(parsed);
                }
                Err(e) => {
                    warn!(
                        "Failed to parse 'data' field: {}, trying to parse entire response",
                        e
                    );
                }
            }
        }

        // If no data field or parsing failed, try parsing the entire response
        match serde_json::from_value::<T>(response_value.clone()) {
            Ok(parsed) => {
                info!("Successfully parsed flattened response");
                Ok(parsed)
            }
            Err(e) => {
                error!("Failed to parse response into requested type: {}", e);
                error!("Response structure was: {}", response_value);
                Err(anyhow!("Failed to parse response: {}", e))
            }
        }
    }
}
