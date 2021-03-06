use ray::Ray;
use vec3::Vec3;
use hittable::HitRecord;
use rand;
use rand::Rng;
use vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {albedo: Vec3},
    Metal {albedo: Vec3, fuzz: f32},
//    Dielectrinc {}
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0)}
    }
}

pub fn scatter(material: &Material, ray_in: &Ray, hit: & HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match material {
        &Material::Lambertian {ref albedo} => {
            let target = hit.p + hit.normal + random_point_in_unit_sphere();
            *scattered = Ray::new(hit.p, target-hit.p);
            *attenuation = *albedo;
            true
        },
        &Material::Metal {ref albedo, ref fuzz} => {
            let reflected = vec3::reflect(&ray_in.dir(), &hit.normal);
            *scattered = Ray::new(hit.p, reflected + *fuzz*random_point_in_unit_sphere());
            *attenuation = *albedo;
            
            vec3::dot(&scattered.dir(), &hit.normal) > 0.0
        }
    }
}
 
fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    
    loop {
        let rand_x = rng.gen_range(-1.0, 1.0);
        let rand_y = rng.gen_range(-1.0, 1.0);
        let rand_z = rng.gen_range(-1.0, 1.0);        
        let v = Vec3::new(rand_x, rand_y, rand_z);
        if v.square_length() < 1.0 {
            return v;
        }
    }
}