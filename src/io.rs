use std::os::unix::io::FromRawFd;

use mio::Ready;
use bytes::BufMut;
use tokio::reactor::PollEvented2;
use futures::{Async, Poll, Stream, Sink, StartSend, AsyncSink};

pub struct Io {
    io: PollEvented2<mio::net::UdpSocket>,
}

impl Io {
    pub fn new(protocol: libc::c_int) -> nix::Result<Io> {
        let fd = unsafe {
            libc::socket(libc::AF_NETLINK, libc::SOCK_RAW, protocol.into())
        };

        if fd < 0 {
            return Err(nix::Error::last());
        }

        let socket = unsafe { std::net::UdpSocket::from_raw_fd(fd) };
        let socket = mio::net::UdpSocket::from_socket(socket).unwrap();
        let io = PollEvented2::new(socket);

        Ok(Io { io })
    }

    pub fn poll_send(&self, buf: &[u8]) -> Poll<usize, std::io::Error> {
        try_ready!(self.io.poll_write_ready());

        //let flags = nix::sys::socket::MsgFlags::empty();
        //let mut dst = nix::sys::socket::SockAddr::Netlink(nix::sys::socket::NetlinkAddr::new(0,0));

        let res = self.io.get_ref().send(buf);
        //use std::os::unix::io::AsRawFd;;
        //let fd = self.io.get_ref().as_raw_fd();
        //let res = nix::sys::socket::sendto(fd, buf, &mut dst, flags);
        //let res = res.map_err(|_| std::io::Error::last_os_error());

        match res {
            Ok(n) => Ok(n.into()),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                self.io.clear_write_ready()?;
                Ok(Async::NotReady)
            }
            Err(e) => Err(e),
        }
    }

    pub fn poll_recv(&self, buf: &mut [u8]) -> Poll<usize, std::io::Error> {
        try_ready!(self.io.poll_read_ready(Ready::readable()));

        //use std::os::unix::io::AsRawFd;;
        //let fd = self.io.get_ref().as_raw_fd();
        //let res = nix::sys::socket::recvfrom(fd, buf);
        //let res = res.map_err(|_| std::io::Error::last_os_error());
        let res = self.io.get_ref().recv(buf);
        
        match res {
            //Ok((ret, _addr)) => {println!("addr={:?}, len={}", _addr, ret); Ok(ret.into())},
            Ok(ret) => Ok(ret.into()),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                self.io.clear_read_ready(Ready::readable())?;
                Ok(Async::NotReady)
            }
            Err(e) => Err(e),
        }
    }
}

impl Sink for crate::NlSocket {
    type SinkItem = bytes::Bytes;
    type SinkError = std::io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        trace!("sending frame");

        if !self.flushed {
            match self.poll_complete()? {
                Async::Ready(()) => {},
                Async::NotReady => return Ok(AsyncSink::NotReady(item)),
            }
        }

        self.wr = item.into();
        self.flushed = false;
        trace!("frame encoded; length={}", self.wr.len());

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), std::io::Error> {
        if self.flushed {
            return Ok(Async::Ready(()))
        }

        trace!("flushing frame; length={}", self.wr.len());
        let n = try_ready!(self.io.poll_send(&self.wr));
        trace!("written {}", n);

        let wrote_all = n == self.wr.len();
        self.wr.clear();
        self.flushed = true;

        if wrote_all {
            Ok(Async::Ready(()))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other,
                               "failed to write entire datagram to socket").into())
        }
    }
}

impl Stream for crate::NlSocket {
    type Item = bytes::Bytes;
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Option<(Self::Item)>, Self::Error> {
        self.rd.reserve(crate::INITIAL_RD_CAPACITY);

        let n = unsafe {
            // Read into the buffer without having to initialize the memory.
            let n = try_ready!(self.io.poll_recv(&mut self.rd.bytes_mut()));
            self.rd.advance_mut(n);
            n
        };
        trace!("received {} bytes", n);
        let buf = self.rd.take().into();
        Ok(Async::Ready(Some(buf)))
    }
}
