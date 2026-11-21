/*
 * Linkbreakers API
 *
 * The Linkbreakers REST API: links, QR codes, workflows, visitors and analytics, as JSON over HTTPS. Send a workspace API token as `Authorization: Bearer <token>`.  ## Entry points  `GET https://api.linkbreakers.com/` and `GET https://api.linkbreakers.com/v1` answer without credentials with the API's name, version and links. This document is served at `https://api.linkbreakers.com/openapi.json`.  ## Rate limits  Every response describes where the caller stands in the IETF `RateLimit-Policy` and `RateLimit` header fields (draft-ietf-httpapi-ratelimit-headers), and in `RateLimit-Limit`, `RateLimit-Remaining`, `RateLimit-Reset` and `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` (a Unix timestamp) for clients that read those. A request past the limit gets `429 Too Many Requests` with `Retry-After` in seconds.  - Workspace API token: 720 requests per 60 seconds, per token. - Dashboard session or OAuth token: 600 requests per 60 seconds, per token. - No credentials: 300 requests per 60 seconds, per client IP.  Each plan also includes a number of API calls per billing cycle. Past it, calls answer `429` with the code `MAX_PUBLIC_API_QUERIED_REACHED` until the cycle renews or the plan changes.  ## Pagination  Every list pages with opaque cursors (AIP-158). Pass `pageSize`, then send the `nextPageToken` of each response back as `pageToken` (or its alias `cursor`) to get the next page. An empty or absent `nextPageToken` means the list is complete. Each list operation describes this in `x-pagination`.  ## Batches and background jobs  `POST /v1/links/bulk` creates up to 250 links in one transactional request; `POST /v1/links/batch` takes the same links as a bare JSON array. `POST /v1/links/bulk/jobs` accepts the same batch and answers `202 Accepted` at once with a job and a `Location` header; poll `GET /v1/jobs/{id}` until `status` is `JOB_STATUS_SUCCEEDED` (the created link ids are in `linksBulkCreate.linkIds`) or `JOB_STATUS_FAILED` (`error` says why, and nothing was created). Jobs can be read for 24 hours after they finish.  ## Paused workspaces  A free workspace whose members have not verified any email address is paused 7 days after sign-up. Its calls then answer `403` with the code `WORKSPACE_UNVERIFIED` and change nothing; QR codes and links keep working for visitors. To lift the pause, ask the workspace owner to verify their email: `POST /v1/members/email-verification` sends the verification email again, and works while the workspace is paused. `unverifiedPauseAt` on the workspace says when the pause starts.  ## Versioning and deprecation  The version is in the path: every operation lives under `/v1`. Within a version, changes are additive only: new operations, new optional request fields, new response fields and new enum values. Clients should ignore fields and enum values they do not know. Anything that would break an existing client ships under a new path version instead.  An operation is retired in two steps. First it is deprecated: it keeps working, is flagged `deprecated: true` in this document, and every response carries a `Deprecation` header field (RFC 9745) with the time it was deprecated and a `Link` with `rel=\"deprecation\"` to the policy. Then, at least 180 days later, it may be removed; that date is announced in advance in a `Sunset` header field (RFC 8594) and as `x-sunset` on the operation. The full policy: https://linkbreakers.com/help/article/api-versioning-and-deprecation-policy  A renamed operation keeps answering its former path while clients still call it, until that path's sunset at the latest: a former path no client calls for 14 days is removed. Responses on a former path carry the same `Deprecation`, `Sunset` and `Link` header fields; this document lists only the current path, and the policy lists the former paths that still answer.  ## Idempotent retries  Every POST, PUT and PATCH accepts an optional `Idempotency-Key` header (draft-ietf-httpapi-idempotency-key-header). Send a new unique value, such as a UUID, with each logical request and reuse it only to retry that request. A retry with the same key and the same request returns the first response, marked `Idempotent-Replayed: true`, instead of running again. Keys are scoped to the token and kept for 24 hours. The same key with a different request is refused with `422`, and a retry while the first request is still running gets `409` with `Retry-After`. Server errors (5xx) and refusals such as 401, 403 and 429 are not stored, so retrying them runs the request again.  ## Identifying your client  Send a `Linkbreakers-Client` request header saying which client made the call: `<client>/<version> (<details>)`, for example `my-integration/1.2.0 (python 3.12; linux)`. The official SDKs, the CLI and the MCP bridge set it for you, as `sdk-python/2.3.1 (python 3.12.4; linux)` or `cli/1.4.0 (darwin arm64)`. It is optional and changes nothing about how a request is handled; it tells us which clients and versions are in use, so we know what can be retired and what to fix first. Keep the details short and free of personal data. Browsers may send it too: it is allowed by CORS.
 *
 * The version of the OpenAPI document: 1.201.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`analytics_service_get_attribute_distribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetAttributeDistributionError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_attribute_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetAttributeMetricError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_attribute_values`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetAttributeValuesError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_breakdown`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetBreakdownError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_device_distribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetDeviceDistributionError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_events_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetEventsMetricError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_geo_breakdown`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetGeoBreakdownError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_grouped_time_series`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetGroupedTimeSeriesError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_hour_of_day_distribution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetHourOfDayDistributionError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_shared_locations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetSharedLocationsError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_step_funnel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetStepFunnelError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_top_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetTopLinksError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_traffic_split_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetTrafficSplitResultsError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_visitor_trends`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetVisitorTrendsError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_get_workspace_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceGetWorkspaceMetricError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`analytics_service_list_visitor_countries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalyticsServiceListVisitorCountriesError {
    Status429(models::ErrorResponse),
    DefaultResponse(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Returns how visitors are spread across the values of one attribute, for example how many gave each rating.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_attribute_distribution(configuration: &configuration::Configuration, get_attribute_distribution_request: models::GetAttributeDistributionRequest, idempotency_key: Option<&str>) -> Result<models::GetAttributeDistributionResponse, Error<AnalyticsServiceGetAttributeDistributionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_attribute_distribution_request = get_attribute_distribution_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/attributes/distribution", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_attribute_distribution_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAttributeDistributionResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAttributeDistributionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetAttributeDistributionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns count, average, minimum and maximum for a visitor attribute, plus how much of the audience carries it.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_attribute_metric(configuration: &configuration::Configuration, get_attribute_metric_request: models::GetAttributeMetricRequest, idempotency_key: Option<&str>) -> Result<models::GetAttributeMetricResponse, Error<AnalyticsServiceGetAttributeMetricError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_attribute_metric_request = get_attribute_metric_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/attributes/metric", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_attribute_metric_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAttributeMetricResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAttributeMetricResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetAttributeMetricError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the individual recorded values of one attribute, most recent first. Intended for free text such as comments.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_attribute_values(configuration: &configuration::Configuration, get_attribute_values_request: models::GetAttributeValuesRequest, idempotency_key: Option<&str>) -> Result<models::GetAttributeValuesResponse, Error<AnalyticsServiceGetAttributeValuesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_attribute_values_request = get_attribute_values_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/attributes/values", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_attribute_values_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAttributeValuesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAttributeValuesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetAttributeValuesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a ranked breakdown of events by any dimension (browser, platform, device type, country, city, visit type, in-app browser, in-app browser version).  Requires the `analytics:read` scope.
pub async fn analytics_service_get_breakdown(configuration: &configuration::Configuration, get_breakdown_request: models::GetBreakdownRequest, idempotency_key: Option<&str>) -> Result<models::GetBreakdownResponse, Error<AnalyticsServiceGetBreakdownError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_breakdown_request = get_breakdown_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/breakdown", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_breakdown_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBreakdownResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBreakdownResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetBreakdownError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve event distribution across device types.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_device_distribution(configuration: &configuration::Configuration, get_device_distribution_request: models::GetDeviceDistributionRequest, idempotency_key: Option<&str>) -> Result<models::GetDeviceDistributionResponse, Error<AnalyticsServiceGetDeviceDistributionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_device_distribution_request = get_device_distribution_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/devices", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_device_distribution_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetDeviceDistributionResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetDeviceDistributionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetDeviceDistributionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve aggregated event analytics for the provided time window and optional link filter.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_events_metric(configuration: &configuration::Configuration, start_date: Option<String>, end_date: Option<String>, link_id: Option<&str>, directory_id: Option<&str>, tag_id: Option<&str>, countries: Option<Vec<String>>) -> Result<models::EventsMetric, Error<AnalyticsServiceGetEventsMetricError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_start_date = start_date;
    let p_query_end_date = end_date;
    let p_query_link_id = link_id;
    let p_query_directory_id = directory_id;
    let p_query_tag_id = tag_id;
    let p_query_countries = countries;

    let uri_str = format!("{}/v1/metrics/events", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_link_id {
        req_builder = req_builder.query(&[("linkId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_directory_id {
        req_builder = req_builder.query(&[("directoryId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tag_id {
        req_builder = req_builder.query(&[("tagId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_countries {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("countries".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("countries", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EventsMetric`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EventsMetric`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetEventsMetricError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve geographic distribution of events grouped by country or city.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_geo_breakdown(configuration: &configuration::Configuration, get_geo_breakdown_request: models::GetGeoBreakdownRequest, idempotency_key: Option<&str>) -> Result<models::GetGeoBreakdownResponse, Error<AnalyticsServiceGetGeoBreakdownError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_geo_breakdown_request = get_geo_breakdown_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/geo", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_geo_breakdown_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetGeoBreakdownResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetGeoBreakdownResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetGeoBreakdownError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve time-bucketed event counts grouped by a dimension like country, city, device type, or visit type.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_grouped_time_series(configuration: &configuration::Configuration, get_grouped_time_series_request: models::GetGroupedTimeSeriesRequest, idempotency_key: Option<&str>) -> Result<models::GetGroupedTimeSeriesResponse, Error<AnalyticsServiceGetGroupedTimeSeriesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_grouped_time_series_request = get_grouped_time_series_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/timeseries", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_grouped_time_series_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetGroupedTimeSeriesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetGroupedTimeSeriesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetGroupedTimeSeriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve event counts aggregated by hour of day, in the workspace's time zone, for identifying peak activity times.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_hour_of_day_distribution(configuration: &configuration::Configuration, get_hour_of_day_distribution_request: models::GetHourOfDayDistributionRequest, idempotency_key: Option<&str>) -> Result<models::GetHourOfDayDistributionResponse, Error<AnalyticsServiceGetHourOfDayDistributionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_hour_of_day_distribution_request = get_hour_of_day_distribution_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/hours", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_hour_of_day_distribution_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetHourOfDayDistributionResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetHourOfDayDistributionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetHourOfDayDistributionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the precise positions visitors shared through a Location request workflow step, newest first, capped at 2000, with the total count. Without startDate and endDate it covers all time.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_shared_locations(configuration: &configuration::Configuration, start_date: Option<String>, end_date: Option<String>, limit: Option<i32>, link_id: Option<&str>, directory_id: Option<&str>, tag_id: Option<&str>, countries: Option<Vec<String>>) -> Result<models::GetSharedLocationsResponse, Error<AnalyticsServiceGetSharedLocationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_start_date = start_date;
    let p_query_end_date = end_date;
    let p_query_limit = limit;
    let p_query_link_id = link_id;
    let p_query_directory_id = directory_id;
    let p_query_tag_id = tag_id;
    let p_query_countries = countries;

    let uri_str = format!("{}/v1/analytics/shared-locations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_link_id {
        req_builder = req_builder.query(&[("linkId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_directory_id {
        req_builder = req_builder.query(&[("directoryId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tag_id {
        req_builder = req_builder.query(&[("tagId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_countries {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("countries".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("countries", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetSharedLocationsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetSharedLocationsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetSharedLocationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the workflow steps visitors reached, in distinct visitors, ordered from arrival to exit, with the conversion rate and drop-off at each step.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_step_funnel(configuration: &configuration::Configuration, get_step_funnel_request: models::GetStepFunnelRequest, idempotency_key: Option<&str>) -> Result<models::GetStepFunnelResponse, Error<AnalyticsServiceGetStepFunnelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_step_funnel_request = get_step_funnel_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/funnel", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_step_funnel_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetStepFunnelResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetStepFunnelResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetStepFunnelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve links ranked by event count for identifying best performing QR codes and links.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_top_links(configuration: &configuration::Configuration, get_top_links_request: models::GetTopLinksRequest, idempotency_key: Option<&str>) -> Result<models::GetTopLinksResponse, Error<AnalyticsServiceGetTopLinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_top_links_request = get_top_links_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/top-links", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_top_links_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTopLinksResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTopLinksResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetTopLinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns visits and visitors per variant of a traffic split step, leaving out bots, link previews and automated browsers, with whether there is enough data (100 per variant) and a sample ratio mismatch warning. A sticky split, the default, compares visitors against the weights. With sticky off every visit is assigned afresh, so it compares visits and one person may appear under several variants. It does not test significance or name a winner.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_traffic_split_results(configuration: &configuration::Configuration, get_traffic_split_results_request: models::GetTrafficSplitResultsRequest, idempotency_key: Option<&str>) -> Result<models::GetTrafficSplitResultsResponse, Error<AnalyticsServiceGetTrafficSplitResultsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_traffic_split_results_request = get_traffic_split_results_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/traffic-split", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_traffic_split_results_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTrafficSplitResultsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTrafficSplitResultsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetTrafficSplitResultsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve new vs returning visitor counts over time.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_visitor_trends(configuration: &configuration::Configuration, get_visitor_trends_request: models::GetVisitorTrendsRequest, idempotency_key: Option<&str>) -> Result<models::GetVisitorTrendsResponse, Error<AnalyticsServiceGetVisitorTrendsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_get_visitor_trends_request = get_visitor_trends_request;
    let p_header_idempotency_key = idempotency_key;

    let uri_str = format!("{}/v1/analytics/visitors", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_idempotency_key {
        req_builder = req_builder.header("Idempotency-Key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_get_visitor_trends_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetVisitorTrendsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetVisitorTrendsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetVisitorTrendsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return high-level workspace totals covering members, links, and activity volume.  Requires the `analytics:read` scope.
pub async fn analytics_service_get_workspace_metric(configuration: &configuration::Configuration, ) -> Result<models::WorkspaceMetric, Error<AnalyticsServiceGetWorkspaceMetricError>> {

    let uri_str = format!("{}/v1/metrics/workspace", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WorkspaceMetric`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WorkspaceMetric`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceGetWorkspaceMetricError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Every country the workspace's visits came from, all time, most visited first, without counts. Meant for country filters. For visit numbers per country, use the geo breakdown.  Requires the `analytics:read` scope.
pub async fn analytics_service_list_visitor_countries(configuration: &configuration::Configuration, ) -> Result<models::ListVisitorCountriesResponse, Error<AnalyticsServiceListVisitorCountriesError>> {

    let uri_str = format!("{}/v1/analytics/visitor-countries", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListVisitorCountriesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListVisitorCountriesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AnalyticsServiceListVisitorCountriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

