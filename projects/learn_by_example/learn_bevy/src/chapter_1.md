# Chapter 1

```rust
/// Marker for the player
#[derive(Component)]
struct Player;

/// Bundle to make it easy to spawn the player entity
/// with all the correct components:
#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    health: Health,
    xp: Xp,
    // including all the components from another bundle
    sprite: SpriteBundle,
}

fn spawn_player(
    // needed for safely creating/removing data in the ECS World
    // (anything done via Commands will be applied later)
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
) {
    // create a new entity with whatever components we want
    commands.spawn(PlayerBundle {
        marker: Player,
        health: Health {
            current: 100,
            max: 125,
        },
        xp: Xp(0),
        sprite: SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    });

    // Call .id() if you want to store the Entity ID of your new entity
    let my_entity = commands.spawn((/* ... */)).id();
}
```