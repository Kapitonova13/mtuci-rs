// Hello Tokio
// use mini_redis::{client, Result};

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Open a connection to the mini-redis address.
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // Set the key "hello" with value "world"
//     client.set("hello", "world".into()).await?;

//     // Get key "hello"
//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }
// ----------------------------------------------------------------
// async fn say_world() {
//     println!("world");
// }

// #[tokio::main]
// async fn main() {
//     // Calling `say_world()` does not execute the body of `say_world()`.
//     let op = say_world();

//     // This println! comes first
//     println!("hello");

//     // Calling `.await` on `op` starts executing `say_world`.
//     op.await;
// }
// ----------------------------------------------------------------

//Spawning 

// use tokio::net::{TcpListener, TcpStream};
// use mini_redis::{Connection, Frame};

// #[tokio::main]
// async fn main() {
//     // Bind the listener to the address
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

//     loop {
//         // The second item contains the IP and port of the new connection.
//         let (socket, _) = listener.accept().await.unwrap();
//         process(socket).await;
//     }
// }

// // use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

//     loop {
//         let (socket, _) = listener.accept().await.unwrap();
//         // A new task is spawned for each inbound socket. The socket is
//         // moved to the new task and processed there.
//         tokio::spawn(async move {
//             process(socket).await;
//         });
//     }
// }


// async fn process(socket: TcpStream) {
//     // The `Connection` lets us read/write redis **frames** instead of
//     // byte streams. The `Connection` type is defined by mini-redis.
//     let mut connection = Connection::new(socket);

//     if let Some(frame) = connection.read_frame().await.unwrap() {
//         println!("GOT: {:?}", frame);

//         // Respond with an error
//         let response = Frame::Error("unimplemented".to_string());
//         connection.write_frame(&response).await.unwrap();
//     }
// }

// ----------------------------------------------------------------
// #[tokio::main]
// async fn main() {
//     let handle = tokio::spawn(async {
//         // Do some async work
//         "return value"
//     });

//     // Do some other work

//     let out = handle.await.unwrap();
//     println!("GOT {}", out);
// }
// ----------------------------------------------------------------
// не компилируется
// use tokio::task;

// #[tokio::main]
// async fn main() {
//     let v = vec![1, 2, 3];

//     task::spawn(async {
//         println!("Here's a vec: {:?}", v);
//     });
// }

// ----------------------------------------------------------------
// use tokio::task::yield_now;
// use std::rc::Rc;

// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         // The scope forces `rc` to drop before `.await`.
//         {
//             let rc = Rc::new("hello");
//             println!("{}", rc);
//         }

//         // `rc` is no longer used. It is **not** persisted when
//         // the task yields to the scheduler
//         yield_now().await;
//     });
// }
// ----------------------------------------------------------------
// use tokio::net::TcpStream;
// use mini_redis::{Connection, Frame};

// async fn process(socket: TcpStream) {
//     use mini_redis::Command::{self, Get, Set};
//     use std::collections::HashMap;

//     // A hashmap is used to store data
//     let mut db = HashMap::new();

//     // Connection, provided by `mini-redis`, handles parsing frames from
//     // the socket
//     let mut connection = Connection::new(socket);

//     // Use `read_frame` to receive a command from the connection.
//     while let Some(frame) = connection.read_frame().await.unwrap() {
//         let response = match Command::from_frame(frame).unwrap() {
//             Set(cmd) => {
//                 // The value is stored as `Vec<u8>`
//                 db.insert(cmd.key().to_string(), cmd.value().to_vec());
//                 Frame::Simple("OK".to_string())
//             }
//             Get(cmd) => {
//                 if let Some(value) = db.get(cmd.key()) {
//                     // `Frame::Bulk` expects data to be of type `Bytes`. This
//                     // type will be covered later in the tutorial. For now,
//                     // `&Vec<u8>` is converted to `Bytes` using `into()`.
//                     Frame::Bulk(value.clone().into())
//                 } else {
//                     Frame::Null
//                 }
//             }
//             cmd => panic!("unimplemented {:?}", cmd),
//         };

//         // Write the response to the client
//         connection.write_frame(&response).await.unwrap();
//     }
// }
// fn main() {}

// Общее состояние

// use bytes::Bytes;
// use mini_redis::{Connection, Frame};
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use tokio::net::{TcpListener, TcpStream};

// type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();

//     println!("Listening");

//     // A hashmap is used to store data
//     let db = Arc::new(Mutex::new(HashMap::new()));

//     loop {
//         let (socket, _) = listener.accept().await.unwrap();
//         // Clone the handle to the hash map.
//         let db = db.clone();

//         println!("Accepted");
//         tokio::spawn(async move {
//             process(socket, db).await;
//         });
//     }
// }

// async fn process(socket: TcpStream, db: Db) {
//     use mini_redis::Command::{self, Get, Set};

//     // Connection, provided by `mini-redis`, handles parsing frames from
//     // the socket
//     let mut connection = Connection::new(socket);

//     while let Some(frame) = connection.read_frame().await.unwrap() {
//         let response = match Command::from_frame(frame).unwrap() {
//             Set(cmd) => {
//                 let mut db = db.lock().unwrap();
//                 db.insert(cmd.key().to_string(), cmd.value().clone());
//                 Frame::Simple("OK".to_string())
//             }
//             Get(cmd) => {
//                 let db = db.lock().unwrap();
//                 if let Some(value) = db.get(cmd.key()) {
//                     Frame::Bulk(value.clone())
//                 } else {
//                     Frame::Null
//                 }
//             }
//             cmd => panic!("unimplemented {:?}", cmd),
//         };

//         // Write the response to the client
//         connection.write_frame(&response).await.unwrap();
//     }
// }

// ----------------------------------------------------------------

// Channels

// use bytes::Bytes;
// use mini_redis::client;
// use tokio::sync::{mpsc, oneshot};

// /// Multiple different commands are multiplexed over a single channel.
// #[derive(Debug)]
// enum Command {
//     Get {
//         key: String,
//         resp: Responder<Option<Bytes>>,
//     },
//     Set {
//         key: String,
//         val: Bytes,
//         resp: Responder<()>,
//     },
// }

// /// Provided by the requester and used by the manager task to send the command
// /// response back to the requester.
// type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = mpsc::channel(32);
//     // Clone a `tx` handle for the second f
//     let tx2 = tx.clone();

//     let manager = tokio::spawn(async move {
//         // Open a connection to the mini-redis address.
//         let mut client = client::connect("127.0.0.1:6379").await.unwrap();

//         while let Some(cmd) = rx.recv().await {
//             match cmd {
//                 Command::Get { key, resp } => {
//                     let res = client.get(&key).await;
//                     // Ignore errors
//                     let _ = resp.send(res);
//                 }
//                 Command::Set { key, val, resp } => {
//                     let res = client.set(&key, val).await;
//                     // Ignore errors
//                     let _ = resp.send(res);
//                 }
//             }
//         }
//     });

//     // Spawn two tasks, one setting a value and other querying for key that was
//     // set.
//     let t1 = tokio::spawn(async move {
//         let (resp_tx, resp_rx) = oneshot::channel();
//         let cmd = Command::Get {
//             key: "foo".to_string(),
//             resp: resp_tx,
//         };

//         // Send the GET request
//         if tx.send(cmd).await.is_err() {
//             eprintln!("connection task shutdown");
//             return;
//         }

//         // Await the response
//         let res = resp_rx.await;
//         println!("GOT (Get) = {:?}", res);
//     });

//     let t2 = tokio::spawn(async move {
//         let (resp_tx, resp_rx) = oneshot::channel();
//         let cmd = Command::Set {
//             key: "foo".to_string(),
//             val: "bar".into(),
//             resp: resp_tx,
//         };

//         // Send the SET request
//         if tx2.send(cmd).await.is_err() {
//             eprintln!("connection task shutdown");
//             return;
//         }

//         // Await the response
//         let res = resp_rx.await;
//         println!("GOT (Set) = {:?}", res);
//     });

//     t1.await.unwrap();
//     t2.await.unwrap();
//     manager.await.unwrap();
// }

// ----------------------------------------------------------------
// IO

// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = [0; 10];

//     // read up to 10 bytes
//     let n = f.read(&mut buffer[..]).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

// AsyncReadExt::read_to_end читает все байты из потока до EOF.

// use tokio::io::{self, AsyncReadExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = Vec::new();

//     // read the whole file
//     f.read_to_end(&mut buffer).await?;
//     Ok(())
// }

// AsyncWriteExt::write записывает буфер в модуль записи, возвращая количество записанных байтов.

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;

//     // Writes some prefix of the byte string, but not necessarily all of it.
//     let n = file.write(b"some bytes").await?;

//     println!("Wrote the first {} bytes of 'some bytes'.", n);
//     Ok(())
// }

// AsyncWriteExt::write_all записывает весь буфер в модуль записи.

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;

//     file.write_all(b"some bytes").await?;
//     Ok(())
// }

// tokio::io::copy асинхронно копирует все содержимое модуля чтения в модуль записи.

// use tokio::fs::File;
// use tokio::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut reader: &[u8] = b"hello";
//     let mut file = File::create("foo.txt").await?;

//     io::copy(&mut reader, &mut file).await?;
//     Ok(())
// }


// ----------------------------------------------------------------
// use tokio::io;
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             // Copy data here
//         });
//     }
// }
// ----------------------------------------------------------------

// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let socket = TcpStream::connect("127.0.0.1:6142").await?;
//     let (mut rd, mut wr) = io::split(socket);

//     // Write data in the background
//     tokio::spawn(async move {
//         wr.write_all(b"hello\r\n").await?;
//         wr.write_all(b"world\r\n").await?;

//         // Sometimes, the rust type inferencer needs
//         // a little help
//         Ok::<_, io::Error>(())
//     });

//     let mut buf = vec![0; 128];

//     loop {
//         let n = rd.read(&mut buf).await?;

//         if n == 0 {
//             break;
//         }

//         println!("GOT {:?}", &buf[..n]);
//     }

//     Ok(())
// }

// ----------------------------------------------------------------


// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let mut buf = vec![0; 1024];

//             loop {
//                 match socket.read(&mut buf).await {
//                     // Return value of `Ok(0)` signifies that the remote has
//                     // closed
//                     Ok(0) => return,
//                     Ok(n) => {
//                         // Copy the data back to socket
//                         if socket.write_all(&buf[..n]).await.is_err() {
//                             // Unexpected socket error. There isn't much we can
//                             // do here so just stop processing.
//                             return;
//                         }
//                     }
//                     Err(_) => {
//                         // Unexpected socket error. There isn't much we can do
//                         // here so just stop processing.
//                         return;
//                     }
//                 }
//             }
//         });
//     }
// }
// ----------------------------------------------------------------

// Framing
// use bytes::Bytes;

// enum Frame {
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Bulk(Bytes),
//     Null,
//     Array(Vec<Frame>),
// }


// use bytes::BytesMut;
// use tokio::net::TcpStream;

// pub struct Connection {
//     stream: TcpStream,
//     buffer: BytesMut,
// }

// impl Connection {
//     pub fn new(stream: TcpStream) -> Connection {
//         Connection {
//             stream,
//             // Allocate the buffer with 4kb of capacity.
//             buffer: BytesMut::with_capacity(4096),
//         }
//     }
// }

// use tokio::io::AsyncReadExt;
// use bytes::Buf;
// use mini_redis::Result;

// pub async fn read_frame(&mut self)
//     -> Result<Option<Frame>>
// {
//     loop {
//         // Attempt to parse a frame from the buffered data. If
//         // enough data has been buffered, the frame is
//         // returned.
//         if let Some(frame) = self.parse_frame()? {
//             return Ok(Some(frame));
//         }

//         // There is not enough buffered data to read a frame.
//         // Attempt to read more data from the socket.
//         //
//         // On success, the number of bytes is returned. `0`
//         // indicates "end of stream".
//         if 0 == self.stream.read_buf(&mut self.buffer).await? {
//             // The remote closed the connection. For this to be
//             // a clean shutdown, there should be no data in the
//             // read buffer. If there is, this means that the
//             // peer closed the socket while sending a frame.
//             if self.buffer.is_empty() {
//                 return Ok(None);
//             } else {
//                 return Err("connection reset by peer".into());
//             }
//         }
//     }
// }

// use tokio::net::TcpStream;

// pub struct Connection {
//     stream: TcpStream,
//     buffer: Vec<u8>,
//     cursor: usize,
// }

// impl Connection {
//     pub fn new(stream: TcpStream) -> Connection {
//         Connection {
//             stream,
//             // Allocate the buffer with 4kb of capacity.
//             buffer: vec![0; 4096],
//             cursor: 0,
//         }
//     }
// }

// use mini_redis::{Frame, Result};

// pub async fn read_frame(&mut self)
//     -> Result<Option<Frame>>
// {
//     loop {
//         if let Some(frame) = self.parse_frame()? {
//             return Ok(Some(frame));
//         }

//         // Ensure the buffer has capacity
//         if self.buffer.len() == self.cursor {
//             // Grow the buffer
//             self.buffer.resize(self.cursor * 2, 0);
//         }

//         // Read into the buffer, tracking the number
//         // of bytes read
//         let n = self.stream.read(
//             &mut self.buffer[self.cursor..]).await?;

//         if 0 == n {
//             if self.cursor == 0 {
//                 return Ok(None);
//             } else {
//                 return Err("connection reset by peer".into());
//             }
//         } else {
//             // Update our cursor
//             self.cursor += n;
//         }
//     }
// }

// use mini_redis::{Frame, Result};
// use mini_redis::frame::Error::Incomplete;
// use bytes::Buf;
// use std::io::Cursor;

// fn parse_frame(&mut self)
//     -> Result<Option<Frame>>
// {
//     // Create the `T: Buf` type.
//     let mut buf = Cursor::new(&self.buffer[..]);

//     // Check whether a full frame is available
//     match Frame::check(&mut buf) {
//         Ok(_) => {
//             // Get the byte length of the frame
//             let len = buf.position() as usize;

//             // Reset the internal cursor for the
//             // call to `parse`.
//             buf.set_position(0);

//             // Parse the frame
//             let frame = Frame::parse(&mut buf)?;

//             // Discard the frame from the buffer
//             self.buffer.advance(len);

//             // Return the frame to the caller.
//             Ok(Some(frame))
//         }
//         // Not enough data has been buffered
//         Err(Incomplete) => Ok(None),
//         // An error was encountered
//         Err(e) => Err(e.into()),
//     }
// }

// use tokio::io::BufWriter;
// use tokio::net::TcpStream;
// use bytes::BytesMut;

// pub struct Connection {
//     stream: BufWriter<TcpStream>,
//     buffer: BytesMut,
// }

// impl Connection {
//     pub fn new(stream: TcpStream) -> Connection {
//         Connection {
//             stream: BufWriter::new(stream),
//             buffer: BytesMut::with_capacity(4096),
//         }
//     }
// }

// use tokio::io::{self, AsyncWriteExt};
// use mini_redis::Frame;

// async fn write_frame(&mut self, frame: &Frame)
//     -> io::Result<()>
// {
//     match frame {
//         Frame::Simple(val) => {
//             self.stream.write_u8(b'+').await?;
//             self.stream.write_all(val.as_bytes()).await?;
//             self.stream.write_all(b"\r\n").await?;
//         }
//         Frame::Error(val) => {
//             self.stream.write_u8(b'-').await?;
//             self.stream.write_all(val.as_bytes()).await?;
//             self.stream.write_all(b"\r\n").await?;
//         }
//         Frame::Integer(val) => {
//             self.stream.write_u8(b':').await?;
//             self.write_decimal(*val).await?;
//         }
//         Frame::Null => {
//             self.stream.write_all(b"$-1\r\n").await?;
//         }
//         Frame::Bulk(val) => {
//             let len = val.len();

//             self.stream.write_u8(b'$').await?;
//             self.write_decimal(len as u64).await?;
//             self.stream.write_all(val).await?;
//             self.stream.write_all(b"\r\n").await?;
//         }
//         Frame::Array(_val) => unimplemented!(),
//     }

//     self.stream.flush().await;

//     Ok(())
// }