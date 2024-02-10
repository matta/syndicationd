use std::{fmt::Debug, time::Duration};

use anyhow::anyhow;
use graphql_client::{GraphQLQuery, Response};
use reqwest::header::{self, HeaderValue};
use serde::{de::DeserializeOwned, Serialize};
use synd_o11y::opentelemetry::extension::*;
use tracing::{error, Span};
use url::Url;

use crate::{auth::Credential, config, types};

use self::query::subscription::SubscriptionOutput;

mod scalar;
pub use scalar::*;
pub mod mutation;
pub mod payload;
pub mod query;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    credential: Option<HeaderValue>,
    endpoint: Url,
}

impl Client {
    pub fn new(endpoint: Url) -> anyhow::Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .user_agent(config::USER_AGENT)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client,
            endpoint,
            credential: None,
        })
    }

    pub fn set_credential(&mut self, auth: Credential) {
        let mut token = HeaderValue::try_from(match auth {
            Credential::Github { access_token } => format!("github {access_token}"),
        })
        .unwrap();
        token.set_sensitive(true);
        self.credential = Some(token);
    }

    #[tracing::instrument(skip(self))]
    pub async fn fetch_subscription(
        &self,
        after: Option<String>,
        first: Option<i64>,
    ) -> anyhow::Result<SubscriptionOutput> {
        let var = query::subscription::Variables { after, first };
        let request = query::Subscription::build_query(var);
        let response: query::subscription::ResponseData = self.request(&request).await?;
        Ok(response.output)
    }

    #[tracing::instrument(skip(self))]
    pub async fn subscribe_feed(&self, url: String) -> anyhow::Result<types::Feed> {
        let var = mutation::subscribe_feed::Variables {
            input: mutation::subscribe_feed::SubscribeFeedInput { url },
        };
        let request = mutation::SubscribeFeed::build_query(var);
        let response: mutation::subscribe_feed::ResponseData = self.request(&request).await?;

        match response.subscribe_feed {
            mutation::subscribe_feed::SubscribeFeedSubscribeFeed::SubscribeFeedSuccess(success) => {
                Ok(types::Feed::from(success.feed))
            }
            mutation::subscribe_feed::SubscribeFeedSubscribeFeed::SubscribeFeedError(err) => {
                Err(anyhow!("Failed to mutate subscribe_feed {err:?}"))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn unsubscribe_feed(&self, url: String) -> anyhow::Result<()> {
        let var = mutation::unsubscribe_feed::Variables {
            input: mutation::unsubscribe_feed::UnsubscribeFeedInput { url },
        };
        let request = mutation::UnsubscribeFeed::build_query(var);
        let response: mutation::unsubscribe_feed::ResponseData = self.request(&request).await?;

        match response.unsubscribe_feed {
            mutation::unsubscribe_feed::UnsubscribeFeedUnsubscribeFeed::UnsubscribeFeedSuccess(
                _,
            ) => Ok(()),
            mutation::unsubscribe_feed::UnsubscribeFeedUnsubscribeFeed::UnsubscribeFeedError(
                err,
            ) => Err(anyhow!("Failed to mutate unsubscribe_feed {err:?}")),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn fetch_entries(
        &self,
        after: Option<String>,
        first: i64,
    ) -> anyhow::Result<payload::FetchEntriesPayload> {
        tracing::debug!("Fetch entries...");

        let var = query::entries::Variables { after, first };
        let request = query::Entries::build_query(var);
        let response: query::entries::ResponseData = self.request(&request).await?;

        tracing::debug!("Got response");

        Ok(response.output.into())
    }

    #[tracing::instrument(skip_all, err(Display))]
    async fn request<Body, ResponseData>(&self, body: &Body) -> anyhow::Result<ResponseData>
    where
        Body: Serialize + Debug + ?Sized,
        ResponseData: DeserializeOwned + Debug,
    {
        let mut request = self
            .client
            .post(self.endpoint.clone())
            .header(
                header::AUTHORIZATION,
                self.credential
                    .as_ref()
                    .expect("Credential not configured. this is a BUG")
                    .clone(),
            )
            .json(body)
            .build()?;

        synd_o11y::opentelemetry::http::inject_with_baggage(
            &Span::current().context(),
            request.headers_mut(),
            std::iter::once(synd_o11y::request_id_key_value()),
        );

        tracing::debug!(url = request.url().as_str(), "Send request");

        let response: Response<ResponseData> = self
            .client
            .execute(request)
            .await?
            .error_for_status()?
            .json()
            .await?;

        match (response.data, response.errors) {
            (_, Some(errs)) if !errs.is_empty() => {
                for err in errs {
                    error!("{err:?}");
                }
                Err(anyhow::anyhow!("failed to request synd api"))
            }
            (Some(data), _) => Ok(data),
            _ => Err(anyhow::anyhow!("unexpected response",)),
        }
    }
}