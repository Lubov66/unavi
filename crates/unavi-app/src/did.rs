use bevy::prelude::*;
use didkit::DIDMethod;

pub struct DidPlugin;

impl Plugin for DidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_user_did);
    }
}

#[derive(Resource)]
pub struct UserDID {
    pub did_key: String,
    key: didkit::JWK,
}

fn set_user_did(mut commands: Commands) {
    let key = match didkit::JWK::generate_ed25519() {
        Ok(key) => key,
        Err(err) => {
            error!("Failed to generate JWK key: {}", err);
            return;
        }
    };

    let source = didkit::Source::Key(&key);

    let did_key = match did_method_key::DIDKey.generate(&source) {
        Some(did) => did,
        None => {
            error!("Failed to generate DID");
            return;
        }
    };

    info!("User DID: {}", did_key);

    commands.insert_resource(UserDID { did_key, key });
}
