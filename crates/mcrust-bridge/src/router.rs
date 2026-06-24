use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crossbeam_channel::Sender;
use mcrust_core::GameHandle;
use mcrust_protocol::{InboundEvent, JoinParams, OutboundCommand, Player, PlayerId};
use tracing::warn;

pub type SessionSink = Sender<OutboundCommand>;

#[derive(Clone)]
pub struct BridgeRouter {
    game_in: Sender<InboundEvent>,
    sessions: Arc<parking_lot::RwLock<HashMap<PlayerId, SessionSink>>>,
    next_player_id: Arc<AtomicU32>,
}

impl BridgeRouter {
    pub fn new(game: &GameHandle) -> Self {
        Self {
            game_in: game.inbound.clone(),
            sessions: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            next_player_id: Arc::new(AtomicU32::new(1)),
        }
    }

    pub fn register_session(&self, player_id: PlayerId, sink: SessionSink) {
        self.sessions.write().insert(player_id, sink);
    }

    pub fn unregister_session(&self, player_id: PlayerId) {
        self.sessions.write().remove(&player_id);
        let _ = self
            .game_in
            .send(InboundEvent::PlayerLeave { player_id });
    }

    pub fn player_join(&self, join: JoinParams, sink: SessionSink) -> PlayerId {
        let id = PlayerId(self.next_player_id.fetch_add(1, Ordering::Relaxed));
        self.register_session(id, sink);
        let player = Player {
            id,
            platform: join.platform,
            name: join.name,
            uuid: join.uuid,
            xuid: join.xuid,
            gamemode: join.gamemode,
        };
        let _ = self.game_in.send(InboundEvent::PlayerJoin { player });
        id
    }

    pub fn forward_inbound(&self, ev: InboundEvent) {
        let _ = self.game_in.send(ev);
    }

    pub fn session_count(&self) -> usize {
        self.sessions.read().len()
    }

    pub fn dispatch_outbound(&self, cmd: OutboundCommand) {
        match &cmd {
            OutboundCommand::Disconnect { player_id, .. }
            | OutboundCommand::KeepAlive { player_id, .. }
            | OutboundCommand::TeleportPlayer { player_id, .. } => {
                self.send_to(*player_id, cmd);
            }
            OutboundCommand::BroadcastMovement { player_id, .. } => {
                let sessions = self.sessions.read();
                for (id, tx) in sessions.iter() {
                    if *id != *player_id {
                        let _ = tx.send(cmd.clone());
                    }
                }
            }
            OutboundCommand::PlayerInfo {
                target_session,
                ..
            } => {
                self.send_to(*target_session, cmd);
            }
        }
    }

    fn send_to(&self, player_id: PlayerId, cmd: OutboundCommand) {
        if let Some(tx) = self.sessions.read().get(&player_id) {
            let _ = tx.send(cmd);
        } else {
            warn!(id = player_id.0, "no session for outbound command");
        }
    }
}

pub fn spawn_outbound_pump(game: GameHandle, router: BridgeRouter) {
    std::thread::spawn(move || loop {
        match game.outbound.recv() {
            Ok(cmd) => router.dispatch_outbound(cmd),
            Err(_) => break,
        }
    });
}