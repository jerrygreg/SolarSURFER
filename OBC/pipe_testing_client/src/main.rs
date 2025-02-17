//client



use winpipe::WinStream;
use std::io;
use std::str;

use std::io::{Read, Write};

use std::time::Duration;
use std::thread;

fn connect_pipe() -> io::Result<()>{
    let mut stream = WinStream::connect("\\\\.\\pipe\\my_pipe_is_cool")?;

    stream.set_write_timeout(Some(Duration::from_millis(5000)))?;
    stream.set_read_timeout(Some(Duration::from_millis(50)))?;
    //WinStream::set_nonblocking(&stream,true)?;

    loop{
        let mut buffer = vec![0u8; 64];
       
        let received = stream.read(buffer.as_mut_slice());
        match received {
            Ok(bytes_read) => {
            
                println!("Received {:?}", str::from_utf8(&buffer[..bytes_read]));
            }
            Err(e) => {
                println!("{e:?}");
                //stream.write("Nothing received, sending notif".as_bytes())?;
            }
        }
        
       
        thread::sleep(Duration::from_millis(100));
    }
    //DROPPING Closes the pipe
    //print!("Input a string to be sent to the server: ");
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("Failed to read");
    //stream.write(("input").as_bytes())?;
    
}

fn main(){
    let whatever = connect_pipe();
    println!("whatever {whatever:?}")
}
