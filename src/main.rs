use std::{thread::Thread, time::Duration};

use anyhow::Result as AResult;
use futures::channel::oneshot::channel;
use grpctest_rust::test_service::{
    test_loader_service_client::TestLoaderServiceClient, BinaryData,
};
use tokio;
use tokio_stream;
use tonic::{transport::Endpoint, Request};
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> AResult<()> {
    let data = {
        let mut d: Vec<u8> = Vec::with_capacity(1024 * 1024);
        for i in 0..1024 * 1024 {
            d.push(i as u8);
        }
        BinaryData {
            value: 0i32,
            data: d,
        }
    };

    let stream = {
        let mut v = Vec::with_capacity(10);
        for _ in 0..10 {
            v.push(data.clone());
        }
        v
    };
    let mut counter = 0;
    let endpoint = Endpoint::new("http://localhost:9000")?
        .http2_keep_alive_interval(Duration::from_secs(1))
        .keep_alive_while_idle(true);
    let conn = endpoint.connect().await?;
    loop {
        let mut client = TestLoaderServiceClient::new(conn.clone());
        let mut stream_clone = stream.clone();
        stream_clone[0].value = counter;
        let request = Request::new(tokio_stream::iter(stream_clone));
        let result = client.load(request).await?.into_inner();
        println!("response is {:?}", result);
        tokio::time::sleep(Duration::from_millis(60)).await;
        counter += 1;
        println!("counter value is {}", counter);
        /*if counter > 75 {
            break;
        }*/
    }

    tokio::time::sleep(Duration::from_secs(500)).await;

    Ok(())
}
