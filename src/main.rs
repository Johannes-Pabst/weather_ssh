pub mod frame;
pub mod messages;

use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use noise::{NoiseFn, Simplex};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng, rng};
use rand_core::OsRng;
use russh::server::{Handle, Msg, Server as _, Session};
use russh::*;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio::time::{Instant, sleep};

use crate::frame::{Frame, read_term_data};
use crate::messages::generate_message;

#[tokio::main]
async fn main() {
    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![
            russh::keys::PrivateKey::random(&mut OsRng, russh::keys::Algorithm::Ed25519).unwrap(),
        ],
        ..Default::default()
    };
    let config = Arc::new(config);
    let mut sh = SshClientManager::new();

    let socket = TcpListener::bind(("0.0.0.0", 2222)).await.unwrap();
    let server = sh.run_on_socket(config, &socket);
    server.await.unwrap()
}

struct SshClientManager {}
impl SshClientManager {
    pub fn new() -> Self {
        SshClientManager {}
    }
}

struct SshClientHandler {
    sessions: Vec<SessionHandlerWrapper>,
    ip: IpAddr,
    user: String,
}
struct SessionHandlerWrapper {
    session_handler: SessionHandler,
    data: SessionData,
}
#[derive(Clone)]
struct SessionData {
    user: String,
    chanel_id: ChannelId,
    ip: IpAddr,
    exit_window: Arc<RwLock<bool>>,
}
enum SessionHandler {
    NonPty(NonPtyHandler),
    Pty(PtyHandler),
}
impl SshClientHandler {
    pub fn new(ip: IpAddr) -> Self {
        SshClientHandler {
            sessions: Vec::new(),
            ip,
            user: String::new(),
        }
    }
}

struct PtyData {
    term: String,
    col_width: u32,
    row_height: u32,
    pix_width: u32,
    pix_height: u32,
    modes: Vec<(Pty, u32)>,
}

impl server::Server for SshClientManager {
    type Handler = SshClientHandler;
    fn new_client(&mut self, addr: Option<std::net::SocketAddr>) -> SshClientHandler {
        SshClientHandler::new(addr.unwrap().ip())
    }
    fn handle_session_error(&mut self, _error: <Self::Handler as russh::server::Handler>::Error) {
        eprintln!("Session error: {_error:#?}");
    }
}

impl server::Handler for SshClientHandler {
    type Error = russh::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        self.sessions.push(SessionHandlerWrapper {
            session_handler: SessionHandler::NonPty(NonPtyHandler::new(
                session.handle(),
                channel.id(),
            )),
            data: SessionData {
                chanel_id: channel.id(),
                user: self.user.clone(),
                ip: self.ip.clone(),
                exit_window:Arc::new(RwLock::new(false))
            },
        });
        Ok(true)
    }
    async fn auth_none(&mut self, user: &str) -> Result<server::Auth, Self::Error> {
        self.user = user.to_string();
        Ok(server::Auth::Accept)
    }
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_success(channel)?;
        let session_handler_wrapper = self
            .sessions
            .iter_mut()
            .find(|x| x.data.chanel_id == channel)
            .unwrap();
        let pty_data = PtyData {
            term: term.to_string(),
            col_width,
            row_height,
            pix_width,
            pix_height,
            modes: modes.to_vec(),
        };
        match &session_handler_wrapper.session_handler {
            SessionHandler::NonPty(npty) => {
                npty.task_handle.abort();
                session_handler_wrapper.session_handler = SessionHandler::Pty(PtyHandler::new(
                    pty_data,
                    session.handle(),
                    session_handler_wrapper.data.clone(),
                ))
            }
            SessionHandler::Pty(pty) => {
                *pty.data.lock().await = pty_data;
            }
        }
        Ok(())
    }
    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_success(channel)?;
        let session_handler_wrapper = self
            .sessions
            .iter_mut()
            .find(|x| x.data.chanel_id == channel)
            .unwrap();
        if let SessionHandler::Pty(pty) = &session_handler_wrapper.session_handler {
            let mut lock = pty.data.lock().await;
            lock.col_width = col_width;
            lock.row_height = row_height;
            lock.pix_width = pix_width;
            lock.pix_height = pix_height;
        }
        Ok(())
    }
    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let session_handler_wrapper = self
            .sessions
            .iter()
            .find(|x| x.data.chanel_id == channel)
            .unwrap();
        if data == [3] {
            let session_handle = session.handle();
            match &session_handler_wrapper.session_handler {
                SessionHandler::NonPty(_) => {
                    panic!();
                }
                SessionHandler::Pty(_) => {
                    let exit_window=*session_handler_wrapper.data.exit_window.read().await;
                    tokio::spawn(async move {
                        session_handle
                            .data(
                                channel,
                                CryptoVec::from(format!("\x1b[0m\x1b[?25h{}", match exit_window{true=>"\x1b[?1049l", false=>""})),
                            )
                            .await
                            .unwrap();
                        sleep(Duration::from_millis(1)).await;
                        session_handle.close(channel).await.unwrap();
                    });
                }
            }
            return Ok(());
        }
        Ok(())
    }
}
struct NonPtyHandler {
    task_handle: JoinHandle<()>,
}
impl NonPtyHandler {
    pub fn new(session: Handle, channel: ChannelId) -> Self {
        NonPtyHandler {
            task_handle: tokio::spawn(async move {
                session
                    .data(
                        channel,
                        CryptoVec::from(format!("waiting for ssh client to request pty...\n\r")),
                    )
                    .await
                    .unwrap();
            }),
        }
    }
}
struct PtyHandler {
    data: Arc<Mutex<PtyData>>,
    task_handle: JoinHandle<()>,
}
impl PtyHandler {
    pub fn new(data: PtyData, session: Handle, session_data: SessionData) -> Self {
        let data = Arc::new(Mutex::new(data));
        PtyHandler {
            data: data.clone(),
            task_handle: tokio::spawn(async move {
                match session_data.user.as_str() {
                    "virus" => {
                        let _ = status_mmessages(session, session_data.chanel_id).await;
                    }
                    "weather" => {
                        let _ = weather(session, session_data.chanel_id, data, &session_data.user, session_data.ip).await;
                    }
                    s => {
                        let _ = help(session, session_data.chanel_id, &session_data.user).await;
                    }
                }
            }),
        }
    }
}
async fn help(session: Handle, channel: ChannelId, username: &str) -> Result<(), CryptoVec> {
    session
        .data(channel, CryptoVec::from(format!("Hello!\n\rThis server uses the ssh username as a way to communicate what should be sent. You have connected with the username \"{username}\". If this is your actual name, don't worry, it won't be saved / logged / sent anywhere.\n\rpossible usernames include:\n\r\"virus\"\n\r\"weather\"\n\r")))
        .await?;
    sleep(Duration::from_millis(1)).await;
    session.close(channel).await.unwrap();
    Ok(())
}
async fn status_mmessages(session: Handle, channel: ChannelId) -> Result<(), CryptoVec> {
    session
        .data(
            channel,
            CryptoVec::from(format!("\x1b[?1049h\x1b[?25l\x1b[2J\x1b[0;0H")),
        )
        .await?;
    let mut rng = SmallRng::from_rng(&mut rng());
    loop {
        let duration = rng.random_range::<f64, _>(0.0..1.0).powi(10) * 5.0;
        let start = Instant::now();
        let message = &format!("{}...  ", generate_message());
        session
        .data(channel, CryptoVec::from(format!("\x1b[0m{}", message)))
        .await?;
    while start.elapsed().as_millis() < (duration * 1000.0) as u128 {
        sleep(Duration::from_millis(
                (rng.random_range::<f32, _>(0.0..1.0).powi(5) * 200.0) as u64,
            ))
            .await;
        session
                .data(
                    channel,
                    CryptoVec::from(format!(
                        "\x1b[0m\x1b[{}G\x1b[0K{}%",
                        message.len(),
                        ((start.elapsed().as_millis() as f64) / (1000.0 * duration) * 100.0)
                        as u128
                    )),
                )
                .await?;
        }
        session
        .data(
                channel,
                CryptoVec::from(format!("\x1b[32m\x1b[{}G\x1b[0Kdone!\n\r", message.len(),)),
            )
            .await?;
    }
}
struct Particle{
    x:f64,
    y:f64,
    z:f64,
}
async fn weather(session: Handle, channel: ChannelId, data: Arc<Mutex<PtyData>>, username: &str, ip:IpAddr) -> Result<(), CryptoVec>{
    let fd=read_term_data();
    let d0=data.lock().await;
    let mut f=Frame::new(d0.col_width as usize, d0.row_height as usize, ());
    session
    .data(
        channel,
            CryptoVec::from(format!("\x1b[?1049h\x1b[?25l\x1b[2J\x1b[0;0H")),
        )
        .await?;
    /*
    puddles
    snow/rain drops/flakes
    calculate average movement from wind+gravity
    calculate camera edge planes
    for each camera edge plane:
    calculate area
    area * normal dot movement = amount of snowflakes to spawn in this area
    wind
    ripples
    grass?
    sun
    clouds
    */
    let mut particles:Vec<Particle>=vec![Particle{x:0.0, y:-1.0, z:1.0}];
    let mut rng = SmallRng::from_rng(&mut rng());
    let noise=noise::Simplex::new(0);
    let mut t=0.0;
    let windx=0.02;
    let windz=-0.02;
    loop {
        t+=1.0;
        let horizon_height=f.height as f64*0.3;
        let y3d=5.0;
        for y in 0..f.height{
            for x in 0..f.width{
                if y as f64 > horizon_height{
                    let z3d=f.width as f64/(y as f64-horizon_height) * y3d;
                    let x3d=(x as f64-f.width as f64/2.0) /(y as f64-horizon_height) * y3d;
                    let quantum_y3d=noise.get([x3d/30.0, z3d/30.0]);
                    if quantum_y3d>0.4{
                        f.set_texel(x, y, ('q', ())).unwrap();
                    }else{
                        f.set_texel(x, y, (' ', ())).unwrap();
                    }
                }else{
                    f.set_texel(x, y, (' ', ())).unwrap();
                }
            }
        }
        particles.push(Particle { x: rng.random_range(-10.0..10.0), y: -3.0, z: rng.random_range(0.0..7.0) });
        {
            let f=&mut f;
            let td=&fd;
            particles=particles.drain(..).filter_map(move |mut p| {
                p.y+=0.03;
                p.x+=windx;
                p.z+=windz;
                let curl=curl_noise_3d_t(&noise, (p.x-windx*t)/10.0, p.y/10.0, (p.z-windz*t)/10.0, t*0.003);
                p.x+=curl[0]/100.0;
                p.y+=curl[1]/100.0;
                p.z+=curl[2]/100.0;
                if p.y<y3d&&p.z>0.0{
                    let sx=p.x/p.z*f.height as f64/2.0+f.width as f64 / 2.0;
                    let sy=p.y/p.z*f.height as f64/2.0+horizon_height;
                    if sx>0.0&&sy>0.0{
                        let sx2=sx as usize;
                        let sy2=sy as usize;
                        // let _=f.set_texel(sx2, sy2, ('*',()));
                        let _=f.set_pixel(sx2, sy2, (1000.0/p.z/p.z) as u8, (1000.0/p.z/p.z) as u8, (1000.0/p.z/p.z) as u8, td);
                    }
                    Some(p)
                }else{
                    None
                }
            }).collect();
        }
        session
            .data(
                channel,
                CryptoVec::from(f.render_str()),
            )
            .await?;
        sleep(Duration::from_millis(30)).await;
    }
}
pub fn curl_noise_3d_t(noise:&Simplex, x:f64, y:f64, z:f64, t:f64)->[f64;3]{
    const D:f64=0.001;
    [
        (noise.get([x, y+D, z, t+2000.0])-noise.get([x, y-D, z, t+2000.0]))/2.0/D-(noise.get([x, y, z+D, t+1000.0])-noise.get([x, y, z-D, t+1000.0]))/2.0/D,
        (noise.get([x, y, z+D, t])-noise.get([x, y, z-D, t]))/2.0/D-(noise.get([x+D, y, z, t+2000.0])-noise.get([x-D, y, z, t+2000.0]))/2.0/D,
        (noise.get([x+D, y, z, t+1000.0])-noise.get([x-D, y, z, t+1000.0]))/2.0/D-(noise.get([x, y+D, z, t])-noise.get([x, y-D, z, t]))/2.0/D,
    ]
}