use rand::Rng;
#[derive(Debug,Clone)]
#[allow(dead_code)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub elements: Vec<f64>,
}


/* Matrix Operations

    Addition: A + B = C

    Multiplication: A * B = C

    Transpose: A^T = A

    Inverse: A^-1 = A

    Determinant: det(A)

    Dot Product: A . B

    Pretty Print Matrix: print(A)

    Pretty Print Vector: print(A)

*/

#[allow(dead_code)]
impl Matrix {


    //user defined constructor
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {

        assert!(data.len()-1 != rows * cols, "Invalid Size");
        return Matrix { rows: rows, columns: cols,elements: data };  
        
    }

    // initialize a matrix with zeros
    pub fn zeros(rows:usize, cols:usize) -> Matrix {

        Matrix { rows: rows, columns: cols, elements: vec![0.0; cols * rows] }
    }


    // initialize a random matrix with values with values between 0 and 1
    pub fn init_random_matrix(rows: usize, cols: usize ) -> Matrix {
        //define the size of the matrix
        let size: usize  = rows * cols;

        let mut rng = rand::thread_rng();
       

        let random_vec: Vec<f64> = (0..size)
            .map(|_| rng.gen_range(0.0..1.0))  // Generate random f64 between 0.0 and 1.0
            .collect();


        return Matrix { rows: rows, columns: cols, elements: random_vec};


    }

    // pretty print the matrix
    pub fn pretty_print_matrix (&self) {
        for i in 0..self.rows {
            for j in 0..self.columns {
                print!("{} ", self.elements[i * self.columns + j]);
            }
            println!();
        }
    }   


    //add two matrices
    pub fn add(&self , other : &Matrix ) -> Matrix {

        //ensure that the dimensions of the two matrices are the same
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.columns, other.columns);


        //perform matrix addition using rust iterators
        let new_elements: Vec<f64> = self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(x, y)| x + y)
            .collect();

        return Matrix { rows: self.rows, columns: self.columns, elements: new_elements };

    }

    //transpose a matrix
    pub fn transpose(&self) -> Matrix {
        let mut buffer = vec![0.0; self.columns * self.rows];

        for i in 0..self.rows {
            for j in 0..self.columns {
                buffer[j * self.rows + i] = self.elements[i * self.columns + j];
            }
        }

        return Matrix {
            rows: self.columns,
            columns: self.rows,
            elements: buffer,
        }
    }

}


