use std::fs;
use std::io;
use std::io::prelude::*;
use conf::Conf;

#[derive(Clone)]
#[derive(Debug)]
pub struct Seed {
  filepath: String
}

static mut seed_count: i32 = 0;

impl Seed {
  pub fn new(conf:&Conf, filepath:&str) -> Seed {
    let path = unsafe {
      seed_count = seed_count + 1;
      format!("{}/queue/tc-{}", conf.output_dir, seed_count)
    };
    let new_seed = Seed { filepath: path };
    new_seed.copy_from_file(filepath);
    new_seed
  }

  pub fn load_buf(&self) -> Vec<u8> {
    let mut buf = vec![];
    fs::File::open(&self.filepath).unwrap().read_to_end(&mut buf).unwrap();
    buf
  }

  pub fn save_buf(&self, buf:&Vec<u8>) {
    let mut f = fs::File::create(&self.filepath).unwrap();
    f.write_all(buf).unwrap();
  }

  fn copy_from_file(&self, path:&str) {
    let mut buf:Vec<u8> = vec![];
    fs::File::open(&path).unwrap().read_to_end(&mut buf).unwrap();
    self.save_buf(&buf)
  }
}

pub fn load_seed_files(conf:&Conf, seed_dir:&str) -> io::Result<Vec<Seed>> {
  debug!("[*] Load seed files...");
  let seeds = try!(fs::read_dir(seed_dir))
                .filter_map(|entry| entry.ok())
                .filter_map(|e| e.path().to_str().and_then(|s| Some(String::from(s))))
                .map(|s| Seed::new(&conf, &s))
                .collect::<Vec<Seed>>();
  debug!("{:?}", seeds);
  Ok(seeds)
}
