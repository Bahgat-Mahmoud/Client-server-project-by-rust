// Importing necessary modules and structs for message handling and logging
use crate::message::{AddResponse, ClientMessage, ServerMessage}; // Import message types
use crate::message::server_message; // Import the server's message module
use crate::message::client_message::Message as ClientMessageType; // Import client message type
use log::{error, info,warn}; // Import logging macros
use prost::Message; // Import Prost for message encoding/decoding
use std::{
    io::{self, ErrorKind, Read, Write}, // Import IO functionality for reading and writing
    net::{TcpListener, TcpStream}, // Import TCP listener and stream for network communication
    sync::{atomic::{AtomicBool, Ordering}, Arc}, // Import synchronization tools for atomic operations and shared ownership
    thread, // Import thread handling for concurrent execution
    time::Duration, // Import duration type for thread sleep
};

// Define the Client structure with a TCP stream for communication
struct Client {
    stream: TcpStream, // TCP stream to interact with the client
}

impl Client {
    // Client constructor to create a new client from a given TCP stream
    pub fn new(stream: TcpStream) -> Self {
        Client { stream } // Return a new Client instance
    }

    // Handle communication with the client
    pub fn handle(&mut self) -> io::Result<()> {
        let mut buffer = [0; 512]; // Buffer to store incoming data
        loop {
            // Read data from the client
            let bytes_read = match self.stream.read(&mut buffer) {
                Ok(0) => {
                    info!("Client disconnected."); // Log when the client disconnects
                    break; // Exit the loop if no data is received (client disconnected)
                }
                Ok(n) => n, // Successfully read 'n' bytes from the stream
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10)); // If no data, sleep briefly and try again
                    continue;
                }
                Err(e) => {
                    error!("Failed to read from client: {}", e); // Log error if reading fails
                    break; // Exit the loop on error
                }
            };
    
            // Decode the received client message
            match ClientMessage::decode(&buffer[..bytes_read]) {
                Ok(client_message) => {
                    match client_message.message { // Match on the decoded client message
                        Some(ClientMessageType::EchoMessage(echo_message)) => {
                            info!("Received EchoMessage: {}", echo_message.content); // Log EchoMessage content
                            let response = ServerMessage {
                                message: Some(server_message::Message::EchoMessage(echo_message)),
                            };
                            let payload = response.encode_to_vec(); // Encode response to byte vector
                            self.stream.write_all(&payload)?; // Send the encoded response back to the client
                            info!("EchoMessage sent back successfully.");
                        }
                        Some(ClientMessageType::AddRequest(add_request)) => {
                            info!("Received AddRequest: a={}, b={}", add_request.a, add_request.b); // Log AddRequest
                            let add_response = AddResponse {
                                result: add_request.a + add_request.b, // Calculate result of addition
                            };
                            let response = ServerMessage {
                                message: Some(server_message::Message::AddResponse(add_response)),
                            };
                            let payload = response.encode_to_vec(); // Encode AddResponse to byte vector
                            self.stream.write_all(&payload)?; // Send the response to the client
                            info!(
                                "AddResponse sent successfully with result: {}",
                                add_request.a + add_request.b // Log result of the addition
                            );
                        }
                        None => {
                            error!("Received an empty or unsupported ClientMessage."); // Log error if no valid message
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to decode ClientMessage: {}", e); // Log decoding error
                }
            }
        }
        Ok(()) // Return success
    }
}

// Define the Server structure with a TCP listener and a flag to check if it's running
pub struct Server {
    listener: TcpListener, // The TCP listener to accept incoming connections
    is_running: Arc<AtomicBool>, // Atomic flag to check if the server is running
}

impl Server {
    /// Creates a new server instance that listens on the given address
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?; // Bind the listener to the provided address

        let is_running = Arc::new(AtomicBool::new(false)); // Create an atomic flag for the server's state
        thread::sleep(Duration::from_millis(1)); // Sleep briefly to ensure the listener is ready
        Ok(Server {
            listener, // Return the server instance with listener
            is_running,
        })
    }

    /// Stops the server by setting the `is_running` flag to `false`
    pub fn stop(&self) {
        if self.is_running.load(Ordering::SeqCst) { // Check if the server is currently running
            self.is_running.store(false, Ordering::SeqCst); // Stop the server
            info!("Shutdown signal sent."); // Log server shutdown
        } else {
            warn!("Server was already stopped or not running."); // Log if the server is already stopped
        }
    }

    /// Runs the server, listens for incoming connections, and handles them
    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst); // Set the server as running
        println!("Server is running on {}", self.listener.local_addr()?); // Log the server's address

        self.listener.set_nonblocking(true)?; // Set the listener to non-blocking mode

        while self.is_running.load(Ordering::SeqCst) { // Keep running while the server is active
            match self.listener.accept() { // Accept new connections
                Ok((stream, addr)) => {
                    println!("New client connected: {}", addr); // Log new client connection
                    let mut client = Client::new(stream); // Create a new client instance
                    thread::spawn(move || { // Spawn a new thread to handle the client
                        if let Err(e) = client.handle() { // Handle client communication
                            println!("Error handling client: {}", e); // Log any error that occurs
                        }
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100)); // Sleep if no new connections are available
                }
                Err(e) => {
                    println!("Error accepting connection: {}", e); // Log error if accepting a connection fails
                }
            }
        }

        println!("Server stopped."); // Log when the server stops
        Ok(()) // Return success
    }
}
