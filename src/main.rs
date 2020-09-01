extern crate piston_window;
extern crate image;

use piston_window::*;
use piston_window::Button::Keyboard;

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow = WindowSettings::new(
        "Lazuli Example", 
        [1920, 1080])
        .fullscreen(true)
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true)
        .build()
        .unwrap();

    let bg_main: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets\\bg_main.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let first: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets\\first.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let second: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets\\second.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let branch_one: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets\\branch_one.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let branch_two: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets\\branch_two.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let cover: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "assets\\cover.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let main_menu_tips_enter = String::from("按空格键进入游戏");
    let main_menu_tips_esc = String::from("按Esc键退出游戏");
    let main_menu_tips_space = String::from("在游戏中按空格键进行游戏");
    let first_scene_text = String::from("第一幕测试文本。");
    let second_scene_text = String::from("第二幕测试文本。");
    let branch_one_option_text = String::from("选项A：用于A选项的内容（按A键选择）");
    let branch_two_option_text = String::from("选项B：用于B选项的内容（按B键选择）");
    let branch_one_text = String::from("分支一的文本");
    let branch_two_text = String::from("分支二的文本");

    enum Scenes {
        MainMenu,
        FirstScene,
        SecondScene,
        SecondSceneSub,
        BranchOne,
        BranchTwo
    }

    let mut current_scene = Scenes::MainMenu;

    let mut glyphs = window.load_font("assets\\sarasa-term-sc-regular.ttf").unwrap();

    while let Some(event) = window.next() {
        match current_scene {
            Scenes::MainMenu => {
                window.draw_2d(&event, |context, gfx, device| {
                    clear(color::WHITE, gfx);
                    image(&bg_main, context.transform, gfx);

                    text::Text::new(20).draw(
                        &main_menu_tips_enter,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(810.0, 580.0), gfx
                    )
                    .unwrap();

                    text::Text::new(20).draw(
                        &main_menu_tips_esc,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(810.0, 620.0), gfx
                    ).unwrap();

                    text::Text::new(20).draw(
                        &main_menu_tips_space,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(810.0, 660.0), gfx
                    ).unwrap();
                    
                    glyphs.factory.encoder.flush(device);
                });

                if let Some(button) = event.press_args() {
                    if button == Keyboard(Key::Space) {
                        current_scene = Scenes::FirstScene;
                    }
                }
            }
            
            Scenes::FirstScene => {
                window.draw_2d(&event, |context, gfx, device| {
                   clear(color::WHITE, gfx);
                   image(&first, context.transform, gfx);
                   image(&cover, context.transform.trans(300.0, 480.0), gfx);

                   text::Text::new(20).draw(
                       &first_scene_text,
                       &mut glyphs,
                       &context.draw_state,
                       context.transform.trans(100.0, 880.0),
                       gfx
                    ).unwrap();

                    glyphs.factory.encoder.flush(device);
                });

                if let Some(button) = event.press_args() {
                    if button == Keyboard(Key::Space) {
                        current_scene = Scenes::SecondScene;
                    }
                }
            }

            Scenes::SecondScene => {
                window.draw_2d(&event, |context, gfx, device| {
                    clear(color::WHITE, gfx);
                    image(&second, context.transform, gfx);
                    image(&cover, context.transform.trans(300.0, 480.0), gfx);
 
                    text::Text::new(20).draw(
                        &second_scene_text,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(100.0, 880.0),
                        gfx
                    ).unwrap();
                    
                    glyphs.factory.encoder.flush(device);
                });

                if let Some(button) = event.press_args() {
                    if button == Keyboard(Key::Space) {
                        current_scene = Scenes::SecondSceneSub;
                    }
                }
            }

            Scenes::SecondSceneSub => {
                window.draw_2d(&event, |context, gfx, device| {
                    text::Text::new(20).draw(
                        &branch_one_option_text,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(750.0, 580.0),
                        gfx
                    ).unwrap();

                    text::Text::new(20).draw(
                        &branch_two_option_text,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(750.0, 620.0),
                        gfx
                    ).unwrap();
                    
                    glyphs.factory.encoder.flush(device);
                });

                if let Some(button) = event.press_args() {
                    if button == Keyboard(Key::A) {
                        current_scene = Scenes::BranchOne;
                    } else if button == Keyboard(Key::B) {
                        current_scene = Scenes::BranchTwo;
                    }
                }
            }

            Scenes::BranchOne => {
                window.draw_2d(&event, |context, gfx, device| {
                    clear(color::WHITE, gfx);
                    image(&branch_one, context.transform, gfx);
 
                    text::Text::new(20).draw(
                        &branch_one_text,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(100.0, 880.0),
                        gfx
                    ).unwrap();

                    text::Text::new(20).draw(
                        &main_menu_tips_esc,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(810.0, 620.0), gfx
                    ).unwrap();
                    
                    glyphs.factory.encoder.flush(device);
                });
            }

            Scenes::BranchTwo => {
                window.draw_2d(&event, |context, gfx, device| {
                    clear(color::WHITE, gfx);
                    image(&branch_two, context.transform, gfx);
 
                    text::Text::new(20).draw(
                        &branch_two_text,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(100.0, 880.0),
                        gfx
                    ).unwrap();

                    text::Text::new(20).draw(
                        &main_menu_tips_esc,
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(810.0, 620.0), gfx
                    ).unwrap();
                    
                    glyphs.factory.encoder.flush(device);
                });
            }
        }
    }
}