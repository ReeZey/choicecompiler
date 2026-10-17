use std::{fs::read_dir, path::Path};

pub fn ask(path: impl AsRef<Path>, input_question: &str) -> anyhow::Result<String> {
    loop {
        let mut dates = Vec::new();

        for (idx, date) in read_dir(&path)?.enumerate() {
            if let Ok(pack) = date {
                let name = pack
                    .file_name()
                    .to_str()
                    .ok_or(anyhow::format_err!("no file name"))?
                    .to_string();
                println!("{idx}. {name:?}");
                dates.push(name);
            }
        }
        let input = take_input(input_question)?;

        let number = input
            .parse::<usize>()
            .map_err(|_| anyhow::format_err!("not a number: [{input}]"))?;

        if let Some(abc) = dates.get(number) {
            return Ok(abc.to_owned());
        } else {
            anyhow::bail!("invalid idx: [{input}]")
        }
    }
}

pub fn take_input(prefix: &str) -> anyhow::Result<String> {
    use std::io::{Write, stdin, stdout};

    print!("{} > ", prefix);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    Ok(input)
}
