pub fn parse_four_delimited_strings(
    input: String,
) -> Result<(String, String, String, u64), teloxide::utils::command::ParseError> {
    let parts: Vec<String> = input.split('|').map(|s| s.trim().to_string()).collect();

    if parts.len() == 4 {
        let res = parts[3].parse::<u64>();
        if res.is_ok() {
            Ok((
                parts[0].clone(),
                parts[1].clone(),
                parts[2].clone(),
                parts[3].clone().parse().expect("Not a valid u64"),
            ))
        } else {
            Err(teloxide::utils::command::ParseError::IncorrectFormat(
                Box::new(res.err().unwrap()),
            ))
        }
    } else {
        Err(teloxide::utils::command::ParseError::TooFewArguments {
            expected: 4,
            found: parts.len(),
            message: "Please, provide correct number of parameters".to_string(),
        })
    }
}

pub fn parse_two_delimited_strings(
    input: String,
) -> Result<(String, String), teloxide::utils::command::ParseError> {
    let parts: Vec<String> = input.split('|').map(|s| s.trim().to_string()).collect();

    if parts.len() == 2 {
        Ok((parts[0].clone(), parts[1].clone()))
    } else {
        Err(teloxide::utils::command::ParseError::TooFewArguments {
            expected: 4,
            found: parts.len(),
            message: "Please, provide correct number of parameters".to_string(),
        })
    }
}
