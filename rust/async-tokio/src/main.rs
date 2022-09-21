use futures::future;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let ports: Vec<u16> = vec![10121, 10122];
    println!("starting echo server ...");
    main_spawn_on_bind_with_join(ports).await;
}

async fn main_spawn_on_bind_with_join(ports: Vec<u16>) {
    // mostly similar to spawn_on_bind function but instead of awaiting the join handles,
    // it is using join_all from futures crate.
    let mut join_handles = Vec::new();
    for port in ports {
        let handle = tokio::spawn(async move {
            let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
            println!("bound to port {}", port);
            let (tcp_stream, _) = listener.accept().await.unwrap();
            echo_loop(tcp_stream).await;
        });
        join_handles.push(handle);
    }
    future::join_all(join_handles).await;
}

async fn main_spawn_on_bind(ports: Vec<u16>) {
    // this will spawn a task to for each port so that binding also happens concurrently
    let mut join_handles = Vec::new();
    for port in ports {
        let handle = tokio::spawn(async move {
            // for each port a task is spawned so iteration will complete fast.
            // each listener will be accepting a connection in separate task and processing it
            let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
            println!("bound to port {}", port);
            let (tcp_stream, _) = listener.accept().await.unwrap();
            // NOTE: handling happens in the same spawned task. here a separate task for handling
            // is not spawned as it fits our basic use case. we expect only one connection per port
            // we can, however spawn another task for handling as well.
            echo_loop(tcp_stream).await;
        });
        join_handles.push(handle);
        // if we await above JoinHandle inside loop then spawned task is awaited in this thread
        // SO essentially until echo_loop for first iteration is not completed, second iteration
        // won't start. NOTE: await on join handle actually gives the result of the task back to
        // current thread. It does not cause the execution. this is not *awaiting* on Future.
    }

    // awaiting outside the loop, so the function does not finish untill all tasks are finished
    for handle in join_handles {
        handle.await.unwrap();
    }
}

async fn main_spawn_on_accept(ports: Vec<u16>) {
    // this will accept in the single thread but spawn when a connection is accepted
    let mut join_handles = Vec::new();
    for port in ports {
        let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
        println!("bound to port {}", port);
        // if no connection comes on first port, yield from accept will go nowhere because there
        // is only one task at hand. everything is in same thread. this new task will read/write
        // in loop but if data is not available, it spawned task will yield to current thread and
        // current thread will execute the second iteration.
        let (tcp_stream, _) = listener.accept().await.unwrap();
        // once the first connection is accepted then a task is spawned and awaited
        // NOTE: Before first port accepts connection, second port cannot accept connection
        let handle = tokio::spawn(async move {
            echo_loop(tcp_stream).await;
        });
        join_handles.push(handle);
        // if we await above JoinHandle inside loop then spawned task is awaited in this thread
        // SO essentially until echo_loop for first iteration is not completed, second iteration
        // won't start. NOTE: await on join handle actually gives the result of the task back to
        // current thread. It does not cause the execution. this is not *awaiting* on Future.
    }

    // awaiting outside the loop, so the function does not finish untill all tasks are finished
    for handle in join_handles {
        handle.await.unwrap();
    }
}

async fn main_no_spawn(ports: Vec<u16>) {
    for port in ports {
        // there is no spawn. so bind, accept and handling happening in same thread
        let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
        println!("bound to port {}", port);
        // if no connection comes on first port, yield from accept will go nowhere because there
        // is only one task at hand. everything is in same thread
        let (tcp_stream, _) = listener.accept().await.unwrap();
        // this echo only once and exits. this gives chance to run second iteration of loop
        // if echo_once also runs infinite loop to read on long running connection, then
        // second iteration won't happen. no one can connect to second port
        echo_once(tcp_stream).await;
    }
}

async fn echo_once(mut stream: TcpStream) {
    println!("incoming connection from {}", stream.peer_addr().unwrap());
    let mut buf = [0; 512];
    // just read once and write once and exit
    match stream.read(&mut buf).await {
        Ok(bytes_read) => {
            println!("received bytes {:?}", &buf[..bytes_read]);
            stream.write_all(&buf[..bytes_read]).await.unwrap();
        }
        Err(_) => {
            println!("connection reset");
        }
    };
}

async fn echo_loop(mut stream: TcpStream) {
    println!("incoming connection from {}", stream.peer_addr().unwrap());
    let mut buf = [0; 512];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("read zero bytes");
                break;
            }
            Ok(bytes_read) => {
                println!("received bytes {:?}", &buf[..bytes_read]);
                stream.write_all(&buf[..bytes_read]).await.unwrap();
            }
            Err(_) => {
                println!("connection reset");
                break;
            }
        };
    }
}
