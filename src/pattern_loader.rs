use crate::{mover::Mover, camera::Camera2D};

pub struct Pattern {
    pub bodies: Vec<Mover>,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            bodies: Vec::<Mover>::new()
        }
    }

    pub fn add_body(mut self, mover: Mover) -> Self {
        self.bodies.push(mover);

        self
    } 
}

pub struct PatternLoader {
    pub patterns: Vec<Pattern>,
    pub pattern: usize,
    pub chosen_pattern: usize,
}

impl PatternLoader {
    pub fn new() -> Self {
        let mut patterns = Vec::<Pattern>::new();

        // Pattern 0
        patterns.push(Pattern::new());

        // Pattern 1
        patterns.push(Pattern::new()
                        .add_body(Mover::new(0.0, 0.0, 5., 0., 0.).apply_forces(false))
                        .add_body(Mover::new(0.0 + 100., 0.0 - 100., 3., 2., 2.)));

        // Pattern 2
        patterns.push(Pattern::new()
                        .add_body(Mover::new(0.0 + 0., 0.0 - 150., 3., 1.35, 0.0))
                        .add_body(Mover::new(0.0 + 0., 0.0 + 150., 3., -1.35, 0.0)));

        // Pattern 3
        patterns.push(Pattern::new()
                        .add_body(Mover::new(0.0 + 0., 0.0 - 150., 3., 0.5, 0.0))
                        .add_body(Mover::new(0.0 + 0., 0.0 + 150., 3., -0.5, 0.0)));

        // Pattern 4
        patterns.push(Pattern::new()
                        .add_body(Mover::new(0.0 + 200., 0.0 - 0., 3., 0., 2.))
                        .add_body(Mover::new(0.0 - 200., 0.0 - 0., 3., 0., -2.))
                        .add_body(Mover::new(0.0 - 0., 0.0 - 200., 3., 2., 0.))
                        .add_body(Mover::new(0.0 - 0., 0.0 + 200., 3., -2., 0.)));

        // Pattern 5
        patterns.push(Pattern::new()
                        .add_body(Mover::new(0.0, 0.0, 5., 0., 0.).apply_forces(false))
                        .add_body(Mover::new(0.0 + 400., 0.0 + 0., 1., 0., 0.4)));

        // Pattern 6
        patterns.push(Pattern::new()
                        .add_body(Mover::new(0.0 + 0., 0.0 - 150., 3., 2., 0.0))
                        .add_body(Mover::new(0.0 + 0., 0.0 + 150., 3., -2., 0.0)));


        Self {
            patterns,
            pattern: 1,
            chosen_pattern: 1,
        }
    }

    pub fn handle_pattern_changes(&mut self, camera: &mut Camera2D, camera_zoom: &mut f32, editor_enabled: &mut bool) -> bool {
        if self.chosen_pattern != self.pattern {
            self.chosen_pattern = self.pattern;
            camera.set_position(0., 0.);
            camera.set_zoom(1.0);
            *camera_zoom = 1.0;
            
            if self.chosen_pattern != 0 {
                *editor_enabled = false;
            } else {
                *editor_enabled = true;
            }
            return true;
        }
        false
    }

    pub fn load_pattern(&self, planets: &mut Vec<Mover>, index: usize) {
        planets.clear();

        for mover in &self.patterns.get(index).unwrap().bodies {
            planets.push(mover.to_owned());
        }

    }

    pub fn reload_pattern(&self, planets: &mut Vec<Mover>) {
        planets.clear();

        for mover in &self.patterns.get(self.chosen_pattern).unwrap().bodies {
            planets.push(mover.to_owned());
        }

    }
}