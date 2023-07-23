use crate::resources::RateCache;
use anyhow::Result;
use kube::runtime::controller::Action;
use log::info;
use std::sync::Arc;
use thiserror::Error;
use tokio::time::Duration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create ConfigMap: {0}")]
    ConfigMapCreationFailed(#[source] kube::Error),
    #[error("MissingObjectKey: {0}")]
    MissingObjectKey(&'static str),
}

/// The reconciler that will be called when either object change
pub async fn reconciler(g: Arc<RateCache>, _ctx: Arc<()>) -> Result<Action, Error> {
    info!("got object {:?}", g.metadata.name);
    Ok(Action::requeue(Duration::from_secs(300)))
}
/// an error handler that will be called when the reconciler fails with access to both the
/// object that caused the failure and the actual error
pub fn error_policy(obj: Arc<RateCache>, _error: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(60))
}
