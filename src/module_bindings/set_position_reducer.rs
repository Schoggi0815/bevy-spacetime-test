// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct SetPositionArgs {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
}

impl From<SetPositionArgs> for super::Reducer {
    fn from(args: SetPositionArgs) -> Self {
        Self::SetPosition {
            position_x: args.position_x,
            position_y: args.position_y,
            position_z: args.position_z,
        }
    }
}

impl __sdk::InModule for SetPositionArgs {
    type Module = super::RemoteModule;
}

pub struct SetPositionCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `set_position`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait set_position {
    /// Request that the remote module invoke the reducer `set_position` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_set_position`] callbacks.
    fn set_position(&self, position_x: f32, position_y: f32, position_z: f32) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `set_position`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`SetPositionCallbackId`] can be passed to [`Self::remove_on_set_position`]
    /// to cancel the callback.
    fn on_set_position(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &f32, &f32, &f32) + Send + 'static,
    ) -> SetPositionCallbackId;
    /// Cancel a callback previously registered by [`Self::on_set_position`],
    /// causing it not to run in the future.
    fn remove_on_set_position(&self, callback: SetPositionCallbackId);
}

impl set_position for super::RemoteReducers {
    fn set_position(&self, position_x: f32, position_y: f32, position_z: f32) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "set_position",
            SetPositionArgs {
                position_x,
                position_y,
                position_z,
            },
        )
    }
    fn on_set_position(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &f32, &f32, &f32) + Send + 'static,
    ) -> SetPositionCallbackId {
        SetPositionCallbackId(self.imp.on_reducer(
            "set_position",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer:
                                super::Reducer::SetPosition {
                                    position_x,
                                    position_y,
                                    position_z,
                                },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, position_x, position_y, position_z)
            }),
        ))
    }
    fn remove_on_set_position(&self, callback: SetPositionCallbackId) {
        self.imp.remove_on_reducer("set_position", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `set_position`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_set_position {
    /// Set the call-reducer flags for the reducer `set_position` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn set_position(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_set_position for super::SetReducerFlags {
    fn set_position(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("set_position", flags);
    }
}
