extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tk_sendfile;

use std::net::SocketAddr;
use std::env;

use futures::Future;
use futures::stream::Stream;
use futures_cpupool::CpuPool;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tk_sendfile::DiskPool;


fn main() {
    let addr = env::args().nth(1).unwrap_or("0.0.0.0:80".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    let disk_pool = DiskPool::new(CpuPool::new(40));

    let mut lp = Core::new().unwrap();
    let handle = lp.handle();
    let socket = TcpListener::bind(&addr, &handle).unwrap();
    let done = socket.incoming().for_each(|(socket, _addr)| {
        handle.spawn(
            disk_pool.send("/data/med.txt", socket)
            .then(|_result| {
                Ok(())
            })
        );
        Ok(())
    });

    lp.run(done).unwrap();
}