//Server
//use winapi::um::namedpipeapi::*;
//use winapi::winbase::{CreateNamedPipeA, CallNamedPipeA}
/*
fn main(){

    let pipename = "\\\\.\\pipe\\my_pipe_is_cool";

    CreateNamedPipeA()
    



}*/


//


use winpipe::WinListener;
use winpipe::WinStream;
use std::io::{Read, Write};
use std::{thread, io, str};
use std::time::Duration;

fn listen_for_pipes() -> io::Result<()>{
    let listener = WinListener::bind("\\\\.\\pipe\\my_pipe_is_cool")?;
    let mut count = 0;
    
        
        let (mut stream, _addr) = listener.accept()?;
        stream.set_write_timeout(Some(Duration::from_millis(5000)))?;
        stream.set_read_timeout(Some(Duration::from_millis(5000)))?;
        //set non_blocking
        WinStream::set_nonblocking(&stream,true)?;
    loop {
        stream.write((format!("Loop number: {count}")).as_bytes())?;
        println!("Sent {count}");
        thread::sleep(Duration::from_millis(250));
        //RECIEVE
        //let mut buffer = vec![0u8; 64];
        //let received = stream.read(buffer.as_mut_slice())?;
        //println!("Received {:?}", str::from_utf8(&buffer[..received]));

        count += 1;
        //DROPPING Closes the pipe
    }
}

fn main(){
    let whatever = listen_for_pipes();
    println!("Done: {whatever:?}");
}//