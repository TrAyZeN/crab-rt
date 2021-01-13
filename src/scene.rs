use crate::objects::Object;

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    #[inline]
    pub const fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }
    #[inline]
    pub const fn from_objects(objects: Vec<Object>) -> Scene {
        Scene { objects }
    }

    #[inline]
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    #[inline]
    pub const fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}
