use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use discortp::{rtp::RtpPacket, Packet};
use opus::Decoder as OpusDecoder;
use songbird::events::{EventContext, EventHandler};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, sync::Mutex};

const SAMPLE_RATE: u32 = 48_000;
const CHANNELS: usize = 2;

struct Stream {
    dec: OpusDecoder,
    file: tokio::fs::File,
}

pub struct Monitor {
    streams: Arc<Mutex<HashMap<u32, Stream>>>,
}

impl Monitor {
    pub fn new() -> Self {
        Self { streams: Arc::new(Mutex::new(HashMap::new())) }
    }
}

#[async_trait]
impl EventHandler for Monitor {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<songbird::events::Event> {
        if let EventContext::RtpPacket(pkt) = ctx {
            let rtp = RtpPacket::new(&pkt.packet)?;
            let ssrc = rtp.get_ssrc().into();

            let payload = rtp.payload();
            let start = pkt.payload_offset;
            let tail  = pkt.payload_end_pad;

            if payload.len() < start + tail {
                return None; // malformed or truncated
            }

            let opus_frame = &payload[start .. payload.len() - tail];
            if opus_frame.is_empty() {
                return None;
            }

            // Write raw audio to file. This will be changed later.
            if !self.streams.lock().await.contains_key(&ssrc) {
                let dec  = OpusDecoder::new(SAMPLE_RATE, opus::Channels::Stereo).ok()?;
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("user_{ssrc}.raw"))
                    .await
                    .ok()?;
                self.streams.lock().await.insert(ssrc, Stream { dec, file });
            }

            if let Some(stream) = self.streams.lock().await.get_mut(&ssrc) {
                let mut pcm = [0i16; 1920];
                if let Ok(samples) = stream.dec.decode(opus_frame, &mut pcm, false) {
                    let bytes = unsafe {
                        std::slice::from_raw_parts(
                            pcm.as_ptr() as *const u8,
                            samples as usize * 2 * CHANNELS,
                        )
                    };
                    let _ = stream.file.write_all(bytes).await;
                }
            }
        }
        None
    }
}
