use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    online: bool,
}

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
}

#[reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: true,
            ..player
        });
    } else {
        ctx.db.player().insert(Player {
            identity: ctx.sender,
            position_x: 0.,
            position_y: 0.,
            position_z: 0.,
            online: true,
        });
    }
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: false,
            ..player
        });
    } else {
        // This branch should be unreachable,
        // as it doesn't make sense for a client to disconnect without connecting first.
        log::warn!(
            "Disconnect event for unknown player with identity {:?}",
            ctx.sender
        );
    }
}

#[reducer]
pub fn set_position(
    ctx: &ReducerContext,
    position_x: f32,
    position_y: f32,
    position_z: f32,
) -> Result<(), String> {
    if let Some(player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            position_x,
            position_y,
            position_z,
            ..player
        });
        Ok(())
    } else {
        Err("Cannot set position for unknown player".to_string())
    }
}

// #[reducer]
// pub fn say_hello(ctx: &ReducerContext) {
//     for person in ctx.db.person().iter() {
//         log::info!("Hello, {}!", person.name);
//     }
//     log::info!("Hello, World!");
// }
