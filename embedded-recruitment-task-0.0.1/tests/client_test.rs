use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage},
    server::Server,
};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

mod client;

fn setup_server_thread(server: Arc<Server>) -> JoinHandle<()> {
    thread::spawn(move || {
        server.run().expect("Server encountered an error");
    })
}

fn create_server() -> Arc<Server> {
    let server = Server::new("localhost:8080");
    let msg = "Failed to start server";
    Arc::new(server.expect(msg))
}

#[test]
fn test_client_connection() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}


#[test]
// #[ignore = "please remove ignore and fix this test"]
fn test_multiple_echo_messages() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and receive multiple messages
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message);

        // Send the message to the server
        assert!(client.send(message).is_ok(), "Failed to send message");

        // Receive the echoed message
        let response = client.receive();
        assert!(
            response.is_ok(),
            "Failed to receive response for EchoMessage"
        );

        match response.unwrap().message {
            Some(server_message::Message::EchoMessage(echo)) => {
                assert_eq!(
                    echo.content, message_content,
                    "Echoed message content does not match"
                );
            }
            _ => panic!("Expected EchoMessage, but received a different message"),
        }
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
// #[ignore = "please remove ignore and fix this test"]
fn test_multiple_clients() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect multiple clients
    let mut clients = vec![
        client::Client::new("localhost", 8080, 1000),
        client::Client::new("localhost", 8080, 1000),
        client::Client::new("localhost", 8080, 1000),
    ];

    for client in clients.iter_mut() {
        assert!(client.connect().is_ok(), "Failed to connect to the server");
    }

    // Prepare multiple messages
    let messages = vec![
        "Hello, World!".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    // Send and receive multiple messages for each client
    for message_content in messages {
        let mut echo_message = EchoMessage::default();
        echo_message.content = message_content.clone();
        let message = client_message::Message::EchoMessage(echo_message.clone());

        for client in clients.iter_mut() {
            // Send the message to the server
            assert!(
                client.send(message.clone()).is_ok(),
                "Failed to send message"
            );

            // Receive the echoed message
            let response = client.receive();
            assert!(
                response.is_ok(),
                "Failed to receive response for EchoMessage"
            );

            match response.unwrap().message {
                Some(server_message::Message::EchoMessage(echo)) => {
                    assert_eq!(
                        echo.content, message_content,
                        "Echoed message content does not match"
                    );
                }
                _ => panic!("Expected EchoMessage, but received a different message"),
            }
        }
    }

    // Disconnect the clients
    for client in clients.iter_mut() {
        assert!(
            client.disconnect().is_ok(),
            "Failed to disconnect from the server"
        );
    }

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}


#[test]
// #[ignore = "please remove ignore and fix this test"]
fn test_client_echo_message() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare the message
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Receive the echoed message
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for EchoMessage"
    );

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(
                echo.content, echo_message.content,
                "Echoed message content does not match"
            );
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}

#[test]
// #[ignore = "please remove ignore and fix this test"]
fn test_client_add_request() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare the message
    let mut add_request = AddRequest::default();
    add_request.a = 10;
    add_request.b = 20;
    let message: client_message::Message = client_message::Message::AddRequest(add_request.clone());

    // Send the message to the server
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Receive the response
    let response = client.receive();
    assert!(
        response.is_ok(),
        "Failed to receive response for AddRequest"
    );

    match response.unwrap().message {
        Some(server_message::Message::AddResponse(add_response)) => {
            assert_eq!(
                add_response.result,
                add_request.a + add_request.b,
                "AddResponse result does not match"
            );
        }
        _ => panic!("Expected AddResponse, but received a different message"),
    }

    // Disconnect the client
    assert!(
        client.disconnect().is_ok(),
        "Failed to disconnect from the server"
    );

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(
        handle.join().is_ok(),
        "Server thread panicked or failed to join"
    );
}


/////////////////////// added casses /////////////////////////////
#[test]
fn test_empty_message() {
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Send an empty message
    let echo_message = EchoMessage { content: "".to_string() };
    let message = client_message::Message::EchoMessage(echo_message);

    assert!(client.send(message).is_ok(), "Failed to send empty message");

    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for empty message");

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(echo.content, "", "Echoed message content should be empty");
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    client.disconnect().unwrap();
    server.stop();
    handle.join().unwrap();
}

#[test]
fn test_unknown_message() {
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Send an unknown message
    let unknown_message = "UnknownMessage".to_string();
    let message = client_message::Message::EchoMessage(EchoMessage { content: unknown_message });

    assert!(client.send(message).is_ok(), "Failed to send unknown message");

    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for unknown message");

    // Check for the server's response to an unknown message (error handling can be customized)
    // You may implement specific handling for this kind of case

    client.disconnect().unwrap();
    server.stop();
    handle.join().unwrap();
}


#[test]
fn test_long_disconnection() {
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Disconnect the client
    client.disconnect().unwrap();

    // Reconnect after a long delay
    thread::sleep(Duration::from_secs(5));
    assert!(client.connect().is_ok(), "Failed to reconnect to the server");

    // Send a message after reconnecting
    let echo_message = EchoMessage { content: "Reconnected!".to_string() };
    let message = client_message::Message::EchoMessage(echo_message);

    assert!(client.send(message).is_ok(), "Failed to send message after reconnection");

    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for reconnection");

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(echo.content, "Reconnected!", "Echoed message content does not match");
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    client.disconnect().unwrap();
    server.stop();
    handle.join().unwrap();
}

#[test]
fn test_max_message_limit() {
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Send many messages quickly
    let message_count = 100;
    for i in 0..message_count {
        let echo_message = EchoMessage { content: format!("Message {}", i) };
        let message = client_message::Message::EchoMessage(echo_message);

        assert!(client.send(message).is_ok(), "Failed to send message {}", i);

        let response = client.receive();
        assert!(response.is_ok(), "Failed to receive response for message {}", i);

        match response.unwrap().message {
            Some(server_message::Message::EchoMessage(echo)) => {
                assert_eq!(echo.content, format!("Message {}", i), "Echoed message content does not match for message {}", i);
            }
            _ => panic!("Expected EchoMessage, but received a different message for message {}", i),
        }
    }

    client.disconnect().unwrap();
    server.stop();
    handle.join().unwrap();
}

#[test]
fn test_echo_and_add_request() {
    // Set up the server in a separate thread
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare EchoMessage
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Echo Test".to_string();
    let echo_msg = client_message::Message::EchoMessage(echo_message.clone());

    // Send the EchoMessage to the server
    assert!(client.send(echo_msg).is_ok(), "Failed to send EchoMessage");

    // Receive the echoed message
    let echo_response = client.receive();
    assert!(echo_response.is_ok(), "Failed to receive EchoMessage response");

    match echo_response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(echo.content, echo_message.content, "Echoed message content does not match");
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    // Prepare AddRequest
    let mut add_request = AddRequest::default();
    add_request.a = 5;
    add_request.b = 7;
    let add_msg = client_message::Message::AddRequest(add_request.clone());

    // Send the AddRequest to the server
    assert!(client.send(add_msg).is_ok(), "Failed to send AddRequest");

    // Receive the AddResponse
    let add_response = client.receive();
    assert!(add_response.is_ok(), "Failed to receive AddResponse");

    match add_response.unwrap().message {
        Some(server_message::Message::AddResponse(add_response)) => {
            assert_eq!(add_response.result, add_request.a + add_request.b, "AddResponse result does not match");
        }
        _ => panic!("Expected AddResponse, but received a different message"),
    }

    // Disconnect the client
    assert!(client.disconnect().is_ok(), "Failed to disconnect from the server");

    // Stop the server and wait for thread to finish
    server.stop();
    assert!(handle.join().is_ok(), "Server thread panicked or failed to join");
}
