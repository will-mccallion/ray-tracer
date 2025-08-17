use crate::{
    hittable::HitRecord,
    math::{ray::Ray, vec3::Vec3},
    scene::Scene,
};

pub trait Material: Send + Sync {
    fn shade(&self, hit_record: &HitRecord, scene: &Scene) -> Vec3<f64>;
}

pub struct Lambertian {
    albedo: Vec3<f64>,
}

impl Lambertian {
    pub fn new(color: image::Rgb<u8>) -> Self {
        let albedo = Vec3::new(
            color[0] as f64 / 255.0,
            color[1] as f64 / 255.0,
            color[2] as f64 / 255.0,
        );
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn shade(&self, hit_record: &HitRecord, scene: &Scene) -> Vec3<f64> {
        const SHADOW_EPSILON: f64 = 0.001;

        let mut final_color = self.albedo * scene.ambient_light;

        let primary_light = &scene.lights[0];
        let light_dir = (primary_light.position - hit_record.point).normalize();
        let dist_to_light = (primary_light.position - hit_record.point).length();

        let shadow_ray_origin = hit_record.point + hit_record.normal * SHADOW_EPSILON;
        let shadow_ray = Ray::new(shadow_ray_origin, light_dir);

        let in_shadow = scene
            .hittables
            .iter()
            .any(|obj| obj.hit(&shadow_ray, 0.001, dist_to_light).is_some());

        if !in_shadow {
            let diffuse_intensity = hit_record.normal.dot(&light_dir).max(0.0);
            let light_color = Vec3::new(1.0, 1.0, 1.0) * primary_light.intensity;
            let diffuse_contribution = self.albedo * light_color * diffuse_intensity;
            final_color = final_color + diffuse_contribution;
        }

        final_color
    }
}
