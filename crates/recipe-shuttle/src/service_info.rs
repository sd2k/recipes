use async_trait::async_trait;
use shuttle_service::{Factory, ResourceBuilder};

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

    async fn build(
        self,
        factory: &mut dyn Factory,
        _runtime: &shuttle_service::Runtime,
    ) -> Result<ServiceInfo, shuttle_service::Error> {
        Ok(ServiceInfo {
            name: factory.get_service_name().to_string(),
        })
    }
}
