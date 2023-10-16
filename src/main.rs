use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit,};
use std::io{self, Write, BufReader};

fn main() {
    let bind_ip = "127.1.1.0";
    let bind_port:u16 = 1234;

    let ip = bind_ip.parse::<Ipv4Addr>();
    let ipaddress = match ip{
        Ok(i) => i,
        Err(e) => {println!("{}", e); exit(0)}
    };
    if bind_port < 0 || bind_port > 65535{
        println!("Invalid port number");
        exit(0);
    }
   let cs = SocketAddrV4::new(ipaddress, bind_port);
   let listen = TcpListener.bind(cs);
   let listener = match listen{
       Ok(l) => l,
       Err(e) => {println!("{}", e); exit(0)}
   };
   let (mut clientsocket, clientaddress) = listener.accept().unwrap();
   println!(Client connected from: "{}", clientaddress);
   loop{
       println!("Enter command to send: ");
       let mut input = String::new();
       io::stdin().read_line(&mut input).expect("String expected");
       input.push('\0');
       clientsocket.write(&input.as_bytes());
       let mut buffer:Vec<u8> = Vec::new();
       let mut reader = BufReader::new(&clientsocket);
       reader.read_until(b'\0', &mut buffer);



   }



   drop(listener);
}
