pub mod mover;
pub mod vec_math;
pub mod pattern_loader;
pub mod camera;
pub mod translations;
pub mod input_manager;

use std::{ops::AddAssign, f32::consts::PI};

use camera::Camera2D;
use mover::Mover;
use notan::{notan_main, AppState, prelude::{Graphics, App, WindowConfig, Color, RenderTexture, TextureFilter, Plugins}, draw::{DrawConfig, CreateDraw, DrawImages, DrawShapes}, egui::{EguiConfig, EguiPluginSugar, SidePanel, panel::Side, Slider, ComboBox, Layout, Align, Window, DragValue, ScrollArea, RichText}, math::Vec2};
use pattern_loader::PatternLoader;
use translations::Translations;

pub const TRAIL_TEX_WIDTH: i32 = (1920. * 8.) as i32;
pub const TRAIL_TEX_HEIGHT: i32 = (1080. * 8.) as i32;

pub const G: f32 = 10.0;

#[derive(AppState)]
pub struct State {
    planets: Vec<Mover>,
    trail_texture: RenderTexture,
    editor_info: EditorInfo,
    pattern_loader: PatternLoader,
    camera: Camera2D,
    camera_zoom: f32,
    trans: Translations,
    new_body: NewBodyInfo,
    object_tracking: Option<usize>
}

struct NewBodyInfo {
    mass: f32,
    moveable: bool,
    dir: f32,
    force: f32
}

struct EditorInfo {
    sim_speed: i32,
    show_trail: bool,
    show_bodies: bool,
    paused: bool,
    editor_enabled: bool
}

impl State {
    fn new(app: &mut App, gfx: &mut Graphics) -> Self {
        let mut planets = Vec::<Mover>::new();

        let pattern_loader = PatternLoader::new();
        pattern_loader.load_pattern(&mut planets, 7);
        
        Self {
            planets,
            trail_texture: gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap(),
            editor_info: EditorInfo {
                sim_speed: 1,
                show_trail: true,
                show_bodies: true,
                paused: true,
                editor_enabled: false
            },
            pattern_loader,
            camera: Camera2D::new(0.0, 0.0, app.window().width() as f32, app.window().height() as f32),
            camera_zoom: 1.,
            trans: Translations::new(),
            new_body: NewBodyInfo {
                mass: 3.,
                moveable: true,
                dir: 0.,
                force: 1.
            },
            object_tracking: None
        }
    }
}

fn update(app: &mut App, state: &mut State) {
    if !state.editor_info.paused {
        for planet in state.planets.iter_mut() {
            planet.save_delta_pos();
        }
    
        for _ in 0..state.editor_info.sim_speed {
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

    match state.object_tracking {
        Some(i) => state.camera.set_position(state.planets.get(i).unwrap().pos.x, state.planets.get(i).unwrap().pos.y),
        _ => ()
    }

    if state.editor_info.editor_enabled {
        if app.mouse.right_was_pressed() {
            let force = Vec2::from_angle(state.new_body.dir * PI / 180.) * state.new_body.force;
            let mouse_pos = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &state.camera);
            state.planets.push(Mover::new(mouse_pos.0, mouse_pos.1, state.new_body.mass, force.x, force.y).apply_forces(state.new_body.moveable));
        }
    }
    
    input_manager::camera_control(app, &mut state.camera, &mut state.camera_zoom);
    input_manager::manage_shortcuts(app, state);
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
    if state.editor_info.show_trail {
        draw.image(&state.trail_texture.texture()).position(-TRAIL_TEX_WIDTH as f32 / 2., -TRAIL_TEX_HEIGHT as f32 / 2.);
    }

    if state.editor_info.show_bodies {
        for planet in &state.planets {
            planet.render(&mut draw);
        }
    }

    if state.editor_info.editor_enabled {
        let mouse_pos = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &state.camera);

        draw.ellipse((mouse_pos.0, mouse_pos.1), (state.new_body.mass.sqrt() * 10., state.new_body.mass.sqrt() * 10.)).color(Color::from_rgba(1., 1., 1., 0.5));

        let mut second_point = Vec2::from_angle(state.new_body.dir * PI / 180.) * state.new_body.force * 40.;
        second_point.add_assign(Vec2::new(mouse_pos.0, mouse_pos.1));

        draw.line((mouse_pos.0, mouse_pos.1), (second_point.x, second_point.y)).color(Color::BLUE);
    }

    gfx.render(&draw);
    

    if state.pattern_loader.handle_pattern_changes(&mut state.camera, &mut state.camera_zoom, &mut state.editor_info.editor_enabled) {
        clear_trail_texture(&mut state.trail_texture, gfx);
        state.pattern_loader.reload_pattern(&mut state.planets);
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
            let txt = if state.pattern_loader.chosen_pattern != 0 {
                format!("{}{}", state.trans.get("pattern"), state.pattern_loader.chosen_pattern)
            } else {
                state.trans.get("create")
            };
            ComboBox::from_label(" ").selected_text(&txt).show_ui(ui, |ui| {
                if ui.selectable_value(&mut state.pattern_loader.pattern, 0, state.trans.get("create")).clicked() {
                    state.object_tracking = None;
                }
                for i in 1..state.pattern_loader.patterns.len() {
                    if ui.selectable_value(&mut state.pattern_loader.pattern, i, format!("{}{}", state.trans.get("pattern"), i)).clicked(){
                        state.object_tracking = None;
                    }
                }
            });
            ui.add_space(20.);

            let slider = Slider::new(&mut state.editor_info.sim_speed, 1..=25).text(state.trans.get("simspeed"));
            ui.add(slider);
            
            if ui.checkbox(&mut state.editor_info.show_trail, state.trans.get("showtrail")).clicked() {
                clear_trail_texture(&mut state.trail_texture, gfx);
            }
            ui.checkbox(&mut state.editor_info.show_bodies, state.trans.get("showbodies"));

            ui.checkbox(&mut state.editor_info.paused, state.trans.get("pause"));

            ui.checkbox(&mut state.editor_info.editor_enabled, state.trans.get("editor"));

            if ui.button(state.trans.get("restart")).clicked() {
                state.pattern_loader.reload_pattern(&mut state.planets);
                clear_trail_texture(&mut state.trail_texture, gfx);
            }

            ui.add_space(20.);
            ui.label(state.trans.get("wasd"));
            ui.label(state.trans.get("qe"));

            // Removing objects from sim
            ui.separator();
            if state.planets.len() > 0 {
                ui.label(RichText::new(state.trans.get("bodiesinsim")).size(15.));
                if state.object_tracking.is_some() {
                    if ui.button(state.trans.get("untrack")).clicked() {
                        state.object_tracking = None;
                    }
                    ui.add_space(10.);
                }
                
                ScrollArea::vertical().auto_shrink([false, false]).max_width(f32::INFINITY).max_height(app.window().height() as f32 / 1.5).show(ui, |ui| {
                    for i in 0..state.planets.len() {
                        ui.label(format!("{} {}", state.trans.get("obj"), i));

                        let mut breakloop = false;

                        ui.horizontal(|ui| {
                            let removebtn =  ui.button(state.trans.get("remove"));
                            let trackbtn =  ui.button(state.trans.get("track"));
                            if removebtn.clicked() {
                                state.object_tracking = None;
                                state.planets.remove(i);
                                breakloop = true;
                            } else if removebtn.hovered() || trackbtn.hovered() {
                                state.planets.get_mut(i).unwrap().selected = true;
                            } else {
                                state.planets.get_mut(i).unwrap().selected = false;
                            }

                            if trackbtn.clicked() {
                                state.object_tracking = Some(i);
                            }
                        });

                        if breakloop {
                            break;
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

        if state.editor_info.editor_enabled {
            Window::new(state.trans.get("addbodies")).resizable(false).show(ctx, |ui| {
                
                let drag_mass = DragValue::new(&mut state.new_body.mass).prefix(state.trans.get("bodymass")).clamp_range(0.000001..=10000.0).speed(0.1);
                ui.add(drag_mass);

                ui.checkbox(&mut state.new_body.moveable, state.trans.get("moveable"));

                let drag_dir = DragValue::new(&mut state.new_body.dir).prefix(state.trans.get("dir")).clamp_range(0.0..=360.0).speed(1.0);
                ui.add(drag_dir);

                let drag_force = DragValue::new(&mut state.new_body.force).prefix(state.trans.get("initf")).clamp_range(0.000001..=10000.0).speed(0.1);
                ui.add(drag_force);

                ui.add_space(20.);

                if ui.button(state.trans.get("clear")).clicked() {
                    state.planets.clear();
                    clear_trail_texture(&mut state.trail_texture, gfx);
                }

                let pos = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &state.camera);
                ui.label(format!("{} (x, y): ({}, {})", state.trans.get("pos"), pos.0 as i32, pos.1 as i32));

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