use regex::Regex;
use serde_json::Value;

const LOCATIONS: &str = include_str!("locations.json");
const YEAR_CODE: &str = include_str!("year.json");

// parse the 8-digit code of G-SHOCK watches(printed on steel back)
pub fn parse(code: &str) -> Value{
    // use regex to check g-shock 8-digit code
    let g_shock_code_regex: Regex = Regex::new(r"^[0-9]{3}[A-Z]{1}[0-9]{3}[A-Z]{1}$").unwrap();
    if !g_shock_code_regex.is_match(code) {
        serde_json::from_str(
            r#"
             {
                "msg": "Your input is invalid! 你的输入不合法！"
             }
            "#
        ).unwrap()
    } else {
        // parse first 4 digits - factory location of the product
        let product_location = &code[0..=3];
        let location_json: Value = serde_json::from_str(LOCATIONS).unwrap();
        let result: Option<&str> = location_json[product_location].as_str();

        // parse last 4 digits - production date
        let product_date: &str = &code[4..];
       
        // ABCDEFGHJ: a year of a decade(1/2/3/4/5/6/7/8/9/0)
        // example: 062C: 2013/2023. the exact year must be after your G-SHOCK release date.
        let year_code: &str = &product_date[3..];

        let production_date = calculate_date(
            &product_date[0..=2],
            year_code
        );
      
        let return_value: Value = match result {
            Some(value) => {
                let location = String::from(value);
                let json = format!(r#"
                {{
                    "location": "{}",
                    "productionDate": "{}"
                }}
                "#, location, &production_date[0..production_date.len() - 2]);
                serde_json::from_str(&json[..]).unwrap()
            },
            None => serde_json::from_str(
                r#"
                 {
                    "msg": "Your code is incorrect! 编码不正确，未找到结果！"
                 }
                "#
            ).unwrap()
        };

        return_value
    }
}

// calculate date by the day of year
fn calculate_date(mut day_of_year: &str, year_code: &str) -> String{
    let mut month = 0;
    let mut day = 0;

    let mut month_leap = 0;
    let mut day_leap = 0;

    if day_of_year.chars().nth(0) == Some('0'){
        day_of_year = &day_of_year[1..];
        if day_of_year.chars().nth(1) == Some('0'){
            day_of_year = &day_of_year[2..];
        }
    }

    let mut day_of_year: i32 = day_of_year.parse().unwrap();
    let mut day_of_year_leap: i32 = day_of_year.clone();

    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; 
    let days_in_month_leap = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; 

    let year_code_json: Value = serde_json::from_str(YEAR_CODE).unwrap();
    let year_last_digit: i64 = year_code_json[year_code].as_i64().unwrap();
    let potential_years: Vec<i64> = vec![2010 + year_last_digit, 2020 + year_last_digit];


    for (index, &days) in days_in_month.iter().enumerate() {
        if day_of_year <= days {
            month = index + 1;
            day = day_of_year;
            break;
        }
        day_of_year -= days;
    }

    for (index, &days) in days_in_month_leap.iter().enumerate() {
        if day_of_year_leap <= days {
            month_leap = index + 1;
            day_leap = day_of_year_leap;
            break;
        }
        day_of_year_leap -= days;
    }

    let mut result: String = "".to_owned();

    for year in potential_years {
        if is_leap_year(year) {
            result += format!("{}.{}.{}, ", year, month_leap, day_leap).as_str();
        }
        else {
            result += format!("{}.{}.{}, ", year, month, day).as_str();
        }
    }

    result
}

// leap year?
fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}