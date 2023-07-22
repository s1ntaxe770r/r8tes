use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{ Api, Client, CustomResourceExt};
use log::info;
use r8tes::resources::RateCache;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = match Client::try_default().await {
        Ok(kubeconfig) => kubeconfig,
        Err(_e) => panic!("{}", "unable to locate kubeconfig "),
    };

    // Generate the CRD
    let custom_resource: CustomResourceDefinition = RateCache::crd();

    // turn to string and print
    let crd_string = serde_yaml::to_string(&custom_resource)?;
    println!("{}", crd_string);

    let crd: Api<CustomResourceDefinition> = Api::all(client);

   // Create the CRD and ignore results as it will fail if it already exists
    let _ = crd.create(&Default::default(), &custom_resource).await;
    

    Ok(())
}
