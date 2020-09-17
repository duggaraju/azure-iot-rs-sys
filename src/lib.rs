#[macro_use]
extern crate log as logger;

mod iothub;


#[cfg(test)]
mod tests {

    use super::iothub::*;
    #[test]
    fn initialize_client() {
        let mut client = IotHubModuleClient::new(move |event: IotHubModuleEvent| { info!("Received event!"); });
    }
}
