use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_whitelist(path: String) -> io::Result<Vec<String>> {
    lines_from_file(Path::new(&path))
}

fn lines_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect())
}
