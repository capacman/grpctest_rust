use std::time::Duration;

use anyhow::Result as AResult;
use grpctest_rust::test_service::{
    test_loader_service_client::TestLoaderServiceClient, BinaryData,
};
use tokio;
use tokio_stream;
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> AResult<()> {
    

    let data = {
        let mut d: Vec<u8> = Vec::with_capacity(1024 * 1024);
        for i in 0..1024 * 1024 {
            d.push(i as u8);
        }
        BinaryData { data: d }
    };

    let stream = {
        let mut v = Vec::with_capacity(10);
        for _ in 0..130 {
            v.push(data.clone());
        }
        v
    };

    loop {
        let mut client = TestLoaderServiceClient::connect("http://localhost:9000").await?;
        let result = client.load(tokio_stream::iter(stream.clone())).await?.into_inner();
        println!("response is {:?}",result);
        tokio::time::sleep(Duration::from_millis(150)).await;
    }

    Ok(())
}
