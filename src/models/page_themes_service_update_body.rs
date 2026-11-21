/*
 * Linkbreakers API
 *
 * The Linkbreakers REST API: links, QR codes, workflows, visitors and analytics, as JSON over HTTPS. Send a workspace API token as `Authorization: Bearer <token>`.  ## Entry points  `GET https://api.linkbreakers.com/` and `GET https://api.linkbreakers.com/v1` answer without credentials with the API's name, version and links. This document is served at `https://api.linkbreakers.com/openapi.json`.  ## Rate limits  Every response describes where the caller stands in the IETF `RateLimit-Policy` and `RateLimit` header fields (draft-ietf-httpapi-ratelimit-headers), and in `RateLimit-Limit`, `RateLimit-Remaining`, `RateLimit-Reset` and `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` (a Unix timestamp) for clients that read those. A request past the limit gets `429 Too Many Requests` with `Retry-After` in seconds.  - Workspace API token: 720 requests per 60 seconds, per token. - Dashboard session or OAuth token: 600 requests per 60 seconds, per token. - No credentials: 300 requests per 60 seconds, per client IP.  Each plan also includes a number of API calls per billing cycle. Past it, calls answer `429` with the code `MAX_PUBLIC_API_QUERIED_REACHED` until the cycle renews or the plan changes.  ## Pagination  Every list pages with opaque cursors (AIP-158). Pass `pageSize`, then send the `nextPageToken` of each response back as `pageToken` (or its alias `cursor`) to get the next page. An empty or absent `nextPageToken` means the list is complete. Each list operation describes this in `x-pagination`.  ## Batches and background jobs  `POST /v1/links/bulk` creates up to 250 links in one transactional request; `POST /v1/links/batch` takes the same links as a bare JSON array. `POST /v1/links/bulk/jobs` accepts the same batch and answers `202 Accepted` at once with a job and a `Location` header; poll `GET /v1/jobs/{id}` until `status` is `JOB_STATUS_SUCCEEDED` (the created link ids are in `linksBulkCreate.linkIds`) or `JOB_STATUS_FAILED` (`error` says why, and nothing was created). Jobs can be read for 24 hours after they finish.  ## Paused workspaces  A free workspace whose members have not verified any email address is paused 7 days after sign-up. Its calls then answer `403` with the code `WORKSPACE_UNVERIFIED` and change nothing; QR codes and links keep working for visitors. To lift the pause, ask the workspace owner to verify their email: `POST /v1/members/email-verification` sends the verification email again, and works while the workspace is paused. `unverifiedPauseAt` on the workspace says when the pause starts.  ## Versioning and deprecation  The version is in the path: every operation lives under `/v1`. Within a version, changes are additive only: new operations, new optional request fields, new response fields and new enum values. Clients should ignore fields and enum values they do not know. Anything that would break an existing client ships under a new path version instead.  An operation is retired in two steps. First it is deprecated: it keeps working, is flagged `deprecated: true` in this document, and every response carries a `Deprecation` header field (RFC 9745) with the time it was deprecated and a `Link` with `rel=\"deprecation\"` to the policy. Then, at least 180 days later, it may be removed; that date is announced in advance in a `Sunset` header field (RFC 8594) and as `x-sunset` on the operation. The full policy: https://linkbreakers.com/help/article/api-versioning-and-deprecation-policy  A renamed operation keeps answering its former path while clients still call it, until that path's sunset at the latest: a former path no client calls for 14 days is removed. Responses on a former path carry the same `Deprecation`, `Sunset` and `Link` header fields; this document lists only the current path, and the policy lists the former paths that still answer.  ## Idempotent retries  Every POST, PUT and PATCH accepts an optional `Idempotency-Key` header (draft-ietf-httpapi-idempotency-key-header). Send a new unique value, such as a UUID, with each logical request and reuse it only to retry that request. A retry with the same key and the same request returns the first response, marked `Idempotent-Replayed: true`, instead of running again. Keys are scoped to the token and kept for 24 hours. The same key with a different request is refused with `422`, and a retry while the first request is still running gets `409` with `Retry-After`. Server errors (5xx) and refusals such as 401, 403 and 429 are not stored, so retrying them runs the request again.  ## Identifying your client  Send a `Linkbreakers-Client` request header saying which client made the call: `<client>/<version> (<details>)`, for example `my-integration/1.2.0 (python 3.12; linux)`. The official SDKs, the CLI and the MCP bridge set it for you, as `sdk-python/2.3.1 (python 3.12.4; linux)` or `cli/1.4.0 (darwin arm64)`. It is optional and changes nothing about how a request is handled; it tells us which clients and versions are in use, so we know what can be retired and what to fix first. Keep the details short and free of personal data. Browsers may send it too: it is allowed by CORS.
 *
 * The version of the OpenAPI document: 1.203.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageThemesServiceUpdateBody {
    #[serde(rename = "badge", skip_serializing_if = "Option::is_none")]
    pub badge: Option<Box<models::MediaSelection>>,
    #[serde(rename = "badgeEnabled", skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    #[serde(rename = "badgeIconColor", skip_serializing_if = "Option::is_none")]
    pub badge_icon_color: Option<Box<models::ThemeColor>>,
    #[serde(rename = "badgeShape", skip_serializing_if = "Option::is_none")]
    pub badge_shape: Option<BadgeShape>,
    #[serde(rename = "bodyFont", skip_serializing_if = "Option::is_none")]
    pub body_font: Option<Box<models::FontConfig>>,
    #[serde(rename = "canvasColor", skip_serializing_if = "Option::is_none")]
    pub canvas_color: Option<Box<models::ThemeColor>>,
    #[serde(rename = "canvasCover", skip_serializing_if = "Option::is_none")]
    pub canvas_cover: Option<Box<models::MediaSelection>>,
    #[serde(rename = "canvasCoverMode", skip_serializing_if = "Option::is_none")]
    pub canvas_cover_mode: Option<CanvasCoverMode>,
    /// A texture over the page background: NONE, BRUSHED (paint streaks) or GRAIN (film grain).
    #[serde(rename = "canvasTexture", skip_serializing_if = "Option::is_none")]
    pub canvas_texture: Option<CanvasTexture>,
    /// How a choice question lays out its answers: AUTO (pills or tiles) or ROWS (full-width rows, indicator on the right).
    #[serde(rename = "choiceLayout", skip_serializing_if = "Option::is_none")]
    pub choice_layout: Option<ChoiceLayout>,
    #[serde(rename = "containerConfig", skip_serializing_if = "Option::is_none")]
    pub container_config: Option<Box<models::ContainerConfig>>,
    #[serde(rename = "containerEnabled", skip_serializing_if = "Option::is_none")]
    pub container_enabled: Option<bool>,
    #[serde(rename = "elementColor", skip_serializing_if = "Option::is_none")]
    pub element_color: Option<Box<models::ThemeColor>>,
    #[serde(rename = "elementFont", skip_serializing_if = "Option::is_none")]
    pub element_font: Option<Box<models::FontConfig>>,
    /// The look of each element job (SURFACE, SELECTED, ACTION, QUIET). Each job sent replaces that job; one sent with an empty style returns to what it follows; jobs not sent are kept. A job not listed follows the shape, the colours, or the look it always had.
    #[serde(rename = "elementStyles", skip_serializing_if = "Option::is_none")]
    pub element_styles: Option<Vec<models::PageThemeElementStyleOverride>>,
    /// Footer logo media selection: id of any image of at most 3 MB from the workspace's media library, or delete to remove the image.
    #[serde(rename = "footerLogo", skip_serializing_if = "Option::is_none")]
    pub footer_logo: Option<Box<models::MediaSelection>>,
    /// Whether to show the footer logo, a small company or brand logo centered at the bottom of every page using this theme (false hides it but keeps the image).
    #[serde(rename = "footerLogoEnabled", skip_serializing_if = "Option::is_none")]
    pub footer_logo_enabled: Option<bool>,
    /// How titles and headline questions are aligned. Unset: centered titles, questions as the question style draws them. UNSPECIFIED on an update clears it.
    #[serde(rename = "headingAlign", skip_serializing_if = "Option::is_none")]
    pub heading_align: Option<HeadingAlign>,
    /// How a form's questions are written: LABEL (small, in the text style) or HEADLINE (centered, in the title style).
    #[serde(rename = "questionStyle", skip_serializing_if = "Option::is_none")]
    pub question_style: Option<QuestionStyle>,
    /// The radius and border every job follows unless it has its own.
    #[serde(rename = "shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<Box<models::PageThemeShape>>,
    /// Small text: hints, counters, consent. Unset: the body text, smaller and muted.
    #[serde(rename = "smallText", skip_serializing_if = "Option::is_none")]
    pub small_text: Option<Box<models::PageThemeSmallText>>,
    /// The text colour headings, body and small text follow when their own font has none. On an update, a colour without colours clears it.
    #[serde(rename = "textColor", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Box<models::ThemeColor>>,
    #[serde(rename = "titleFont", skip_serializing_if = "Option::is_none")]
    pub title_font: Option<Box<models::FontConfig>>,
    /// How visitor pages pace their content: CLASSIC shows a whole form at once and a chat as a transcript, FOCUSED shows one question per screen and a chat as a short fading trail of its newest messages.
    #[serde(rename = "visitorExperience", skip_serializing_if = "Option::is_none")]
    pub visitor_experience: Option<VisitorExperience>,
}

impl PageThemesServiceUpdateBody {
    pub fn new() -> PageThemesServiceUpdateBody {
        PageThemesServiceUpdateBody {
            badge: None,
            badge_enabled: None,
            badge_icon_color: None,
            badge_shape: None,
            body_font: None,
            canvas_color: None,
            canvas_cover: None,
            canvas_cover_mode: None,
            canvas_texture: None,
            choice_layout: None,
            container_config: None,
            container_enabled: None,
            element_color: None,
            element_font: None,
            element_styles: None,
            footer_logo: None,
            footer_logo_enabled: None,
            heading_align: None,
            question_style: None,
            shape: None,
            small_text: None,
            text_color: None,
            title_font: None,
            visitor_experience: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BadgeShape {
    #[serde(rename = "PAGE_THEME_BADGE_SHAPE_UNSPECIFIED")]
    PageThemeBadgeShapeUnspecified,
    #[serde(rename = "PAGE_THEME_BADGE_SHAPE_CIRCLE")]
    PageThemeBadgeShapeCircle,
    #[serde(rename = "PAGE_THEME_BADGE_SHAPE_SQUARE")]
    PageThemeBadgeShapeSquare,
    #[serde(rename = "PAGE_THEME_BADGE_SHAPE_ROUNDED")]
    PageThemeBadgeShapeRounded,
}

impl Default for BadgeShape {
    fn default() -> BadgeShape {
        Self::PageThemeBadgeShapeUnspecified
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CanvasCoverMode {
    #[serde(rename = "PAGE_THEME_BACKGROUND_COVER_MODE_UNSPECIFIED")]
    PageThemeBackgroundCoverModeUnspecified,
    #[serde(rename = "PAGE_THEME_BACKGROUND_COVER_MODE_FULL")]
    PageThemeBackgroundCoverModeFull,
    #[serde(rename = "PAGE_THEME_BACKGROUND_COVER_MODE_PROGRESSIVE_TOP")]
    PageThemeBackgroundCoverModeProgressiveTop,
    #[serde(rename = "PAGE_THEME_BACKGROUND_COVER_MODE_PROGRESSIVE_BOTTOM")]
    PageThemeBackgroundCoverModeProgressiveBottom,
}

impl Default for CanvasCoverMode {
    fn default() -> CanvasCoverMode {
        Self::PageThemeBackgroundCoverModeUnspecified
    }
}
/// A texture over the page background: NONE, BRUSHED (paint streaks) or GRAIN (film grain).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CanvasTexture {
    #[serde(rename = "PAGE_THEME_CANVAS_TEXTURE_UNSPECIFIED")]
    PageThemeCanvasTextureUnspecified,
    #[serde(rename = "PAGE_THEME_CANVAS_TEXTURE_NONE")]
    PageThemeCanvasTextureNone,
    #[serde(rename = "PAGE_THEME_CANVAS_TEXTURE_BRUSHED")]
    PageThemeCanvasTextureBrushed,
    #[serde(rename = "PAGE_THEME_CANVAS_TEXTURE_GRAIN")]
    PageThemeCanvasTextureGrain,
}

impl Default for CanvasTexture {
    fn default() -> CanvasTexture {
        Self::PageThemeCanvasTextureUnspecified
    }
}
/// How a choice question lays out its answers: AUTO (pills or tiles) or ROWS (full-width rows, indicator on the right).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChoiceLayout {
    #[serde(rename = "PAGE_THEME_CHOICE_LAYOUT_UNSPECIFIED")]
    PageThemeChoiceLayoutUnspecified,
    #[serde(rename = "PAGE_THEME_CHOICE_LAYOUT_AUTO")]
    PageThemeChoiceLayoutAuto,
    #[serde(rename = "PAGE_THEME_CHOICE_LAYOUT_ROWS")]
    PageThemeChoiceLayoutRows,
}

impl Default for ChoiceLayout {
    fn default() -> ChoiceLayout {
        Self::PageThemeChoiceLayoutUnspecified
    }
}
/// How titles and headline questions are aligned. Unset: centered titles, questions as the question style draws them. UNSPECIFIED on an update clears it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HeadingAlign {
    #[serde(rename = "PAGE_THEME_TEXT_ALIGN_UNSPECIFIED")]
    PageThemeTextAlignUnspecified,
    #[serde(rename = "PAGE_THEME_TEXT_ALIGN_CENTER")]
    PageThemeTextAlignCenter,
    #[serde(rename = "PAGE_THEME_TEXT_ALIGN_LEFT")]
    PageThemeTextAlignLeft,
}

impl Default for HeadingAlign {
    fn default() -> HeadingAlign {
        Self::PageThemeTextAlignUnspecified
    }
}
/// How a form's questions are written: LABEL (small, in the text style) or HEADLINE (centered, in the title style).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuestionStyle {
    #[serde(rename = "PAGE_THEME_QUESTION_STYLE_UNSPECIFIED")]
    PageThemeQuestionStyleUnspecified,
    #[serde(rename = "PAGE_THEME_QUESTION_STYLE_LABEL")]
    PageThemeQuestionStyleLabel,
    #[serde(rename = "PAGE_THEME_QUESTION_STYLE_HEADLINE")]
    PageThemeQuestionStyleHeadline,
}

impl Default for QuestionStyle {
    fn default() -> QuestionStyle {
        Self::PageThemeQuestionStyleUnspecified
    }
}
/// How visitor pages pace their content: CLASSIC shows a whole form at once and a chat as a transcript, FOCUSED shows one question per screen and a chat as a short fading trail of its newest messages.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VisitorExperience {
    #[serde(rename = "PAGE_THEME_VISITOR_EXPERIENCE_UNSPECIFIED")]
    PageThemeVisitorExperienceUnspecified,
    #[serde(rename = "PAGE_THEME_VISITOR_EXPERIENCE_CLASSIC")]
    PageThemeVisitorExperienceClassic,
    #[serde(rename = "PAGE_THEME_VISITOR_EXPERIENCE_FOCUSED")]
    PageThemeVisitorExperienceFocused,
}

impl Default for VisitorExperience {
    fn default() -> VisitorExperience {
        Self::PageThemeVisitorExperienceUnspecified
    }
}

