# rust-listener
A simple Rust tool that listens for messages on a named queue in Azure Service Bus. 

This listener is meant to demonstrate the use of the Azure Service Bus crate provided as part of the Azure SDK for Rust.
It expects to pull messages off a queue that are formatted in JSON as one of the following key/value pairs:

{
    "Planet": "Earth"
}

{
    "Planet": "Mars"
}

{
    "Planet": "Europa"
}

{
    "Planet": "Jupiter"
}

The listenter does some simple message processing in the form of matching the planet to print out a planet-specific message.
The listener uses values read from a .env file to build a connection string to your service bus implementation.
Replace the values in '<>' brackets listed in the .env file with the relevant details from your servcice bus instance.

The listener will loop forever, waiting to read messages from the service bus. You will need another tool to send messages to the service bus.
At some point I will get around to creating a sender to match the listener. 
