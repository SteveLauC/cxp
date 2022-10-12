use std::{
    env::args_os,
    ffi::{OsStr, OsString},
    fs::create_dir_all,
    io::Result,
    path::PathBuf,
    process::{exit, Command},
};

use dirs::data_dir;

const USAGE: &str = r#"
USAGE:
cxp command [operand]

    where `command` can be:
        c: copy files
        x: cut files
        p: paste files into $PWD
        l: list files
        t: list files in a tree format
        e: empty file buffer"#;

struct Cxp {
    data_dir: PathBuf,
}

impl Cxp {
    fn new() -> Self {
        let mut path = data_dir().expect("can not find data dir");
        path.push("cxp");
        let _ = create_dir_all(path.as_path());
        Self { data_dir: path }
    }

    fn copy<I, S>(&self, operand: I) -> Result<()>
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<OsStr>,
    {
        self.empty()?;
        Command::new("cp")
            .arg("-r")
            .args(operand)
            .arg(self.data_dir.as_path())
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn cut<I, S>(&self, operand: I) -> Result<()>
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<OsStr>,
    {
        self.empty()?;
        self.copy(operand.clone())?;
        Command::new("mv")
            .args(operand)
            .arg(self.data_dir.as_path())
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn paste(&self) -> Result<()> {
        for opt_file in self.data_dir.read_dir()? {
            let file = opt_file?;
            let path = file.path();

            Command::new("cp")
                .arg("-r")
                .arg(path)
                .arg(".")
                .spawn()?
                .wait()?;
        }

        Ok(())
    }

    fn empty(&self) -> Result<()> {
        for opt_file in self.data_dir.read_dir()? {
            let file = opt_file?;
            let path = file.path();

            Command::new("rm").arg("-r").arg(path).spawn()?.wait()?;
        }

        Ok(())
    }

    fn list(&self) -> Result<()> {
        Command::new("ls")
            .arg("-al")
            .arg(self.data_dir.as_path())
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn tree(&self) -> Result<()> {
        Command::new("tree")
            .arg("-al")
            .arg(self.data_dir.as_path())
            .spawn()?
            .wait()?;

        Ok(())
    }
}

fn main() {
    let mut av = args_os().skip(1);
    if av.len() < 1 {
        eprintln!("{}", USAGE);
        exit(1);
    }
    let cxp = Cxp::new();
    // `command` should be UTF-8 encoded, use `unwrap()` here.
    let cmd = av.next().unwrap().into_string().unwrap();
    let operand = av.collect::<Vec<OsString>>();
    let res = match cmd.as_str() {
        "c" => cxp.copy(operand),
        "x" => cxp.cut(operand),
        "p" => cxp.paste(),
        "l" => cxp.list(),
        "t" => cxp.tree(),
        "e" => cxp.empty(),
        _ => {
            eprintln!("Unsupported Command");
            eprintln!("{}", USAGE);
            exit(1);
        }
    };

    if let Err(msg) = res {
        eprintln!("Error: {:?}", msg);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        env::temp_dir,
        fs::{remove_file, File},
    };

    #[test]
    fn test_copy() {
        let cxp = Cxp::new();
        let mut buffer = cxp.data_dir.clone();

        let mut tmp_file = temp_dir();
        tmp_file.push("cxp_test_copy");

        let _ = File::create(tmp_file.as_path());

        assert!(tmp_file.exists());

        cxp.copy([tmp_file.as_path()])
            .expect("failed to copy tmp_file to the buffer");

        buffer.push("cxp_test_copy");
        assert!(buffer.exists());

        cxp.empty().expect("failed to empty the buffer");
        remove_file(tmp_file).expect("failed to clean temporary file");
    }

    #[test]
    fn test_move() {
        let cxp = Cxp::new();
        let mut buffer = cxp.data_dir.clone();

        let mut tmp_file = temp_dir();
        tmp_file.push("cxp_test_move");

        let _ = File::create(tmp_file.as_path());

        assert!(tmp_file.exists());

        cxp.copy([tmp_file.as_path()])
            .expect("failed to copy tmp_file to the buffer");

        buffer.push("cxp_test_move");
        assert!(buffer.exists());

        cxp.empty().expect("failed to empty the buffer");
        remove_file(tmp_file).expect("failed to clean temporary file");
    }
}
