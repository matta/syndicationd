use tower_http::trace::HttpMakeClassifier;
use tracing::Level;

#[derive(Clone)]
pub struct MakeSpan;

impl<B> tower_http::trace::MakeSpan<B> for MakeSpan {
    fn make_span(&mut self, request: &axum::http::Request<B>) -> tracing::Span {
        tracing::span!(
            Level::INFO,
            "http",
            method = %request.method(),
            uri = %request.uri(),
        )
    }
}

pub fn layer() -> tower_http::trace::TraceLayer<HttpMakeClassifier, MakeSpan> {
    tower_http::trace::TraceLayer::new_for_http().make_span_with(MakeSpan)
}
