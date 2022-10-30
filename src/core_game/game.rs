extern crate piston_window;

use std::time;

use piston_window::*;
use rand;

pub struct Game {
    matrix : Vec<Vec<u8>>,
    height : usize,
    width : usize,
    height_window : usize,
    width_window : usize,
    size_cell_w : f64,
    size_cell_h : f64,
    last_update : u128,
}



impl Game {

    pub fn new( width : usize,height : usize,width_w : usize , height_w : usize) -> Self {
        Self {
            matrix : vec![vec![0; width]; height],
            height,
            width,
            height_window : height_w,
            width_window : width_w,
            size_cell_w : (width_w / width )as f64,
            size_cell_h : (height_w / height) as f64,
            last_update : 1,
        }
    }




    pub fn generate(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.matrix[i][j] = rand::random::<u8>() % 2;
            }
        }
    }
    
    pub fn update(&mut self){
        let mut new_matrix = vec![vec![0; self.width]; self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                let mut count = 0;
                for k in 0..3 {
                    for l in 0..3 {
                        if k == 1 && l == 1 {
                            continue;
                        }
                        let x = (i + k + self.height - 1) % self.height;
                        let y = (j + l + self.width - 1) % self.width;
                        count += self.matrix[x][y] as usize;
                    }
                }
                if count == 3 {
                    new_matrix[i][j] = 1;
                } else if count == 2 {
                    new_matrix[i][j] = self.matrix[i][j];
                } else {
                    new_matrix[i][j] = 0;
                }
            }
        }
        self.matrix = new_matrix;
    }

    pub fn print_value(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.matrix[i][j]);
            }
            println!("");
        }
    }



    
    pub fn print(&self) {
        for row in &self.matrix {
            for cell in row {
                if *cell == 0 {
                    print!("O");
                } else {
                    print!("X");
                }                    
            }
            println!();
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn print_window(&mut self){
        let mut window: PistonWindow =
        WindowSettings::new("Game of life", [self.width_window as u32, self.height_window as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();
        
        while let Some(event) = window.next() {
            // press r to generate a new matrix
            if let Some(Button::Keyboard(Key::R)) = event.press_args() {
                self.generate();
            }
            // press space to pause the game and press again to continue
            if let Some(Button::Keyboard(Key::Space)) = event.press_args() {
                if self.last_update ==0 {
                    self.last_update = 1;
                    window.set_title("Game of life".to_string());
                } else {
                    self.last_update = 0;
                    //set title to pause
                    window.set_title("Game of life - PAUSE".to_string());
                }
            }
            // press c to clear the matrix
            if let Some(Button::Keyboard(Key::C)) = event.press_args() {
                self.matrix = vec![vec![0; self.width]; self.height];
            }
            
            if self.last_update < time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis() && self.last_update != 0 {
                self.update();
                self.last_update = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis() + 100;
                
            }
            window.draw_2d(&event, |context, graphics, _device| {
                clear([0.0,0.0,0.0,1.0], graphics);
                for i in 0..self.height {
                    for j in 0..self.width {
                        if self.matrix[i][j] == 1 {
                            let x = j as f64 * self.size_cell_w;
                            let y = i as f64 * self.size_cell_h;
                            rectangle([1.0, 0.0, 0.0, 1.0], // red
                                [x , y , self.size_cell_w, self.size_cell_h ],
                                context.transform,
                                graphics);
                        }else{
                            let x = j as f64 * self.size_cell_w;
                            let y = i as f64 * self.size_cell_h;
                            rectangle([1.0;4], // black
                                [x , y , self.size_cell_w, self.size_cell_h ],
                                context.transform,
                                graphics);
                        }
                    }
                }
            });
        }
    }
}