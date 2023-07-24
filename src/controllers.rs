use crate::resources::RateCache;
use anyhow::Result;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{runtime::controller::Action, Api, client, Client};

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
pub async fn reconciler(g: Arc<RateCache>, _ctx: Arc<()> ) -> Result<Action, Error> {
  
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


async fn create_redis_deployment(name: String , client:kube::Client)  {

    let  deployment = Api::<Deployment>::all(client.clone());
    let image_name = name+"-r8s";

    // create redis deployment with redis alpine image
    let mut  dep =  Deployment::default();
    // set deployment namespace
    dep.metadata.namespace = Some("rates".to_string());


    dep.spec = Some(k8s_openapi::api::apps::v1::DeploymentSpec {
        replicas: Some(1),
        selector: k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
            match_labels: Some(std::collections::BTreeMap::new()),
            match_expressions: None,
        },
        template: k8s_openapi::api::core::v1::PodTemplateSpec {
            metadata: Some(k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                labels: Some(std::collections::BTreeMap::new()),
                ..Default::default()
            }),
            spec: Some(k8s_openapi::api::core::v1::PodSpec {
                containers: vec![k8s_openapi::api::core::v1::Container {
                    ports: Some(vec![k8s_openapi::api::core::v1::ContainerPort{
                        container_port: 6379,
                        protocol: Some("TCP".to_string()),
                        ..Default::default()

                    }]),
                    name: image_name.clone().to_string(),
                    image: Some("redis:alpine".to_string()),
                
                    ..Default::default()
                }],
                ..Default::default()
            }),
        },
        ..Default::default()
    });

    let dep = deployment.create(&Default::default(), &dep).await;

    match dep {
        Ok(o) => info!("created {:?}", o),
        Err(e) => info!("create failed: {}", e),
    }
  
}
