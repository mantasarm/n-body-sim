pub mod mover;
pub mod vec_math;
pub mod pattern_loader;
pub mod camera;
pub mod translations;

use std::{ops::AddAssign, f32::consts::PI};

use camera::Camera2D;
use mover::Mover;
use notan::{notan_main, AppState, prelude::{Graphics, App, WindowConfig, Color, RenderTexture, TextureFilter, Plugins, KeyCode}, draw::{DrawConfig, CreateDraw, DrawImages, DrawShapes}, egui::{EguiConfig, EguiPluginSugar, SidePanel, panel::Side, Slider, ComboBox, Layout, Align, Window, DragValue, ScrollArea, RichText}, math::Vec2};
use pattern_loader::load_selected_pattern;
use translations::Translations;

pub const TRAIL_TEX_WIDTH: i32 = (1920. * 8.) as i32;
pub const TRAIL_TEX_HEIGHT: i32 = (1080. * 8.) as i32;

pub const G: f32 = 10.0;

#[derive(AppState)]
struct State {
    planets: Vec<Mover>,
    trail_texture: RenderTexture,
    sim_speed: i32,
    show_trail: bool,
    show_bodies: bool,
    pattern: i32,
    chosen_pattern: i32,
    camera: Camera2D,
    camera_zoom: f32,
    paused: bool,
    trans: Translations,
    editor_enabled: bool,

    new_body_mass: f32,
    new_body_moveable: bool,
    new_body_dir: f32,
    new_body_force: f32
}

impl State {
    fn new(app: &mut App, gfx: &mut Graphics) -> Self {
        let mut planets = Vec::<Mover>::new();

        pattern_loader::load_selected_pattern(&mut planets, 6);
        
        Self {
            planets,
            trail_texture: gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap(),
            sim_speed: 1,
            show_trail: true,
            show_bodies: true,
            pattern: -1,
            chosen_pattern: 0,
            camera: Camera2D::new(0.0, 0.0, app.window().width() as f32, app.window().height() as f32),
            camera_zoom: 1.,
            paused: false,
            trans: Translations::new(),
            editor_enabled: false,

            new_body_mass: 3.,
            new_body_moveable: true,
            new_body_dir: 0.,
            new_body_force: 1.
        }
    }
}

fn update(app: &mut App, state: &mut State) {
    if !state.paused {
        for planet in state.planets.iter_mut() {
            planet.save_delta_pos();
        }
    
        for _ in 0..state.sim_speed {
            for i in 0..state.planets.len() {
                let temp_mover = state.planets.get_mut(i).unwrap().clone();
                for j in 0..state.planets.len() {
                    if i != j {
                        temp_mover.attract(state.planets.get_mut(j).unwrap());
                    }
                }
            }
    
            for i in 0..state.planets.len() {
                state.planets.get_mut(i).unwrap().update(app);
            }
        }
    }

    if state.editor_enabled {
        if app.mouse.right_was_pressed() {
            let force = Vec2::from_angle(state.new_body_dir * PI / 180.) * state.new_body_force;
            let mouse_pos = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &state.camera);
            state.planets.push(Mover::new(mouse_pos.0, mouse_pos.1, state.new_body_mass, force.x, force.y).apply_forces(state.new_body_moveable));
        }
    }
    
    camera_control(app, &mut state.camera, &mut state.camera_zoom);
    manage_shortcuts(app, state);
}

fn manage_shortcuts(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.paused = !state.paused;
    } else if app.keyboard.was_pressed(KeyCode::F) {
        state.editor_enabled = !state.editor_enabled;
    } else if app.keyboard.is_down(KeyCode::T) {
        state.new_body_dir += 1.;
    } else if app.keyboard.is_down(KeyCode::R) {
        state.new_body_dir -= 1.;
    }
}

fn camera_control(app: &mut App, camera: &mut Camera2D, camera_zoom: &mut f32) {
    let mut speed = 5.;
    if app.keyboard.shift() {
        speed = 10.;
    }
    if app.keyboard.is_down(KeyCode::D) {
        camera.pos_add_x(speed);
    }
    if app.keyboard.is_down(KeyCode::A) {
        camera.pos_add_x(-speed);
    }
    if app.keyboard.is_down(KeyCode::S) {
        camera.pos_add_y(speed);
    }
    if app.keyboard.is_down(KeyCode::W) {
        camera.pos_add_y(-speed);
    }
    camera.set_zoom(*camera_zoom);

    if app.keyboard.is_down(KeyCode::Q) {
        *camera_zoom -= *camera_zoom * app.timer.delta_f32();
    }
    if app.keyboard.is_down(KeyCode::E) {
        *camera_zoom += *camera_zoom * app.timer.delta_f32();
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut trail_draw = state.trail_texture.create_draw();
    for planet in &state.planets {
         planet.render_trail(&mut trail_draw);
    }
    gfx.render_to(&state.trail_texture, &trail_draw);

    let mut draw = gfx.create_draw();
    draw.clear(Color::from_hex(0x252526FF));

    state.camera.apply(&mut draw);
    if state.show_trail {
        draw.image(&state.trail_texture.texture()).position(-TRAIL_TEX_WIDTH as f32 / 2., -TRAIL_TEX_HEIGHT as f32 / 2.);
    }

    if state.show_bodies {
        for planet in &state.planets {
            planet.render(&mut draw);
        }
    }

    if state.editor_enabled {
        let mouse_pos = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &state.camera);

        draw.ellipse((mouse_pos.0, mouse_pos.1), (state.new_body_mass.sqrt() * 10., state.new_body_mass.sqrt() * 10.)).color(Color::from_rgba(1., 1., 1., 0.5));

        let mut second_point = Vec2::from_angle(state.new_body_dir * PI / 180.) * state.new_body_force * 40.;
        second_point.add_assign(Vec2::new(mouse_pos.0, mouse_pos.1));

        draw.line((mouse_pos.0, mouse_pos.1), (second_point.x, second_point.y)).color(Color::BLUE);
    }

    gfx.render(&draw);
    

    if state.chosen_pattern != state.pattern {
        clear_trail_texture(&mut state.trail_texture, gfx);
        state.chosen_pattern = state.pattern;
        state.camera.set_position(0., 0.);
        state.camera.set_zoom(1.0);
        state.camera_zoom = 1.0;
        if state.chosen_pattern != -1 {
            load_selected_pattern(&mut state.planets, state.pattern);
            state.editor_enabled = false;
        } else {
            state.planets.clear();
            state.editor_enabled = true;
        }
    }

    let output = plugins.egui(|ctx| {
        SidePanel::new(Side::Left, "left_panel").resizable(false).min_width(app.window().width() as f32 / 8.).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(state.trans.get("settings"));
            });
            ui.separator();

            ui.label(state.trans.get("chooselang"));
            ComboBox::from_label("  ").selected_text(state.trans.chosen_lang.as_str()).show_ui(ui, |ui| {
               ui.selectable_value(&mut state.trans.chosen_lang, "en".to_string(), "English");
               ui.selectable_value(&mut state.trans.chosen_lang, "lt".to_string(), "Lietuvių");
            });
            ui.add_space(20.);

            ui.label(state.trans.get("choose"));
            let txt = if state.chosen_pattern != -1 {
                format!("{}{}", state.trans.get("pattern"), state.chosen_pattern)
            } else {
                state.trans.get("create")
            };
            ComboBox::from_label(" ").selected_text(&txt).show_ui(ui, |ui| {
                ui.selectable_value(&mut state.pattern, -1, state.trans.get("create"));
                for i in 0..=9 {
                    ui.selectable_value(&mut state.pattern, i, format!("{}{}", state.trans.get("pattern"), i));
                }
            });
            ui.add_space(20.);

            let slider = Slider::new(&mut state.sim_speed, 1..=25).text(state.trans.get("simspeed"));
            ui.add(slider);
            
            if ui.checkbox(&mut state.show_trail, state.trans.get("showtrail")).clicked() {
                clear_trail_texture(&mut state.trail_texture, gfx);
            }
            ui.checkbox(&mut state.show_bodies, state.trans.get("showbodies"));

            ui.checkbox(&mut state.paused, state.trans.get("pause"));

            ui.checkbox(&mut state.editor_enabled, state.trans.get("editor"));

            if ui.button(state.trans.get("restart")).clicked() {
                if state.chosen_pattern != -1 {
                    load_selected_pattern(&mut state.planets, state.pattern);
                }
                clear_trail_texture(&mut state.trail_texture, gfx);
            }

            ui.add_space(20.);
            ui.label(state.trans.get("wasd"));
            ui.label(state.trans.get("qe"));

            // Removing objects from sim
            ui.separator();
            if state.planets.len() > 0 {
                ui.label(RichText::new(state.trans.get("bodiesinsim")).size(15.));
                ScrollArea::vertical().auto_shrink([false, false]).max_width(f32::INFINITY).max_height(app.window().height() as f32 / 1.5).show(ui, |ui| {
                    for i in 0..state.planets.len() {
                        ui.label(format!("{} {}", state.trans.get("obj"), i));
                        let btn =  ui.button(state.trans.get("remove"));
                        if btn.clicked() {
                            state.planets.remove(i);
                            break;
                        } else if btn.hovered() {
                            state.planets.get_mut(i).unwrap().selected = true;
                        } else {
                            state.planets.get_mut(i).unwrap().selected = false;
                        }
                        ui.label(format!("{} (x, y): ({:.1}, {:.1})", state.trans.get("vel"), state.planets.get(i).unwrap().vel.x, state.planets.get(i).unwrap().vel.y));
                        ui.label(format!("{} (x, y): ({}, {})", state.trans.get("pos"), state.planets.get(i).unwrap().pos.x as i32, state.planets.get(i).unwrap().pos.y as i32));
                        ui.label(format!("{}: {}", state.trans.get("mass"), state.planets.get(i).unwrap().m));
                        

                        ui.add_space(20.);
                    }
                });
                ui.separator();
            }

            // Credits
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add_space(20.);
                ui.hyperlink("https://github.com/mantasarm/n-body-sim");
                ui.label(state.trans.get("credits"));
            });

        });

        if state.editor_enabled {
            Window::new(state.trans.get("addbodies")).resizable(false).show(ctx, |ui| {
                
                let drag_mass = DragValue::new(&mut state.new_body_mass).prefix(state.trans.get("bodymass")).clamp_range(0.000000001..=10000.0).speed(0.1);
                ui.add(drag_mass);

                ui.checkbox(&mut state.new_body_moveable, state.trans.get("moveable"));

                let drag_dir = DragValue::new(&mut state.new_body_dir).prefix(state.trans.get("dir")).clamp_range(0.0..=360.0).speed(1.0);
                ui.add(drag_dir);

                let drag_force = DragValue::new(&mut state.new_body_force).prefix(state.trans.get("initf")).clamp_range(0.000000000001..=30.0).speed(0.1);
                ui.add(drag_force);

                ui.add_space(20.);

                if ui.button(state.trans.get("clear")).clicked() {
                    state.planets.clear();
                    clear_trail_texture(&mut state.trail_texture, gfx);
                }

                ui.label(state.trans.get("rclick"));
            });
        }
    });
    
    gfx.render(&output);
}

fn clear_trail_texture(trail_tex: &mut RenderTexture, gfx: &mut Graphics) {
    let mut draw = trail_tex.create_draw();
    draw.clear(Color::from_hex(0x252526FF));
    gfx.render_to(&trail_tex, &draw);
}

fn get_mouse_in_world(mouse_pos: &(f32, f32), window_size: (i32, i32), camera: &Camera2D) -> (f32, f32) {
    let mouse_x = map(&mouse_pos.0, 0.0, window_size.0 as f32, 0.0, camera.work_size.x / camera.scale().x);
    let mouse_y = map(&mouse_pos.1, 0.0, window_size.1 as f32, 0.0, camera.work_size.y / camera.scale().y);

    (camera.pos.x - camera.work_size.x / 2.0 / camera.scale().x + mouse_x, camera.pos.y - camera.work_size.y / 2.0 / camera.scale().y + mouse_y)
}

fn map(value: &f32, begin: f32, end: f32, new_begin: f32, new_end: f32) -> f32 {
    new_begin + (new_end - new_begin) * ((value - begin) / (end - begin))
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(WindowConfig::new().vsync(true).title("Gravitacija").multisampling(4).resizable(true).maximized(true))
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}