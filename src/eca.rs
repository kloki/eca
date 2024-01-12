use rand::Rng;
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Init {
    Random,
    Single,
}

pub struct RuleSet {
    rules: [bool; 8],
}

impl RuleSet {
    pub fn new(num: u8) -> Self {
        let mut rules = [false; 8];
        for i in (0..8).rev() {
            if (num >> i) & 1 == 1 {
                rules[i] = true
            }
        }

        RuleSet { rules }
    }

    pub fn apply_rules(&self, l: bool, m: bool, r: bool) -> bool {
        self.rules[(l as usize) * 4 + (m as usize) * 2 + (r as usize)]
    }
}

pub struct Eca {
    pub iterations: Vec<Vec<bool>>,
    rs: RuleSet,
}

impl Eca {
    pub fn new(width: usize, initialization: Init, rs: RuleSet) -> Self {
        let mut init = vec![false; width];
        match initialization {
            Init::Single => init[(width / 2) as usize] = true,
            Init::Random => {
                let mut rng = rand::thread_rng();
                for x in 0..(width - 1) {
                    init[x] = rng.gen();
                }
            }
        }

        Eca {
            iterations: vec![init],
            rs,
        }
    }

    pub fn iterate(&mut self, iterations: usize) {
        let width = self.iterations[0].len() - 1;
        for _ in 0..iterations {
            let last = self.iterations.len() - 1;
            let new = (0..self.iterations[last].len())
                .map(|x| {
                    let l = {
                        //wrap around if at the edges
                        if x == 0 {
                            self.iterations[last][width]
                        } else {
                            self.iterations[last][x - 1]
                        }
                    };
                    let m = self.iterations[last][x];

                    let r = {
                        if x == width {
                            self.iterations[last][0]
                        } else {
                            self.iterations[last][x + 1]
                        }
                    };
                    self.rs.apply_rules(l, m, r)
                })
                .collect::<Vec<bool>>();
            self.iterations.push(new);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        let rs = RuleSet::new(110);
        assert_eq!(rs.apply_rules(true, true, true), false);
        assert_eq!(rs.apply_rules(true, true, false), true);
        assert_eq!(rs.apply_rules(true, false, true), true);
        assert_eq!(rs.apply_rules(true, false, false), false);
        assert_eq!(rs.apply_rules(false, true, true), true);
        assert_eq!(rs.apply_rules(false, true, false), true);
        assert_eq!(rs.apply_rules(false, false, true), true);
        assert_eq!(rs.apply_rules(false, false, false), false);
    }
    #[test]
    fn test_30() {
        let rs = RuleSet::new(30);
        assert_eq!(rs.apply_rules(true, true, true), false);
        assert_eq!(rs.apply_rules(true, true, false), false);
        assert_eq!(rs.apply_rules(true, false, true), false);
        assert_eq!(rs.apply_rules(true, false, false), true);
        assert_eq!(rs.apply_rules(false, true, true), true);
        assert_eq!(rs.apply_rules(false, true, false), true);
        assert_eq!(rs.apply_rules(false, false, true), true);
        assert_eq!(rs.apply_rules(false, false, false), false);
    }
}
