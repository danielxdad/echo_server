use std::{
    io,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

struct Transformer<T: Read + Write> {
    inner: T,
}

// impl<T: Read + Write> Write for Transformer<T> {
impl<T> Write for Transformer<T>
where
    T: Read + Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf.to_ascii_uppercase().as_slice())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T> Read for Transformer<T>
where
    T: Read + Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

fn main() -> std::io::Result<()> {
    let interface = "127.0.0.1:3145";
    let listener = TcpListener::bind(interface)?;

    println!("Listening at {}", interface);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || match handle(stream) {
                    Ok(n) => println!("Write back: {}", n),
                    Err(err) => eprintln!("Error: {:?}", err),
                });
            }
            Err(err) => eprintln!("{:?}", err),
        }
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> std::io::Result<usize> {
    let mut buff = [0_u8; 512];
    let mut write_back = 0;
    let mut transformer = Transformer { inner: &mut stream };
    let result: std::io::Result<usize>;

    //println!(
    //    "Client from: {:?}",
    //    stream.peer_addr().expect("Unkwown peer addr")
    //);

    loop {
        match transformer.read(&mut buff) {
            Ok(n) if n == 0 => {
                result = Ok(write_back);
                break;
            }
            Ok(n) => {
                let s = String::from_utf8(buff[0..n].to_vec()).expect("Invalid utf8 string");
                //println!("{}, {}, {}", s.trim_end(), s.len(), s.capacity());
                //write_back += stream.write(&s.as_bytes())?;
                write_back += transformer.write(&s.as_bytes())?;
            }
            Err(err) => {
                result = Err(err);
                break;
            }
        }
    }

    result
}
