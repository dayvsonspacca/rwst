# Rwst (Rust WebSocket Tester)

Rwst is a command-line tool designed to test WebSocket servers with ease. It allows you to connect to a WebSocket server, send messages, listen for incoming messages, and measure the time it takes for operations to complete. This tool is ideal for developers who need a quick and easy way to interact with and test WebSocket servers.

## Features

- **Connect**: Establish a connection with a WebSocket server.
- **Ping**: Send a "Ping" message to the server and measure the round-trip time.
- **Listen**: Continuously listen for incoming messages from the server.
- **Send**: Send custom messages to the server and measure the time taken.
- **Exit**: Gracefully disconnect from the server.

## Installation

1. Ensure you have Rust and Cargo installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Install via cargo:
   ```sh
   cargo install rwst
   ```

## Usage

To use Rwst, simply run the binary and provide the WebSocket server URL as an argument:

```sh
rwst ws://example.com:port
```

Once connected, you can choose from the following options:

1. **Ping**: Sends a "Ping" message to the server and displays the time taken for the response.
2. **Listen**: Listens for incoming messages from the server.
3. **Send**: Prompts you to enter a custom message to send to the server.
4. **Exit**: Disconnects from the server and exits the program.

### Example

```sh
./rwst ws://echo.websocket.org
```

Example output:

```
Connecting to ws://echo.websocket.org...
Connection with ws://echo.websocket.org established successfully! Took 1.234s
Enter Command: Ping
Pong Took 0.002s
Enter Command: Send
Message: Hello, WebSocket!
Message sent: Hello, WebSocket! Took 0.001s
Enter Command: Listen
Message received: Hello, WebSocket!
Enter Command: Exit
Disconnecting from ws://echo.websocket.org
You chose to exit, bye 👋 see you later.
```

## Dependencies

- **Clap**: Command-line argument parsing.
- **Colored**: Terminal text coloring.
- **Tungstenite**: WebSocket client library.
- **Inquire**: Interactive prompts for command-line interfaces.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
