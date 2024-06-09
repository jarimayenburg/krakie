use macroquad::prelude::*;

use macroquad_tiled as tiled;

use macroquad_platformer::*;

struct Player {
    collider: Actor,
    speed: Vec2,
}

#[macroquad::main("Krakie")]
async fn main() {
    let tileset_bank = load_texture("assets/tilesets/bank.png").await.unwrap();
    tileset_bank.set_filter(FilterMode::Nearest);

    let tileset_krakie = load_texture("assets/tilesets/krakie_character.png")
        .await
        .unwrap();
    tileset_krakie.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/maps/bank.json").await.unwrap();
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[
            ("../tilesets/bank.png", tileset_bank),
            ("../tilesets/krakie_character.png", tileset_krakie),
        ],
        &[],
    )
    .unwrap();

    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("banklayer-1", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 32., 32., 30, 1);

    let mut player = Player {
        collider: world.add_actor(vec2(50.0, 80.0), 32, 64),
        speed: vec2(0., 0.),
    };

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 640.0, 960.0, -640.0));

    loop {
        clear_background(WHITE);

        set_camera(&camera);

        tiled_map.draw_tiles("banklayer-1", Rect::new(0.0, 0.0, 960.0, 640.0), None);

        // draw player
        {
            // sprite id from tiled
            const PLAYER_SPRITE: u32 = 6;

            let pos = world.actor_pos(player.collider);

            tiled_map.spr(
                "krakie_character",
                PLAYER_SPRITE,
                Rect::new(pos.x, pos.y, 32.0, 64.0),
            );
        }

        // player movement control
        {
            let pos = world.actor_pos(player.collider);
            let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

            if on_ground == false {
                player.speed.y += 2000. * get_frame_time();
            }

            if is_key_down(KeyCode::Right) {
                player.speed.x = 400.0;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -400.0;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) {
                if on_ground {
                    player.speed.y = -480.;
                }
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }

        next_frame().await
    }
}
