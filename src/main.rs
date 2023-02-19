pub mod mover;
pub mod vec_math;
pub mod pattern_loader;
pub mod camera;
pub mod translations;

use camera::Camera2D;
use mover::Mover;
use notan::{notan_main, AppState, prelude::{Graphics, App, WindowConfig, Color, RenderTexture, TextureFilter, Plugins, KeyCode}, draw::{DrawConfig, CreateDraw, DrawImages}, egui::{EguiConfig, EguiPluginSugar, SidePanel, panel::Side, Slider, ComboBox, Layout, Align}};
use pattern_loader::load_selected_pattern;
use translations::Translations;

pub const TRAIL_TEX_WIDTH: i32 = 1920 * 6;
pub const TRAIL_TEX_HEIGHT: i32 = 1080 * 6;

#[derive(AppState)]
struct State {
    planets: Vec<Mover>,
    trail_texture: RenderTexture,
    sim_speed: i32,
    show_trail: bool,
    show_bodies: bool,
    g_constant: f32,
    pattern: i32,
    chosen_pattern: i32,
    camera: Camera2D,
    camera_zoom: f32,
    paused: bool,
    translations: Translations,
    chosen_lang: String
}

impl State {
    fn new(app: &mut App, gfx: &mut Graphics) -> Self {
        let mut planets = Vec::<Mover>::new();

        pattern_loader::load_selected_pattern(&mut planets, app, 0);
        
        Self {
            planets,
            trail_texture: gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap(),
            sim_speed: 1,
            show_trail: true,
            show_bodies: true,
            g_constant: 10.,
            pattern: 2,
            chosen_pattern: 0,
            camera: Camera2D::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., app.window().width() as f32, app.window().height() as f32),
            camera_zoom: 1.,
            paused: false,
            translations: Translations::new(),
            chosen_lang: "en".to_string()
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
                        temp_mover.attract(state.planets.get_mut(j).unwrap(), state.g_constant);
                    }
                }
            }
    
            for i in 0..state.planets.len() {
                state.planets.get_mut(i).unwrap().update(app);
            }
        }
    }
    
    let mut speed = 5.;
    if app.keyboard.shift() {
        speed = 10.;
    }
    if app.keyboard.is_down(KeyCode::D) {
        state.camera.pos_add_x(speed);
    }
    if app.keyboard.is_down(KeyCode::A) {
        state.camera.pos_add_x(-speed);
    }
    if app.keyboard.is_down(KeyCode::S) {
        state.camera.pos_add_y(speed);
    }
    if app.keyboard.is_down(KeyCode::W) {
        state.camera.pos_add_y(-speed);
    }
    state.camera.set_zoom(state.camera_zoom);

    if app.keyboard.is_down(KeyCode::Q) {
        state.camera_zoom -= state.camera_zoom * app.timer.delta_f32();
    }
    if app.keyboard.is_down(KeyCode::E) {
        state.camera_zoom += state.camera_zoom * app.timer.delta_f32();
    }
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.paused = !state.paused;
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

    gfx.render(&draw);
    

    if state.chosen_pattern != state.pattern {
        load_selected_pattern(&mut state.planets, app, state.pattern);
        state.trail_texture = gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).build().unwrap();
        state.chosen_pattern = state.pattern;
        state.camera.set_position(app.window().width() as f32 / 2., app.window().height() as f32 / 2.);
        state.camera.set_zoom(1.0);
        state.camera_zoom = 1.0;
    }

    let output = plugins.egui(|ctx| {
        SidePanel::new(Side::Left, "left_panel").resizable(false).min_width(app.window().width() as f32 / 5.).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(state.translations.get(&state.chosen_lang, "settings"));
            });
            ui.separator();

            ui.label(state.translations.get(&state.chosen_lang, "chooselang"));
            ComboBox::from_label("  ").selected_text(&state.chosen_lang).show_ui(ui, |ui| {
               ui.selectable_value(&mut state.chosen_lang, "en".to_string(), "English");
               ui.selectable_value(&mut state.chosen_lang, "lt".to_string(), "Lietuvių");
            });
            ui.add_space(20.);

            ui.label(state.translations.get(&state.chosen_lang, "choose"));
            ComboBox::from_label(" ").selected_text(format!("{}{}", state.translations.get(&state.chosen_lang, "pattern"), state.chosen_pattern)).show_ui(ui, |ui| {
                for i in 0..=9 {
                    ui.selectable_value(&mut state.pattern, i, format!("{}{}", state.translations.get(&state.chosen_lang, "pattern"), i));
                }
            });
            ui.add_space(20.);

            let slider = Slider::new(&mut state.sim_speed, 1..=25).text(state.translations.get(&state.chosen_lang, "simspeed"));
            ui.add(slider);
            
            if ui.checkbox(&mut state.show_trail, state.translations.get(&state.chosen_lang, "showtrail")).clicked() {
                state.trail_texture = gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).build().unwrap();
            }
            ui.checkbox(&mut state.show_bodies, state.translations.get(&state.chosen_lang, "showbodies"));

            ui.checkbox(&mut state.paused, state.translations.get(&state.chosen_lang, "pause"));

            if ui.button(state.translations.get(&state.chosen_lang, "restart")).clicked() {
                load_selected_pattern(&mut state.planets, app, state.pattern);
                state.trail_texture = gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).build().unwrap();
            }

            ui.add_space(20.);
            ui.label(state.translations.get(&state.chosen_lang, "wasd"));
            ui.label(state.translations.get(&state.chosen_lang, "qe"));

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add_space(20.);
                ui.hyperlink("https://github.com/mantasarm/n-body-sim");
                ui.label(state.translations.get(&state.chosen_lang, "credits"));
            });
        });
    });
    
    gfx.render(&output);
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(WindowConfig::new().vsync(true).title("Gravitacija").multisampling(8).resizable(true).maximized(true))
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}