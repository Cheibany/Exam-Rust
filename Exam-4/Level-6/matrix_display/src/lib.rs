#[derive(Debug,Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);
impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let t:Vec<Vec<i32>>=slice.iter().map(|&vec| vec.to_vec()).collect();
        Matrix(t)
    }
}
use std::fmt;
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len(){
        write!(f,"(")?;
        for j in 0..self.0[i].len(){
            if j != self.0[i].len()-1{
                write!(f,"{} ",self.0[i][j])?;
            }else{
                write!(f,"{}",self.0[i][j])?;
            }
        }
        if i != self.0.len()-1{
            write!(f,")\n")?;
        }else{
            write!(f,")")?;
        }
        }
        Ok(())
    }
}