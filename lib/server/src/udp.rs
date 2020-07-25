use common::bytes::Bytes;
use common::futures::SinkExt;
use common::futures_util::stream::StreamExt;
use common::tokio_util::codec::BytesCodec;
use common::tokio_util::udp::UdpFramed;
use tokio::net::UdpSocket;

pub async fn start() {
    let socket = UdpSocket::bind("0.0.0.0:5060")
        .await
        .expect("binding udp socket");
    common::log::debug!("starting udp server listening in port 5060");
    let socket = UdpFramed::new(socket, BytesCodec::new());
    let (mut sink, mut stream) = socket.split();

    while let Some(request) = stream.next().await {
        match request {
            Ok((request, addr)) => {
                let response = processor::process_message(request).await;
                common::log::info!("{}", addr);
                match response {
                    Ok(response) => sink
                        .send((Bytes::from(response), addr))
                        .await
                        .expect("failed"),
                    Err(e) => common::log::error!("{}", e.to_string()),
                };
            }
            Err(e) => common::log::error!("{:?}", e),
        }
    }
}