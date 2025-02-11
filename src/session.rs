use bevy::{
    ecs::system::{Commands, ResMut},
    log::{info, warn},
    state::state::NextState,
};
use bevy_ggrs::{
    ggrs::{self},
    Session,
};
use bevy_matchbox::{prelude::PeerId, MatchboxSocket};

use crate::GameState;

const MAX_PLAYER_COUNT: usize = 2;

// The first generic parameter, u8, is the input type: 4-directions + fire fits
// easily in a single byte
// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
pub type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

pub fn connect(mut commands: Commands) {
    let room = "ws://127.0.0.1:3536/coolgamer?next=2";
    info!("connecting to server: {room}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if socket.get_channel(0).is_err() {
        return; // we assume the game has already started
    }
    // check for new connections
    socket.update_peers();
    let players = socket.players();

    if players.len() < MAX_PLAYER_COUNT {
        return; // we need more players to get started
    }

    info!("nice! everyone is here. let's go!");

    let mut builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(MAX_PLAYER_COUNT)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        builder = match builder.add_player(player, i) {
            Ok(b) => b,
            Err(e) => {
                warn!("failed to add player to the session: {}", e);
                return;
            }
        }
    }

    let chan = match socket.take_channel(0) {
        Ok(chan) => chan,
        Err(e) => {
            info!("failed to take channel: {}", e);
            return;
        }
    };

    let session = match builder.start_p2p_session(chan) {
        Ok(s) => s,
        Err(e) => {
            warn!("failed to take channel: {}", e);
            return;
        }
    };

    commands.insert_resource(Session::P2P(session));
    next_state.set(GameState::InGame);
}
