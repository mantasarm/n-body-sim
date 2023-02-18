pub mod mover;
pub mod vec_math;
pub mod pattern_loader;
pub mod camera;

use camera::Camera2D;
use mover::Mover;
use notan::{notan_main, AppState, prelude::{Graphics, App, WindowConfig, Color, RenderTexture, TextureFilter, Plugins, KeyCode}, draw::{DrawConfig, CreateDraw, DrawImages}, egui::{EguiConfig, Window, EguiPluginSugar, SidePanel, panel::Side, Slider, ComboBox}};
use pattern_loader::load_selected_pattern;

pub const TRAIL_TEX_WIDTH: i32 = 1920 * 6;
pub const TRAIL_TEX_HEIGHT: i32 = 1080 * 6;

#[derive(AppState)]
struct State {
    planets: Vec<Mover>,
    trail_texture: RenderTexture,
    sim_speed: i32,
    show_trail: bool,
    show_bodies: bool,
    G: f32,
    pattern: i32,
    chosen_pattern: i32,
    camera: Camera2D,
    camera_zoom: f32,
    paused: bool
}

impl State {
    fn new(app: &mut App, gfx: &mut Graphics) -> Self {
        let mut planets = Vec::<Mover>::new();

        pattern_loader::load_selected_pattern(&mut planets, app, 0);
        
        Self {
            //sun,
            planets,
            trail_texture: gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap(),
            sim_speed: 1,
            show_trail: true,
            show_bodies: true,
            G: 10.,
            pattern: 2,
            chosen_pattern: 0,
            camera: Camera2D::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., app.window().width() as f32, app.window().height() as f32),
            camera_zoom: 1.,
            paused: false
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
                        temp_mover.attract(state.planets.get_mut(j).unwrap(), state.G);
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
                ui.heading("Parametrai");
            });
            ui.separator();

            ui.label("Pasirinkite raštą: ");
            ComboBox::from_label("").selected_text(format!("raštas{}", state.chosen_pattern)).show_ui(ui, |ui| {
                ui.selectable_value(&mut state.pattern, 0, "raštas0");
                ui.selectable_value(&mut state.pattern, 1, "raštas1");
                ui.selectable_value(&mut state.pattern, 2, "raštas2");
                ui.selectable_value(&mut state.pattern, 3, "raštas3");
                ui.selectable_value(&mut state.pattern, 4, "raštas4");
                ui.selectable_value(&mut state.pattern, 5, "raštas5");
                ui.selectable_value(&mut state.pattern, 6, "raštas6");
                ui.selectable_value(&mut state.pattern, 7, "raštas7");
                ui.selectable_value(&mut state.pattern, 8, "raštas8");
                ui.selectable_value(&mut state.pattern, 9, "raštas9");
            });
            ui.add_space(20.);

            let slider = Slider::new(&mut state.sim_speed, 1..=25).text("Simuliacijos greitis");
            ui.add(slider);
            
            if ui.checkbox(&mut state.show_trail, "Rodyti trajektoriją").clicked() {
                state.trail_texture = gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).build().unwrap();
            }
            ui.checkbox(&mut state.show_bodies, "Rodyti kūnus");

            ui.checkbox(&mut state.paused, "Sustabdyti");

            if ui.button("Paleisti iš naujo").clicked() {
                load_selected_pattern(&mut state.planets, app, state.pattern);
                state.trail_texture = gfx.create_render_texture(TRAIL_TEX_WIDTH, TRAIL_TEX_HEIGHT).build().unwrap();
            }

            ui.add_space(20.);
            ui.label("Naudokite W, A, S ir D klavišus judėjimui");
            ui.label("Naudokite Q ir E klavišus atitraukti ir pritraukti vaizdą");

            // for mover in &state.planets {
            //     ui.label(format!("pos: {}, x_force: {}, y_force: {}", mover.pos, mover.vel.x, mover.vel.y));
            // }
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