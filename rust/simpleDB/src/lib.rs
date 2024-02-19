#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::TrailingData);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let result = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    // Tests for empty messages and unknown commands

    #[test]
    fn test_only_nl() {
        let line = "\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_command() {
        let line = "SERVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnknownCommand);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of RETRIEVE command

    #[test]
    fn test_retrieve_w_whitespace() {
        let line = "RETRIEVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve_payload() {
        let line = "RETRIEVE this has a payload\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve() {
        let line = "RETRIEVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Retrieve);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of PUBLISH command

    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_publish() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_payload() {
        let line = "PUBLISH\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::MissingPayload);
        assert_eq!(result, expected);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Debug, PartialEq, Eq)]
enum Error {
    TrailingData,
    IncompleteMessage,
    EmptyMessage, 
    UnknownCommand,
    UnexpectedPayload,
    MissingPayload,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let split = input.split_once('\n');
   
    if split.is_none() {
        return Err(Error::IncompleteMessage);
    }

    if let Some((line1, line2)) = split {
        if line1.is_empty() {
            return Err(Error::EmptyMessage)
        }
        if !line2.is_empty() {
            return Err(Error::TrailingData);
        }

        let mut splitn = line1.splitn(9 as usize, ' ');
        let command = splitn.next();
        match command.unwrap() {
            "RETRIEVE" => {
                if let Some(_) = splitn.next() {
                    return Err(Error::UnexpectedPayload);
                } 
                return Ok(Command::Retrieve);
            },
            "PUBLISH" => {
                let mut payload = Vec::new();
                while let Some(x) = splitn.next() {
                    payload.push(x);
                }
                if payload.is_empty() {
                    return Err(Error::MissingPayload);
                }
                return Ok(Command::Publish(payload.join(" ")));
            },
            "" => {
                return Err(Error::MissingPayload); 
            }
            _ => {
                return Err(Error::UnknownCommand); 
            }
        };
    }
    Err(Error::UnknownCommand)
}
