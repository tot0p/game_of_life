use rand;

pub struct Game {
    matrix : Vec<Vec<u8>>,
    height : usize,
    width : usize,
}

impl Game {

    pub fn new(height : usize, width : usize) -> Self {
        Self {
            matrix : vec![vec![0; width]; height],
            height,
            width,
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
}