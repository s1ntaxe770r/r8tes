use std::sync::Arc;

use futures::StreamExt;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    runtime::{watcher, Controller},
    Api, Client, CustomResourceExt,
};
use log::info;
use log::warn;
use r8tes::controllers::{error_policy, reconciler};
use r8tes::resources::RateCache;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = match Client::try_default().await {
        Ok(kubeconfig) => kubeconfig,
        Err(_e) => panic!("{}", "unable to locate kubeconfig "),
    };

    // Generate the CRD
    let custom_resource: CustomResourceDefinition = RateCache::crd();

    let crd: Api<CustomResourceDefinition> = Api::all(client.clone());
    // Create the CRD and ignore results as it will fail if it already exists
    let _ = crd.create(&Default::default(), &custom_resource).await;

    let rc = Api::<RateCache>::all(client.clone());

    let context = Arc::new(());
    info!("starting controller");
    // intialize controller
    Controller::new(rc.clone(), watcher::Config::default())
        .owns(rc, watcher::Config::default())
        .run(reconciler, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => warn!("reconcile failed: {}", e),
            }
        })
        .await;

    Ok(())
}
