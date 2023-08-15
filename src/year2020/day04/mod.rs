use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REG_HGT: Regex = Regex::new(r"^(?<value>\d+)(?<unit>cm|in)$").unwrap();
    static ref REG_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref REG_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").split("\n\n");

    let passports: Vec<Passport> = input.map(|passport| passport.parse::<Passport>().unwrap()).collect();

    let result1 = passports.iter().filter(|passport| passport.has_all_fields()).count();
    let result2 = passports.iter().filter(|passport| passport.is_valid()).count();

    println!("04\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[derive(Debug, Default)]
struct Passport
{
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Debug, Default)]
struct Height
{
    value: usize,
    unit: String,
}

impl Passport
{
    fn has_all_fields(&self) -> bool
    {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool
    {
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        self.has_all_fields()
            && self.byr.unwrap() >= 1920
            && self.byr.unwrap() <= 2002
            && self.iyr.unwrap() >= 2010
            && self.iyr.unwrap() <= 2020
            && self.eyr.unwrap() >= 2020
            && self.eyr.unwrap() <= 2030
            && colors.contains(&self.ecl.as_ref().unwrap().as_str())
            && REG_HCL.is_match(self.hcl.as_ref().unwrap())
            && REG_PID.is_match(self.pid.as_ref().unwrap())
            && match self.hgt.as_ref().unwrap().unit.as_str() {
                "cm" => self.hgt.as_ref().unwrap().value >= 150 && self.hgt.as_ref().unwrap().value <= 193,
                "in" => self.hgt.as_ref().unwrap().value >= 59 && self.hgt.as_ref().unwrap().value <= 76,
                _ => false,
            }
    }
}

impl std::str::FromStr for Passport
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        for field in s.split_whitespace() {
            let mut field = field.split(':');
            let key = field.next().unwrap();
            let value = field.next().unwrap();

            match key {
                "byr" => passport.byr = Some(value.parse().unwrap()),
                "iyr" => passport.iyr = Some(value.parse().unwrap()),
                "eyr" => passport.eyr = Some(value.parse().unwrap()),
                "hgt" => {
                    if let Some(caps) = REG_HGT.captures(value) {
                        passport.hgt = Some(Height {
                            value: caps["value"].parse().unwrap(),
                            unit: caps["unit"].to_string(),
                        });
                    }
                    else {
                        passport.hgt = Some(Height::default());
                    }
                }
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value.to_string()),
                _ => panic!("Unknown field"),
            }
        }

        Ok(passport)
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (260, 153));
    }
}
