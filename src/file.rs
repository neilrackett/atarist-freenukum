pub struct File(pub std::fs::File);

impl File {
    pub fn open(name: &str) -> std::io::Result<Self> {
        println!("Opening file {:?}", name);
        Ok(File(std::fs::File::open(name)?))
    }

    pub fn as_ref_mut(&mut self) -> &mut std::fs::File {
        &mut self.0
    }
}
