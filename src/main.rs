use rusty_engine::{game, prelude::*};
use rand::prelude::*;


#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
    in_menu: bool, 
    in_color_selection: bool,
    selected_color: Option<String>,
    selected_color2: Option<String>,
    
}

fn main() {
    let mut game = Game::new();
    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
        in_menu: true,
        selected_color: None,
        selected_color2: None,
        in_color_selection: false,
    });

    // game setup goes here
    

    
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    for i in 0..50{
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierRed);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }


    
    


    let obstacle_preset = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingCarRed,
        SpritePreset::RollingBlockSmall,
        SpritePreset::RollingBallBlue,
        SpritePreset::RollingBallRed,
        SpritePreset::RollingBlockNarrow,
        SpritePreset::RollingBlockSquare,
        
       
        
        
    ];
    for(i, preset) in obstacle_preset.into_iter().enumerate(){
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);

    }

    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(450.0, 320.0);

}
const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.in_menu {
        let menu_message = engine.add_text("menu", "Press Enter to Start\nPress C to Select Car Color\nPress Q to Quit");
        menu_message.translation = Vec2::new(0.0, 0.0);
        menu_message.font_size = 40.0;

        let instructions = engine.add_text("instructions", 
            "First Player plays with W and S and Second Player plays with Up and Down.");
        instructions.translation = Vec2::new(0.0, -100.0);
        instructions.font_size = 20.0;
        // Exit or Start Game from Menu
        if engine.keyboard_state.just_pressed(KeyCode::Q) {
            engine.should_exit = true;
        } 
        if engine.keyboard_state.just_pressed(KeyCode::Return) {
           game_state.in_menu = false;
           engine.texts.remove("menu");
           engine.texts.remove("instructions");
           start_game(engine, game_state);
        }
        
        if engine.keyboard_state.just_pressed(KeyCode::C) {

            game_state.in_menu = false;
            game_state.in_color_selection = true;
            engine.texts.remove("menu");
            engine.texts.remove("instructions");
            color_selection(engine, game_state);

        }

        
        return;
    }
    if game_state.in_color_selection {
        // Handle color selection input for both players
        if game_state.selected_color.is_none() {
            if engine.keyboard_state.just_pressed(KeyCode::Key1) {
                game_state.selected_color = Some("Black".to_string());
            } else if engine.keyboard_state.just_pressed(KeyCode::Key2) {
                game_state.selected_color = Some("Blue".to_string());
            } else if engine.keyboard_state.just_pressed(KeyCode::Key3) {
                game_state.selected_color = Some("Green".to_string());
            } else if engine.keyboard_state.just_pressed(KeyCode::Key4) {
                game_state.selected_color = Some("Yellow".to_string());
            }
        }

        // Player 2 color selection
        if game_state.selected_color2.is_none() {
            if engine.keyboard_state.just_pressed(KeyCode::Key5) {
                game_state.selected_color2 = Some("Black".to_string());
            } else if engine.keyboard_state.just_pressed(KeyCode::Key6) {
                game_state.selected_color2 = Some("Blue".to_string());
            } else if engine.keyboard_state.just_pressed(KeyCode::Key7) {
                game_state.selected_color2 = Some("Green".to_string());
            } else if engine.keyboard_state.just_pressed(KeyCode::Key8) {
                game_state.selected_color2 = Some("Yellow".to_string());
            }
        }

        // Check if both players have selected colors
        if game_state.selected_color.is_some() && game_state.selected_color2.is_some() {
            engine.texts.remove("color_selection"); // Remove color selection text
            start_game(engine, game_state); // Start the game with selected colors
        }
        return;
    }

    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }

    
    if game_state.lost{
        if engine.keyboard_state.just_pressed(KeyCode::R) {
            // Reset game state
            game_state.health_amount = 5;
            game_state.lost = false;
            
           

            // Reset player position
            let player1 = engine.sprites.get_mut("player1").unwrap();
            player1.translation.y = 150.0;
            player1.rotation = 0.0;
            let player1 = engine.sprites.get_mut("player2").unwrap();
            player1.translation.y = -150.0;
            player1.rotation = 0.0;

            // Reset obstacles
            for sprite in engine.sprites.values_mut() {
                if sprite.label.starts_with("obstacle") {
                    sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                    sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
                }
            }

            // Update health message
            let health_message = engine.texts.get_mut("health_message").unwrap();
            health_message.value = format!("Health: {}", game_state.health_amount);
            
            

            // Clear 'Game Over' text
            engine.texts.remove("game_over");
            engine.texts.remove("game_message");

            // Restart background music
            engine.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);
        }

        return;
        
    }
    
    

    

    
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::W){
        direction +=1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::S){
        direction -=1.0;
    }

    let mut direction_2 = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up){
        direction_2 +=1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down){
        direction_2 -=1.0;
    }



    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -360.0 || player1.translation.y > 360.0{
        game_state.health_amount = 0;
    }
    let player2 = engine.sprites.get_mut("player2").unwrap();
    player2.translation.y += direction_2 * PLAYER_SPEED * engine.delta_f32;
    player2.rotation = direction_2 * 0.15;
    if player2.translation.y < -360.0 || player2.translation.y > 360.0{
        game_state.health_amount = 0;
    }

    for sprite in engine.sprites.values_mut(){
        if sprite.label.starts_with("roadline"){
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0{
                sprite.translation.x += 1500.0;
            }
        }
        if sprite.label.starts_with("obstacle"){
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0{
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);

            }
        }
        
            if sprite.label.starts_with("tree_top") || sprite.label.starts_with("tree_bottom") {
                sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
                if sprite.translation.x < -800.0 {
                    sprite.translation.x += 1600.0; // Wrap to the right edge when moving left
                }
            }
        
        
            
        
        
    }
    
    
    


    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..){
        if !event.pair.either_contains("player2") &&  !event.pair.either_contains("player1") || event.state.is_end(){
            continue;
        }
        if game_state.health_amount > 0{
            game_state.health_amount -=1;
            health_message.value = format!("Healths: {}", game_state.health_amount);
            
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }
    if game_state.health_amount == 0{
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "GAME OVER!" );
        game_over.font_size = 90.0;
        let game_message = engine.add_text("game_message", "Press R to restart the game\nPress Q to quit the game");
        game_message.font_size = 50.0;
        game_message.translation = Vec2::new(0.0, -100.0);
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
fn color_selection(engine: &mut Engine, game_state: &mut GameState) {
    let color_selection_message = engine.add_text("color_selection", "Player 1: Press 1 for Black, 2 for Blue, 3 for Green, 4 for Yellow.\nPlayer 2: Press 5 for Black, 6 for Blue, 7 for Green, 8 for Yellow.");
    color_selection_message.translation = Vec2::new(0.0, 0.0);
    color_selection_message.font_size = 30.0;
}


fn start_game(engine: &mut Engine, game_state: &mut GameState) {
    // Set health and game state
    
    game_state.health_amount = 5;
    game_state.lost = false;
    game_state.in_color_selection = false;

    // Remove menu text
    

    // Select car color for player 1 based on menu choice
    let player1_color = match game_state.selected_color.as_ref().map(String::as_str) {
        Some("Black") => SpritePreset::RacingCarBlack,
        Some("Blue") => SpritePreset::RacingCarBlue,
        Some("Green") => SpritePreset::RacingCarGreen,
        Some("Yellow") => SpritePreset::RacingCarYellow,
        _ => SpritePreset::RacingCarBlack, // Default color
    };

    // Initialize player sprites
    let player1 = engine.add_sprite("player1", player1_color);
    player1.translation.x = -500.0;
    player1.translation.y = 150.0;
    player1.layer = 10.0;
    player1.collision = true;

    let player2_color = match game_state.selected_color2.as_ref().map(String::as_str) {
        Some("Black") => SpritePreset::RacingCarBlack,
        Some("Blue") => SpritePreset::RacingCarBlue,
        Some("Green") => SpritePreset::RacingCarGreen,
        Some("Yellow") => SpritePreset::RacingCarYellow,
        _ => SpritePreset::RacingCarBlue, // Default color
    };

    let player2 = engine.add_sprite("player2", player2_color);
    player2.translation.x = -500.0;
    player2.translation.y = -150.0;
    player2.layer = 10.0;
    player2.collision = true;

    // Play background music
    engine.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Add road lines
    for i in 0..50 {
        let roadline = engine.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierRed);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }
    for i in 0..10 {
        // Add trees along the top edge
        let tree_top = engine.add_sprite(format!("tree_top{}", i), "tree.png"); // Assuming you load the custom tree image here
        tree_top.translation = Vec2::new(-600.0 + i as f32 * 200.0, 450.0); // Adjust `y` position as needed
        tree_top.scale = 0.3;
        tree_top.layer = 0.5;
        tree_top.collision = true;
    
        // Add trees along the bottom edge
        let tree_bottom = engine.add_sprite(format!("tree_bottom{}", i),   "tree.png");
        tree_bottom.translation = Vec2::new(-600.0 + i as f32 * 200.0, -450.0); // Adjust `y` position as needed
        tree_bottom.scale = 0.3;
        tree_bottom.layer = 0.5;
        tree_bottom.collision = true;
    }
    

    // Add obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingCarRed,
        SpritePreset::RollingBlockSmall,
        SpritePreset::RollingBallBlue,
        SpritePreset::RollingBallRed,
        SpritePreset::RollingBlockNarrow,
        SpritePreset::RollingBlockSquare,


    ];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = engine.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // Health message
    let health_message = engine.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(450.0, 320.0);
}