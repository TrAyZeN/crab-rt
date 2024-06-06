use alloc::vec::Vec;

use crate::bvh::BvhNode;
use crate::materials::Material;
use crate::objects::{Object, Sphere};
use crate::vec::Color3;

/// A structure containing what to render.
#[derive(Debug, Default)]
pub struct Scene {
    bvh: BvhNode,
    background: Background,
}

impl Scene {
    /// Constructs a new `Scene` containing the given objects and the given background.
    ///
    /// # Examples
    /// ```
    /// ```
    #[inline]
    #[must_use]
    pub fn new(objects: Vec<Object>, background: Background) -> Self {
        let bvh = if objects.is_empty() {
            BvhNode::default()
        } else {
            BvhNode::new(objects, (0., 0.1)) // TODO: time inteval
        };

        Self { bvh, background }
    }

    /// Returns the bvh of the objects present in the scene.
    ///
    /// # Examples
    /// ```
    /// ```
    #[inline]
    #[must_use]
    pub const fn bvh(&self) -> &BvhNode {
        &self.bvh
    }

    /// Returns the background color of the scene.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::scene::{Background, Scene};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene = Scene::new(Vec::new(), Background::Color(Vec3::new(0.1, 0.2, 0.3)));
    /// assert_eq!(
    ///     scene.background(),
    ///     &Background::Color(Vec3::new(0.1, 0.2, 0.3))
    /// );
    ///
    /// let scene = Scene::new(
    ///     Vec::new(),
    ///     Background::Gradient(Vec3::new(0.1, 0.2, 0.3), Vec3::new(1., 1., 1.)),
    /// );
    /// assert_eq!(
    ///     scene.background(),
    ///     &Background::Gradient(Vec3::new(0.1, 0.2, 0.3), Vec3::new(1., 1., 1.))
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn background(&self) -> &Background {
        &self.background
    }
}

/// A builder for `Scene`.
#[derive(Debug, Default)]
pub struct SceneBuilder {
    objects: Vec<Object>,
    background: Background,
}

impl SceneBuilder {
    /// Constructs a new empty `SceneBuilder`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::scene::{Background, SceneBuilder};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene_builder = SceneBuilder::new(Background::Color(Vec3::zero()));
    /// assert_eq!(
    ///     scene_builder.build().background(),
    ///     &Background::Color(Vec3::zero())
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(background: Background) -> Self {
        Self {
            objects: Vec::new(),
            background,
        }
    }

    /// Adds an object to the `SceneBuilder`.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    ///
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::objects::{Object, Sphere};
    /// use crab_rt::scene::{Background, SceneBuilder};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene_builder = SceneBuilder::new(Background::Color(Vec3::zero())).add_object(Object::new(
    ///     Sphere::new(Vec3::zero(), 1., Arc::new(Lambertian::default())),
    /// ));
    /// ```
    #[inline]
    #[must_use]
    pub fn add_object(mut self, object: Object) -> Self {
        self.objects.push(object);

        self
    }

    /// Adds a `Sphere<M>` to the `SceneBuilder`.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    ///
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::objects::{Object, Sphere};
    /// use crab_rt::scene::{Background, SceneBuilder};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene_builder = SceneBuilder::new(Background::Color(Vec3::zero())).add_sphere(Sphere::new(
    ///     Vec3::zero(),
    ///     1.,
    ///     Arc::new(Lambertian::default()),
    /// ));
    /// ```
    #[inline]
    #[must_use]
    pub fn add_sphere<M: 'static + Material>(self, sphere: Sphere<M>) -> Self {
        self.add_object(Object::new(sphere))
    }

    /// Consumes the `SceneBuilder` to build a `Scene`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::scene::{Background, SceneBuilder};
    /// use crab_rt::vec::Vec3;
    ///
    /// let scene = SceneBuilder::new(Background::Color(Vec3::zero())).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn build(self) -> Scene {
        Scene::new(self.objects, self.background)
    }
}

#[derive(Debug, PartialEq)]
pub enum Background {
    Color(Color3),
    Gradient(Color3, Color3),
}

impl Background {
    #[must_use]
    pub fn color(&self, t: f32) -> Color3 {
        match self {
            Self::Color(c) => *c,
            Self::Gradient(c1, c2) => t * c1 + (1. - t) * c2,
        }
    }
}

impl Default for Background {
    #[inline]
    fn default() -> Self {
        Self::Color(Color3::zero())
    }
}
