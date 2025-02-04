use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

pub fn change_name(mut query: Query<&mut Name, With<Transform>>, mut rng: GlobalEntropy<WyRand>) {
    query
        .single_mut()
        .set(format!("Changed {}", rng.next_u32()));
}
