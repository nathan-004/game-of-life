use std::io::{self, Write};
use std::fs::{self, File};

use macroquad::{file, prelude::*};

const WIDTH: usize = 150;
const HEIGHT: usize = 75;
const CELL_SIZE: f32 = 10.0;
const GRID_OFFSET_Y: f32 = 100.0;

struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: vec![vec![false; WIDTH]; HEIGHT],
        }
    }

    // Initialiser avec un pattern aléatoire
    fn randomize(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut rng = seed;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                rng = (rng * 1103515245 + 12345) & 0x7fffffff;
                self.cells[y][x] = (rng % 100) < 30; // 30% de chance d'être vivant
            }
        }
    }

    fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.cells[y][x] = false;
            }
        }
    }

    // Compter les voisins vivants
    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = (x as isize + dx + WIDTH as isize) % WIDTH as isize;
                let ny = (y as isize + dy + HEIGHT as isize) % HEIGHT as isize;
                
                if self.cells[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
        
        count
    }

    // Calculer la prochaine génération
    fn next_generation(&mut self) {
        let mut new_cells = vec![vec![false; WIDTH]; HEIGHT];
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.cells[y][x];
                
                // Règles du jeu de la vie
                new_cells[y][x] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,  // Survie
                    (false, 3) => true,              // Naissance
                    _ => false,                      // Mort
                };
            }
        }
        
        self.cells = new_cells;
    }

    // Afficher la grille
    fn display_console(&self) {
        // Effacer l'écran (ANSI escape codes)
        print!("\x1B[2J\x1B[1;1H");
        
        println!("╔{}╗", "═".repeat(WIDTH));
        
        for row in &self.cells {
            print!("║");
            for &cell in row {
                print!("{}", if cell { "█" } else { " " });
            }
            println!("║");
        }
        
        println!("╚{}╝", "═".repeat(WIDTH));
        println!("Jeu de la vie de Conway - Appuyez sur Ctrl+C pour quitter");
        
        io::stdout().flush().unwrap();
    }

    fn draw(&self) {
        draw_rectangle_lines(0.0, GRID_OFFSET_Y, CELL_SIZE * WIDTH as f32, CELL_SIZE * HEIGHT as f32, 5.0, WHITE);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE + GRID_OFFSET_Y,
                        CELL_SIZE - 1.0,
                        CELL_SIZE - 1.0,
                        WHITE,
                    );
                }
            }
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        writeln!(file, "{}", WIDTH)?; // stock width at start
        let str_grid: String = self.cells.iter()
            .flat_map(|row| row.iter().map(|&cell| if cell { '1' } else { '0' }))
            .collect();
        
        writeln!(file, "{}", str_grid)?;

        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let content = fs::read_to_string(filename)?;
        let mut lines = content.lines();

        let width: usize = lines.next().unwrap_or("").parse().unwrap_or(WIDTH);

        if let Some(grid_str) = lines.next() {
            for (i, c) in grid_str.chars().enumerate() {
                let y = i / width;
                let x = i % width;
                if y < HEIGHT && x < WIDTH {
                    self.cells[y][x] = c == '1';
                }
            }
        }

        Ok(())
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Jeu de la vie de Conway".to_owned(),
        //window_width: (WIDTH as f32 * CELL_SIZE) as i32,
        //window_height: (HEIGHT as f32 * CELL_SIZE + GRID_OFFSET_Y) as i32,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new();
    let mut paused = true;
    let mut frame_count = 0;
    let mut speed = 6; // Frames entre chaque génération
    
    loop {
        clear_background(BLACK);

        // Gestion des entrées
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::R) {
            grid.randomize();
            paused = true;
        }

        if is_key_pressed(KeyCode::N) {
            grid.next_generation();
        }

        if is_key_pressed(KeyCode::C) {
            grid.clear();
        }

        if is_key_pressed(KeyCode::S) {
            let _ = grid.save_to_file("save");
        }

        if is_key_pressed(KeyCode::L) {
            let _ = grid.load_from_file("save");
        }

        if is_key_pressed(KeyCode::KpAdd) {
            // Augmenter la vitesse (diminuer le délai)
            if speed > 1 {
                speed -= 1;
            }
        }

        if is_key_pressed(KeyCode::KpSubtract) {
            // Diminuer la vitesse (augmenter le délai)
            speed += 1;
        }

        if is_key_pressed(KeyCode::Up) {
            // Augmenter la vitesse (diminuer le délai)
            if speed > 1 {
                speed -= 1;
            }
        }

        if is_key_pressed(KeyCode::Down) {
            // Diminuer la vitesse (augmenter le délai)
            speed += 1;
        }

        // Dessiner avec la souris
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let grid_x = (mx / CELL_SIZE) as usize;
            let grid_y = ((my - GRID_OFFSET_Y) / CELL_SIZE) as usize;
            if grid_x < WIDTH && grid_y < HEIGHT {
                grid.cells[grid_y][grid_x] = true;
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let grid_x = (mx / CELL_SIZE) as usize;
            let grid_y = ((my - GRID_OFFSET_Y) / CELL_SIZE) as usize;
            if grid_x < WIDTH && grid_y < HEIGHT {
                grid.cells[grid_y][grid_x] = false;
            }
        }

        // Mise à jour de la simulation
        if !paused {
            frame_count += 1;
            if frame_count >= speed {
                grid.next_generation();
                frame_count = 0;
            }
        }

        // Affichage
        grid.draw();

        // Interface
        let status = if paused { "PAUSE" } else { "EN COURS" };
        draw_text(
            &format!("Status: {} | FPS: {}", status, get_fps()),
            10.0,
            20.0,
            20.0,
            GREEN,
        );
        draw_text("ESPACE: Play/Pause | Esc: Quitter", 10.0, 40.0, 16.0, LIGHTGRAY);
        draw_text("R: Aléatoire | C: Effacer | S: Sauvegarder | L: Enregistrer", 10.0, 56.0, 16.0, LIGHTGRAY);
        draw_text("N: Étape suivante | Souris: Dessiner | +/-: Accélérer/Ralentir", 10.0, 72.0, 16.0, LIGHTGRAY);

        next_frame().await
    }
}