use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 50;
const HEIGHT: usize = 25;

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
    fn display(&self) {
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
}

fn main() {
    let mut grid = Grid::new();
    grid.randomize();
    
    // Boucle principale
    loop {
        grid.display();
        grid.next_generation();
        thread::sleep(Duration::from_millis(100));
    }
}