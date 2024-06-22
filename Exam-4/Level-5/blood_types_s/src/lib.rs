#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}
impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        self.donors().contains(other)
	}
	pub fn recipients(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        let mut donate : Vec<Self>=vec![];
        match self{
            BloodType {antigen: O, rh_factor: Positive} => donate=vec![
                BloodType {antigen: O,rh_factor: Positive},
                BloodType {antigen: A, rh_factor: Positive},
                BloodType {antigen: B, rh_factor: Positive},
                BloodType {antigen: AB, rh_factor: Positive},
            ],
            BloodType {antigen: O, rh_factor: Negative} => donate=vec![
                BloodType {antigen: O,rh_factor: Positive},
                BloodType {antigen: O,rh_factor: Negative},
                BloodType {antigen: A, rh_factor: Positive},
                BloodType {antigen: A, rh_factor: Negative},
                BloodType {antigen: AB, rh_factor: Positive},
                BloodType {antigen: AB, rh_factor: Negative},
                BloodType {antigen: B, rh_factor: Positive},
                BloodType {antigen: B, rh_factor: Negative},
            ],
            BloodType {antigen: A, rh_factor: Positive} => donate=vec![
                BloodType {antigen: A, rh_factor: Positive},
                BloodType {antigen: AB, rh_factor: Positive},
            ],
            BloodType {antigen: A, rh_factor: Negative} => donate = vec![
                BloodType {antigen : A , rh_factor: Negative},
                BloodType {antigen : AB , rh_factor: Negative},
                BloodType {antigen: A, rh_factor: Positive},
                BloodType {antigen: AB, rh_factor: Positive},
            ],
            BloodType {antigen: B, rh_factor: Positive} => donate = vec![
                BloodType {antigen : B , rh_factor: Positive},
                BloodType {antigen : AB , rh_factor: Positive},
            ],
            BloodType {antigen: B, rh_factor: Negative} => donate = vec![
                BloodType {antigen : B , rh_factor: Negative},
                BloodType {antigen : B , rh_factor: Positive},
                BloodType {antigen : AB , rh_factor: Positive},
                BloodType {antigen : AB , rh_factor: Negative},
            ],
            BloodType {antigen: AB, rh_factor: Positive} => donate=vec![
                BloodType {antigen: AB, rh_factor: Positive},
            ],
            BloodType {antigen: AB, rh_factor: Negative} =>donate=vec![
                BloodType {antigen: AB, rh_factor: Positive},
                BloodType {antigen: AB, rh_factor: Negative},
            ],
        }
        donate
	}
    
	pub fn donors(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        let mut receive : Vec<Self>=vec![];
        match self{
            BloodType {antigen: O, rh_factor: Positive} => receive=vec![
                BloodType {antigen: O,rh_factor: Positive},
                BloodType {antigen: O,rh_factor: Negative},
            ],
            BloodType {antigen: O, rh_factor: Negative} => receive=vec![
                BloodType {antigen: O,rh_factor: Negative},
            ],
            BloodType {antigen: A, rh_factor: Positive} => receive=vec![
                BloodType {antigen: A, rh_factor: Positive},
                BloodType {antigen: A, rh_factor: Negative},
                BloodType {antigen: O, rh_factor: Negative},
                BloodType {antigen: O, rh_factor: Positive}
            ],
            BloodType {antigen: A, rh_factor: Negative} => receive = vec![
                BloodType {antigen : A , rh_factor: Negative},
                BloodType {antigen: O, rh_factor: Negative},
            ],
            BloodType {antigen: B, rh_factor: Positive} => receive = vec![
                BloodType {antigen : B , rh_factor: Positive},
                BloodType {antigen : B , rh_factor: Negative},
                BloodType {antigen: O, rh_factor: Positive},
                BloodType {antigen: O, rh_factor: Negative},
            ],
            BloodType {antigen: B, rh_factor: Negative} => receive = vec![
                BloodType {antigen : B , rh_factor: Negative},
                BloodType {antigen: O, rh_factor: Negative},
            ],
            BloodType {antigen: AB, rh_factor: Positive} => receive=vec![
                BloodType {antigen: O,rh_factor: Positive},
                BloodType {antigen: O,rh_factor: Negative},
                BloodType {antigen: A, rh_factor: Positive},
                BloodType {antigen: A, rh_factor: Negative},
                BloodType {antigen: AB, rh_factor: Positive},
                BloodType {antigen: AB, rh_factor: Negative},
                BloodType {antigen: B, rh_factor: Positive},
                BloodType {antigen: B, rh_factor: Negative},
            ],
            BloodType {antigen: AB, rh_factor: Negative} =>receive=vec![
                BloodType {antigen: AB, rh_factor: Negative},
                BloodType {antigen: A, rh_factor: Negative},
                BloodType {antigen: B, rh_factor: Negative},
                BloodType {antigen: O, rh_factor: Negative},
            ],
        }
        receive
	}
}