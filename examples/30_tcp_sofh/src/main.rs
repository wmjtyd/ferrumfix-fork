use fesofh::SeqDecoder;
use std::io;
use std::net;

fn main() -> io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:8080")?;
    let reader = listener.accept()?.0;
    let decoder = SeqDecoder::default();
    let frames = decoder.read_frames(reader);
    for frame in frames {
        if let Ok(frame) = frame {
            let payload_clone = &frame.payload().to_vec()[..];
            let payload_utf8 = String::from_utf8_lossy(payload_clone);
            println!("Received message '{}'", payload_utf8);
        } else {
            break;
        }
    }
    Ok(())
}
