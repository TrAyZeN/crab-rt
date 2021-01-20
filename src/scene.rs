use crate::objects::Object;
use crate::vec::Vec3;

#[derive(Debug, Default)]
pub struct Scene {
    objects: Vec<Object>,
    background: Background,
}

impl Scene {
    /// Constructs a new empty `Scene`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::scene::{Scene, Background};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene = Scene::new(Background::Color(Vec3::zero()));
    /// assert!(scene.get_objects().is_empty());
    /// ```
    #[inline]
    pub const fn new(background: Background) -> Self {
        Self {
            objects: Vec::new(),
            background,
        }
    }

    /// Constructs a new `Scene` containing the given objects.
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    #[inline]
    pub const fn from_objects(objects: Vec<Object>, background: Background) -> Self {
        Self {
            objects,
            background,
        }
    }

    /// Adds an object to the scene.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::scene::{Scene, Background};
    /// use crab_rt::vec::Vec3;
    /// use crab_rt::objects::Object;
    /// use crab_rt::materials::Lambertian;
    ///
    /// let scene = Scene::new(Background::Color(Vec3::zero()));
    /// scene.add_object(Object::new(Sphere::new(
    ///     Vec3::zero(),
    ///     1.,
    ///     Lambertian::default(),
    /// )));
    /// assert_eq!(scene.get_objects()[0], Object::new(Sphere::new(
    ///     Vec3::zero(),
    ///     1.,
    ///     Lambertian::default(),
    /// )));
    /// ```
    #[inline]
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    /// Returns the objects present in the scene.
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    #[inline]
    pub const fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    /// Returns the background color of the scene.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::scene::{Background, Scene};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene = Scene::new(Background::Color(Vec3::new(0.1, 0.2, 0.3)));
    /// assert_eq!(scene.get_background(), &Background::Color(Vec3::new(0.1, 0.2, 0.3)));
    ///
    /// let scene = Scene::new(Background::Gradient(
    ///     Vec3::new(0.1, 0.2, 0.3),
    ///     Vec3::new(1., 1., 1.),
    /// ));
    /// assert_eq!(
    ///     scene.get_background(),
    ///     &Background::Gradient(Vec3::new(0.1, 0.2, 0.3), Vec3::new(1., 1., 1.))
    /// );
    /// ```
    #[inline]
    pub const fn get_background(&self) -> &Background {
        &self.background
    }
}

#[derive(Debug, PartialEq)]
pub enum Background {
    Color(Vec3),
    Gradient(Vec3, Vec3),
}

impl Background {
    pub fn color(&self, t: f32) -> Vec3 {
        match self {
            Self::Color(c) => c.clone(),
            Self::Gradient(c1, c2) => t * c1 + (1. - t) * c2,
        }
    }
}

impl Default for Background {
    #[inline]
    fn default() -> Self {
        Self::Color(Vec3::zero())
    }
}
