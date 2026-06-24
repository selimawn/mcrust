use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use mcrust_protocol::{InboundEvent, OutboundCommand, Player, PlayerId, Vec3f};
use tracing::{debug, info};

#[derive(Clone)]
pub struct GameConfig {
    pub spawn: Vec3f,
    pub tick_ms: u64,
}

pub struct GameHandle {
    pub inbound: Sender<InboundEvent>,
    pub outbound: Receiver<OutboundCommand>,
}

impl GameHandle {
    pub fn new(cfg: GameConfig) -> Self {
        run_game_loop(cfg)
    }
}

struct PlayerState {
    player: Player,
    position: Vec3f,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
    last_keepalive: i64,
}

pub fn run_game_loop(cfg: GameConfig) -> GameHandle {
    let (in_tx, in_rx) = crossbeam_channel::unbounded();
    let (out_tx, out_rx) = crossbeam_channel::unbounded();

    thread::spawn(move || {
        let mut players: HashMap<PlayerId, PlayerState> = HashMap::new();
        let mut next_id: u32 = 1;
        let tick = Duration::from_millis(cfg.tick_ms);
        let mut keepalive_counter: i64 = 0;

        loop {
            let frame_start = Instant::now();

            while let Ok(ev) = in_rx.try_recv() {
                match ev {
                    InboundEvent::PlayerJoin { mut player } => {
                        if player.id.0 == 0 {
                            player.id = PlayerId(next_id);
                            next_id += 1;
                        }
                        let id = player.id;
                        info!(player_id = id.0, name = %player.name, ?player.platform, "player joined world");
                        let state = PlayerState {
                            player: player.clone(),
                            position: cfg.spawn.clone(),
                            yaw: 0.0,
                            pitch: 0.0,
                            on_ground: true,
                            last_keepalive: 0,
                        };
                        for (other_id, other) in &players {
                            if *other_id != id {
                                let _ = out_tx.send(OutboundCommand::PlayerInfo {
                                    target_session: id,
                                    player: other.player.clone(),
                                    position: other.position.clone(),
                                });
                                let _ = out_tx.send(OutboundCommand::PlayerInfo {
                                    target_session: *other_id,
                                    player: player.clone(),
                                    position: cfg.spawn.clone(),
                                });
                            }
                        }
                        players.insert(id, state);
                        let _ = out_tx.send(OutboundCommand::TeleportPlayer {
                            player_id: id,
                            position: cfg.spawn.clone(),
                            yaw: 0.0,
                            pitch: 0.0,
                        });
                    }
                    InboundEvent::PlayerLeave { player_id } => {
                        players.remove(&player_id);
                        debug!(id = player_id.0, "player left world");
                    }
                    InboundEvent::PlayerInput {
                        player_id,
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                        on_ground,
                    } => {
                        if let Some(p) = players.get_mut(&player_id) {
                            p.position = Vec3f { x, y, z };
                            p.yaw = yaw;
                            p.pitch = pitch;
                            p.on_ground = on_ground;
                            let _ = out_tx.send(OutboundCommand::BroadcastMovement {
                                player_id,
                                position: p.position.clone(),
                                yaw,
                                pitch,
                                on_ground,
                            });
                        }
                    }
                    InboundEvent::KeepAliveAck {
                        player_id,
                        payload,
                    } => {
                        if let Some(p) = players.get_mut(&player_id) {
                            p.last_keepalive = payload;
                        }
                    }
                }
            }

            keepalive_counter = keepalive_counter.wrapping_add(1);
            for id in players.keys().copied().collect::<Vec<_>>() {
                let _ = out_tx.send(OutboundCommand::KeepAlive {
                    player_id: id,
                    payload: keepalive_counter,
                });
            }

            let elapsed = frame_start.elapsed();
            if elapsed < tick {
                thread::sleep(tick - elapsed);
            }
        }
    });

    GameHandle {
        inbound: in_tx,
        outbound: out_rx,
    }
}

pub fn default_spawn() -> Vec3f {
    Vec3f {
        x: 0.5,
        y: 64.0,
        z: 0.5,
    }
}