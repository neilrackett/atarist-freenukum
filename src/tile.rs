use sdl2::render::{Texture, TextureCreator};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Back0,
    Back1,
    Back2,
    Back3,
    Solid0,
    Solid1,
    Solid2,
    Solid3,
    Anim0,
    Anim1,
    Anim2,
    Anim3,
    Anim4,
    Anim5,
    Object0,
    Object1,
    Object2,
    Man0,
    Man1,
    Man2,
    Man3,
    Man4,
    Font1,
    Font2,
    Border,
    Numbers,
}

impl Category {
    pub fn basename(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }

    pub fn filename(&self) -> PathBuf {
        let mut s = self.basename();
        s.push_str(".DN1");
        PathBuf::from(s)
    }

    pub fn count(&self) -> u8 {
        use Category as C;
        match self {
            C::Back0
            | C::Back1
            | C::Back2
            | C::Back3
            | C::Solid0
            | C::Solid1
            | C::Solid2
            | C::Solid3
            | C::Anim0
            | C::Anim1
            | C::Anim2
            | C::Anim3
            | C::Anim4
            | C::Anim5
            | C::Man0
            | C::Man1
            | C::Man2
            | C::Man3
            | C::Man4
            | C::Font1
            | C::Font2
            | C::Border
            | C::Numbers => 48,

            C::Object0 | C::Object1 | C::Object2 => 50,
        }
    }

    pub fn transparent(&self) -> bool {
        use Category as C;
        match self {
            C::Back0
            | C::Solid0
            | C::Anim0
            | C::Anim1
            | C::Anim2
            | C::Anim3
            | C::Anim4
            | C::Anim5
            | C::Object0
            | C::Object1
            | C::Object2
            | C::Man0
            | C::Man1
            | C::Man2
            | C::Man3
            | C::Man4
            | C::Font1
            | C::Font2
            | C::Border
            | C::Numbers => true,

            C::Back1
            | C::Back2
            | C::Back3
            | C::Solid1
            | C::Solid2
            | C::Solid3 => false,
        }
    }
}

pub struct Tiles<'t, T> {
    path: PathBuf,
    creator: TextureCreator<T>,
    cache: BTreeMap<Category, Vec<Texture<'t>>>,
}

impl<'t, T> Tiles<'t, T> {
    pub fn new<P: AsRef<Path>>(
        path: P,
        creator: TextureCreator<T>,
    ) -> Self {
        Tiles {
            path: path.as_ref().to_path_buf(),
            creator,
            cache: BTreeMap::new(),
        }
    }

    fn get_cached<'u>(
        &mut self,
        category: &Category,
    ) -> Result<&'t Vec<Texture<'u>>, String>
    where
        'u: 't,
    {
        use std::collections::btree_map::Entry;
        let entry: Entry<'t, _, _> = self.cache.entry(*category);
        match entry {
            Entry::Vacant(entry) => {
                Ok(entry.insert(self.load_textures(category)?))
            }
            Entry::Occupied(entry) => Ok(entry.get()),
        }
    }

    fn load_textures(
        &self,
        category: &Category,
    ) -> Result<Vec<Texture<'t>>, String> {
        unimplemented!();
    }

    pub fn get_background(
        &mut self,
        index: u8,
    ) -> Result<Texture<'t>, String> {
        unimplemented!();
    }
}
