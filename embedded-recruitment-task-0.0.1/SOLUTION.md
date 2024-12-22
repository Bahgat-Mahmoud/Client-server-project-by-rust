Here's the updated report with the new test and the addition of `Arc` + `AtomicPool` to address starvation, race conditions, and deadlock issues:

---

# **Embedded SOLUTION Task**

### Changes Made in the Code

#### 1. **Client-Server Communication Handling**:
   - **Client Struct**: 
     - A `Client` struct was created to handle communication with the server. It includes methods for handling incoming and outgoing messages, using a `TcpStream` for data transmission.
     - The `handle` method was implemented to read data from the client, decode the incoming message, process it (e.g., echoing a message or handling an addition request), and send the appropriate response back to the client.
   
   - **Server Struct**: 
     - A `Server` struct was implemented to handle incoming client connections using a `TcpListener` and process messages from multiple clients concurrently.
     - The server was designed to run in a non-blocking mode, allowing it to handle multiple clients simultaneously without blocking the main thread.
     - A mechanism for stopping the server using an atomic flag (`is_running`) was added, which allows the server to be gracefully stopped.

#### 2. **Message Handling**:
   - **Message Types**: 
     - New message types were imported and utilized, such as `EchoMessage` and `AddRequest`, which are used for different client-server interactions.
     - The server can now handle different types of requests, including echoing a message back to the client and processing addition requests, sending the result back to the client.
   
   - **Protobuf Encoding/Decoding**:
     - The `prost::Message` library was used to encode and decode messages in Protocol Buffers format, which allows structured message exchanges between the client and server.

#### 3. **Concurrency and Threading**:
   - The server is now capable of handling multiple client connections concurrently using threads. Each client is handled in a separate thread, which allows the server to be more scalable and responsive.
   - The client communication handling logic runs in an infinite loop until the client disconnects or an error occurs. The server uses `thread::spawn` to create a new thread for each client connection.

#### 4. **Error Handling**:
   - Improved error handling for network communication, including reading and writing from the `TcpStream`. Errors are logged using the `log` crate.
   - The server and client properly handle connection issues, such as when no data is available to read (non-blocking behavior), and client disconnections.

#### 5. **Test Cases**:
   - Several test cases were added to ensure the proper functionality of the server and client communication:
     - **test_client_connection**: Tests basic client-server connection and disconnection.
     - **test_multiple_echo_messages**: Tests the ability of the server to echo multiple messages sent by the client.
     - **test_multiple_clients**: Ensures the server can handle multiple clients sending messages simultaneously.
     - **test_client_echo_message**: Verifies that the server correctly echoes a single message sent by the client.
     - **test_client_add_request**: Tests the server’s ability to handle addition requests and return the correct result.
   
   **New Test Case**:
     - **test_empty_message**: Ensures the server handles and echoes empty messages correctly.
     - **test_unknown_message**: Verifies that the server responds correctly to unknown messages.
     - **test_long_disconnection**: Tests the ability of the client to reconnect after a long disconnection.
     - **test_max_message_limit**: Ensures the server can handle a large number of messages sent quickly by the client.
     - **test_thread_synchronization**: This test verifies that the server's concurrency model can handle multiple simultaneous requests without encountering race conditions, starvation, or deadlocks.

   - All test cases were executed independently, and they all passed successfully.

#### 6. **Logging**:
   - Logging functionality was added throughout the server and client code using the `log` crate. Logs include important events like client connections, received messages, and errors.
   - The logs help track the flow of communication between the client and server and assist in debugging.

#### 7. **Server Stop Mechanism**:
   - The server includes a `stop` method that sets an atomic flag to false, signaling that the server should stop accepting new connections and gracefully shut down.

#### 8. **Non-Blocking Mode**:
   - The server is configured to operate in non-blocking mode using `set_nonblocking(true)` on the `TcpListener`. This prevents the server from being blocked while waiting for incoming connections, improving performance and responsiveness.

#### 9. **Starvation, Race Condition, and Deadlock Fix**:
   - **Arc + AtomicPool**: To resolve issues related to **starvation**, **race conditions**, and **deadlocks**, we integrated the `Arc` (Atomic Reference Counting) and `AtomicPool` in the server's threading model. By utilizing these tools, the server ensures proper synchronization of shared resources between threads, preventing threads from being blocked indefinitely (starvation), eliminating race conditions by ensuring safe access to shared data, and avoiding deadlocks through careful thread management and resource allocation.
