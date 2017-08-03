extern crate mio;
use mio::*;
use mio::tcp::{TcpListener, TcpStream};

pub fn use_mio() {
    let mut value = 6.0;
    let val = &mut value as *mut f64;
    println!("val memory address: {:?}", val);

    // Setup some tokens to allow us to indentify which event is for which socket.
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);
    let addr = "127.0.0.1:3454".parse().unwrap();

    // Set the server socket.
    let server = TcpListener::bind(&addr).unwrap();

    // Create a poll instance.
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections.
    poll.register(&server,SERVER, Ready::readable(), PollOpt::edge()).unwrap();
    
    // Setup the client socket.
    let sock = TcpStream::connect(&addr).unwrap();
    
    // Register the socket.
    poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();
    
    // Create storage for enents.
    let mut events = Events::with_capacity(1024); 
    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let _ = server.accept();
                }
                CLIENT => {
                    return;
                }
                _  => unreachable!(),
            }
        }
    }
}
