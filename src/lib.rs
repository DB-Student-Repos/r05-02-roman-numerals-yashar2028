const SYMBOLES: [(usize, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman;

impl Roman {
    pub fn from(mut num: usize) -> String {
        let mut result = String::new();
        for (values, symboles) in SYMBOLES {
            while num >= values {
                result.push_str(symboles);
                num -= values;
            }
        }
        result
    } 
}
