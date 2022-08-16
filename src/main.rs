use azure_messaging_servicebus::prelude::*;
use rustdotenv::load;
use serde_json::{Value};
use async_std::task;

#[tokio::main]
async fn main() {
    load(None);

    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
      .expect("Service Bus Namespace not found in .env file");
    let queue_name = std::env::var("AZURE_QUEUE_NAME")
      .expect("Queue name not found in .env file");
    let policy_name = std::env::var("AZURE_POLICY_NAME")
      .expect("Policy name not found in .env file");
    let policy_key = std::env::var("AZURE_POLICY_KEY")
      .expect("Policy key not found in .env file");
    let http_client = azure_core::new_http_client();

    let client = Client::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )
    .expect("Failed to create client");

    loop {
        task::block_on(async {
            let received_message = client
            .receive_and_delete_message()
            .await
            .expect("Failed to receive message");
            
            let value = serde_json::from_str(&received_message);
            
            if value.is_ok() {
                let message: Value = value.unwrap();

                let planet = &message["Planet"];

                match planet.as_str() {
                    Some("Earth") => {
                        println!("Destination Earth - Let's drive");
                    },
                    Some("Mars") => {
                        println!("Destination Mars - Blast off");
                    },
                    Some("Europa") => {
                        println!("Destination Europa - Wait for teleportation");
                    }
                    _other => {
                        println!("Destination Unknown - Maybe try getting an Uber?");
                    }
                }
            }   
        });
    }
}
