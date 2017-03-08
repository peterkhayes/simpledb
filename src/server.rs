extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use parser;

use std::io;
use std::str;
use std::net::SocketAddr;
use self::tokio_core::io::{Io, Framed, Codec, EasyBuf};
use self::tokio_proto::pipeline::{ServerProto};
use self::tokio_proto::TcpServer;
use self::tokio_service::Service;
use self::futures::{future, Future, BoxFuture};

struct StatementCodec;

impl Codec for StatementCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        // extract the next statement from the buffer (up to a semicolon)
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b';') {
            let statement = buf.drain_to(i);
            // also remove the semicolon itself.
            buf.drain_to(1);


            match str::from_utf8(statement.as_slice()) {
                Ok(s) => {
                    let result = s.trim();
                    Ok(Some(result.to_string()))
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8"))
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, response: String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(response.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

struct StatementProto;

impl <T: Io + 'static> ServerProto<T> for StatementProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = String;
    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = String;
    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, StatementCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(StatementCodec))
    }
}

struct SQLServer;

impl Service for SQLServer {
    type Request = String;
    type Response = String;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let parsed = parser::parse(req).unwrap();
        future::ok(parsed.table).boxed()
    }
}

pub fn run (addr : SocketAddr) {
    let server = TcpServer::new(StatementProto, addr);
    server.serve(|| Ok(SQLServer));
}
