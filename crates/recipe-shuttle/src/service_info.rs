use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shuttle_common::resource::Type;
use shuttle_runtime::{Error, Factory, ResourceBuilder};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// The Shuttle service name.
    pub name: String,
}

pub struct ShuttleServiceInfo;

#[async_trait]
impl ResourceBuilder<ServiceInfo> for ShuttleServiceInfo {
    fn new() -> Self {
        Self
    }

    const TYPE: Type = Type::Secrets;

    type Config = ();

    type Output = ServiceInfo;

    fn config(&self) -> &Self::Config {
        &()
    }

    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, Error> {
        Ok(ServiceInfo {
            name: factory.get_service_name().to_string(),
        })
    }

    async fn build(build_data: &Self::Output) -> Result<ServiceInfo, Error> {
        Ok(build_data.clone())
    }
}
