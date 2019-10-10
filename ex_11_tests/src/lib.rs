
pub struct Rectange {
    lenght: u32,
    width: u32,
}

impl Rectange {
    pub fn can_hold(&self, other: Rectange) -> bool {
        self.lenght > other.lenght && self.width > other.width
    }

    pub fn is_equal(&self, other: Rectange) -> bool {
        self.lenght == other.lenght && self.width == other.width
    }
}

#[cfg(test)]
mod tests {

    use super::Rectange;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_rectangles() {
        let rec1 = Rectange{lenght: 7, width: 8};
        let rec2 = Rectange{lenght: 4, width: 3};

        assert!(rec1.can_hold(rec2))
    }

    #[test]
    fn test_rec_equality() {
        let rec1 = Rectange{lenght: 4, width: 3};
        let rec2 = Rectange{lenght: 4, width: 3};
        let rec3 = Rectange{lenght: 8, width: 5};

        assert_eq!(rec1.is_equal(rec2), true);
        assert_ne!(rec1.is_equal(rec3), true);
    }

    #[test]
    fn failde_test() {
        let var1 = 1;
        let var2 = 2;
        assert_eq!(var1, var2, "This cool test failed");
    }
}
