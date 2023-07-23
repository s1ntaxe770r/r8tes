use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CustomResource, JsonSchema, Default, Copy)]
#[kube(
    group = "operators.gopher.net",
    version = "v1",
    kind = "RateCache",
    namespaced,
    shortname = "rc"
)]
pub struct RateCacheSpec {}
