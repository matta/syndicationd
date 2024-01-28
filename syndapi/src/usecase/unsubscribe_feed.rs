use std::sync::Arc;

use crate::{
    persistence::{self, Datastore},
    principal::Principal,
    usecase::{Input, Output},
};

use super::{authorize::Unauthorized, Usecase};

pub struct UnsubscribeFeed {
    pub datastore: Arc<dyn Datastore>,
}

pub struct UnsubscribeFeedInput {
    pub url: String,
}

pub struct UnsubscribeFeedOutput {}

impl Usecase for UnsubscribeFeed {
    type Input = UnsubscribeFeedInput;

    type Output = UnsubscribeFeedOutput;

    type Error = anyhow::Error;

    fn new(make: &super::MakeUsecase) -> Self {
        Self {
            datastore: make.datastore.clone(),
        }
    }

    async fn authorize(
        &self,
        principal: Principal,
        _input: &UnsubscribeFeedInput,
    ) -> Result<Principal, Unauthorized> {
        Ok(principal)
    }

    async fn usecase(
        &self,
        Input {
            principal,
            input: UnsubscribeFeedInput { url },
            ..
        }: Input<Self::Input>,
    ) -> Result<Output<Self::Output>, super::Error<Self::Error>> {
        tracing::debug!("Unsubscribe feed: {url}");

        self.datastore
            .delete_feed_subscription(persistence::types::FeedSubscription {
                user_id: principal.user_id().unwrap().to_owned(),
                url,
            })
            .await?;

        Ok(Output {
            output: UnsubscribeFeedOutput {},
        })
    }
}