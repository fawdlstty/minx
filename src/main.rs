pub mod value;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            while let Ok(n) = socket.read(&mut buf).await {
                if n == 0 {
                    break;
                }

                // match buf[0] {
                //     b'+' => {} // 单行字符串。"+OK\r\n"
                //     b'-' => {} // 错误。"-err msg\r\n"
                //     b':' => {} // 整型。":123\r\n"
                //     b'$' => {} // 多行字符串。"$6\r\nfoobar\r\n"、"$-1\r\n"（不存在的值）
                //     b'*' => {} // 数组。"*-1\r\n"（空对象的数组）
                //     _ => {}
                // }

                // socket
                //     .write_all(&buf[0..n])
                //     .await
                //     .expect("failed to write data to socket");
            }
        });
    }
}
