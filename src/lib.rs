#[macro_use]
extern crate log as logger;

pub mod iothub;


#[cfg(test)]
mod tests {

    use super::iothub::*;
    #[test]
    fn initialize_client() {
        let _client = IotHubModuleClient::new(move |_event| { info!("Received event!"); });
    }
}
