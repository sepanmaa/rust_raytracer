use std;

use vector::Vector3;


pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Box<Geometry>>,
}

#[derive(Copy,Clone)]
pub struct Material {
    pub shininess: f64,
    pub spec_color: Vector3,
    pub color: Vector3,
    pub reflection: f64,
}

pub struct Sphere {
    pub pos: Vector3,
    pub radius: f64,
    pub material: Material
}

pub struct Plane {
    pub pos: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

pub struct BBox {
    pub v1: Vector3,
    pub v2: Vector3,
    pub material: Material,
}

pub struct Camera {
    pub pos: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub dist: f64
}

pub struct Light {
    pub pos: Vector3,
    pub color: Vector3
}

pub struct Intersection {
    pos: Vector3,
    normal: Vector3,
    dist: f64,
    material: Material,
}

pub struct Ray {
    origin: Vector3,
    dir: Vector3,
}

pub trait Geometry {
    fn material(&self) -> Material;
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}

impl Scene {
    pub fn add<T: Geometry + 'static>(&mut self, g: T) {
        self.objects.push(Box::new(g));
    }
}

impl Geometry for BBox {
    fn material(&self) -> Material {
        self.material
    }
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let mut tnear = -(std::f64::INFINITY);
        let mut tfar = std::f64::INFINITY;

        if ray.dir.x == 0.0 && ray.origin.x < self.v1.x && ray.origin.x > self.v2.x {
            return None;
        }

        let mut t1 = (self.v1.x - ray.origin.x) / ray.dir.x;
        let mut t2 = (self.v2.x - ray.origin.x) / ray.dir.x;
        let mut n = v3!(1.0, 0.0, 0.0);

        if t1 > t2 { std::mem::swap(&mut t1, &mut t2);  }
        if t1 > tnear { tnear = t1; n = v3!(1.0, 0.0, 0.0); }
        if t2 < tfar { tfar = t2; }
        if tnear > tfar || tfar < 0.0 { return None; }

        t1 = (self.v1.y - ray.origin.y) / ray.dir.y;
        t2 = (self.v2.y - ray.origin.y) / ray.dir.y;

        if t1 > t2 { std::mem::swap(&mut t1, &mut t2);  }
        if t1 > tnear { tnear = t1; n = v3!(0.0, 1.0, 0.0);
        }
        if t2 < tfar { tfar = t2; }
        if tnear > tfar || tfar < 0.0 { return None; }

        t1 = (self.v1.z - ray.origin.z) / ray.dir.z;
        t2 = (self.v2.z - ray.origin.z) / ray.dir.z;

        if t1 > t2 { std::mem::swap(&mut t1, &mut t2);  }
        if t1 > tnear { tnear = t1; n = v3!(0.0, 0.0, -1.0); }
        if t2 < tfar { tfar = t2; }
        if tnear > tfar || tfar < 0.0 { return None; }

        Some(Intersection { pos: ray.origin + ray.dir * tnear,
                            normal: n,
                            dist: tnear,
                            material: self.material() })
    }
}

impl Geometry for Sphere {
    fn material(&self) -> Material {
        self.material
    }
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let l = self.pos - ray.origin;
        let tca = l.dot(ray.dir);
        if tca < 0.0 {
            return None;
        }
        let d = (l.dot(l)-tca*tca).sqrt();
        let d2 = d*d;
        let radius2 = self.radius*self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 { return None; }
        let t = if t0 < 0.0 { t1} else { t0.min(t1) };

        let p = ray.origin+ray.dir*t;
        let n = (p-self.pos).normalize();
        Some (Intersection {pos: p, normal: n, dist: t, material: self.material() })
    }
}

impl Geometry for Plane {
    fn material(&self) -> Material {
        self.material
    }
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let t = (self.pos - ray.origin).dot(self.normal)/(ray.dir.dot(self.normal));
        if t > 0.0 {
            let p = ray.origin+ray.dir*t;
            let mut material = self.material();
            let c = ((((2.0*p.z).floor() + (2.0*p.x).floor()) as i32) % 2) as f64;
            material.color = material.color * c.abs();
            Some(Intersection {
                dist: t,
                normal: self.normal,
                pos: p,
                material: material,
            })
        } else {
            None
        }
    }
}

fn cast_ray(scene: &Scene, ray: &Ray) -> Option<Intersection> {
    let mut closest: f64 = std::f64::INFINITY;
    let mut isect: Option<Intersection> = None;
    for o in scene.objects.iter() {
        match o.intersects(&ray) {
            Some(i) => {
                if i.dist < closest {
                    closest = i.dist;
                    isect = Some(i);
                }
            },
            _ => {}
        }
    }
    isect
}

fn blinn_phong(light_dir: Vector3, isect: &Intersection) -> Vector3 {
    let material = isect.material;
    let diffuse = light_dir.dot(isect.normal).max(0.0);
    let mut specular = 0.0;
    if diffuse > 0.0 {
        let view_dir = (isect.pos * (-1.0)).normalize();
        let half_dir = (light_dir + view_dir).normalize();
        let spec_angle = half_dir.dot(isect.normal).max(0.0);
        specular = spec_angle.powf(material.shininess);
    }
    return material.color * diffuse + material.spec_color * specular;
}

fn shade_pixel(scene: &Scene, ray: &Ray, trace_depth: i32) -> Vector3 {
    let mut pixel = v3!(0.0, 0.0, 0.0);
    match cast_ray(&scene, &ray) {
        None => { pixel = v3!(0.0, 0.4, 1.0); }, // background color
        Some(isect) => {
            for ref light in &scene.lights {
                let light_dir = (light.pos - isect.pos).normalize();
                let shadow_ray = Ray { origin: isect.pos+light_dir*0.001,
                                       dir: light_dir };
                match cast_ray(&scene, &shadow_ray) {
                    Some(..) => { },
                    None => { pixel = pixel + blinn_phong(light_dir, &isect); }
                }
                pixel = pixel + isect.material.color * 0.1; // ambient
                    
                if isect.material.reflection > 0.0 {                        
                    let reflection_dir = ray.dir - isect.normal*ray.dir.dot(isect.normal)*2.0;
                    let reflection_ray = Ray { origin: isect.pos+reflection_dir*0.001,
                                               dir: reflection_dir };
                    if trace_depth > 0 {
                        pixel = shade_pixel(&scene, &reflection_ray, trace_depth - 1)
                            * isect.material.reflection;
                    }
                }
            }
        }
    }
    return pixel;
}

pub fn raytrace(scene: &Scene, width: usize, height: usize) -> Vec<Vector3> {
    let mut pixels: Vec<Vector3> = vec![v3!(0.0, 0.0, 0.0); width*height];
    for y in 0..height {
        for x in 0..width {
            let u = (x as f64) * 2.0 / (width as f64) - 1.0;
            let v = (y as f64) * 2.0 / (height as f64) - 1.0;
            let camera = &scene.camera;
            let forward = camera.right.cross(&camera.up).normalize();
            let pos =
                camera.pos
                + forward*camera.dist
                + camera.right*u
                + camera.up*v;
            let ray_dir: Vector3 = (pos-camera.pos).normalize();
            let ray = Ray { origin: camera.pos.clone(), dir: ray_dir };
            pixels[(height-1-y)*width+x] = shade_pixel(&scene, &ray, 3);
        }
    }
    pixels
}

pub fn scene() -> Scene {
    let cam = Camera {
        pos: v3!(0.0, 0.0, -1.0),
        up: v3!(0.0, 1.0, 0.0),
        right: v3!(1.33, 0.0, 0.0),
        dist: 2.0,
    };
    Scene { camera: cam, lights: vec![], objects: vec![] }
}


pub fn basic_material(color: Vector3) -> Material {
    Material { shininess: 16.0,
               spec_color: v3!(1.0, 1.0, 1.0),
               color: color,
               reflection: 0.0  }
}
