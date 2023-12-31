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
