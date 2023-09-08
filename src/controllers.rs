use crate::resources::RateCache;
use anyhow::Result;
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container, ContainerPort, PodSpec, PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::{
    api::PostParams, client, core::ObjectMeta, runtime::controller::Action, Api, Client,
    ResourceExt,
};
use serde_json::json;
use std::collections::BTreeMap;

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
    let client = Client::try_default().await.unwrap();
    let deploy_name = Arc::clone(&g);
    info!("creating cache");
    create_redis_deployment(deploy_name.metadata.name.clone().unwrap(), client).await;

    Ok(Action::requeue(Duration::from_secs(300)))
}
/// an error handler that will be called when the reconciler fails with access to both the
/// object that caused the failure and the actual error
pub fn error_policy(obj: Arc<RateCache>, _error: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(60))
}

async fn create_redis_deployment(name: String, client: kube::Client) {
    let deployment: Api<Deployment> = Api::namespaced(client.clone(), "rates");

    let deployment_name = format!("{}-r8tes", name);

    let dep: Deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(deployment_name.to_owned()),
            namespace: Some("rates".to_string()), // Set your desired namespace
            ..ObjectMeta::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(1),
            selector: LabelSelector {
                match_labels: Some({
                    let mut labels = BTreeMap::new();
                    labels.insert("app".to_owned(), "my-app".to_owned()); // Match with the labels in the PodTemplateSpec
                    labels
                }),
                ..LabelSelector::default()
            },
            template: PodTemplateSpec {
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.to_owned(),
                        image: Some("redis:alpine".to_owned()), // Redis Alpine image
                        ports: Some(vec![ContainerPort {
                            container_port: 6379, // Redis default port
                            ..ContainerPort::default()
                        }]),
                        ..Container::default()
                    }],
                    ..PodSpec::default()
                }),
                metadata: Some(ObjectMeta {
                    labels: Some({
                        let mut labels = BTreeMap::new();
                        labels.insert("app".to_owned(), "my-app".to_owned()); // Set the same labels as in spec.selector
                        labels
                    }),
                    ..ObjectMeta::default()
                }),
                ..PodTemplateSpec::default()
            },
            ..DeploymentSpec::default()
        }),
        ..Deployment::default()
    };
    let dep = deployment.create(&PostParams::default(), &dep).await;
    match dep {
        Ok(o) => info!("created {:?}", o),
        Err(e) => info!("create failed: {}", e),
    }
}
