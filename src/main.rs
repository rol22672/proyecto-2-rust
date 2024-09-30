mod camera;
mod cube;
mod material;
mod color;
mod ray_intersect;
mod framebuffer;
mod intersect;

use nalgebra_glm::{Vec3};
use crate::camera::Camera;
use crate::cube::Cube;
use crate::material::Material;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent, DeviceEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    // Configurar la ventana utilizando winit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ray Tracing Cube")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut framebuffer = Framebuffer::new(800, 600);

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(800, 600, surface_texture).expect("Failed to create pixel buffer");

    // Configurar la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 2.0, 5.0),   // Posición de la cámara
        Vec3::new(0.0, 0.0, 0.0),   // Centro de la escena
        Vec3::new(0.0, 1.0, 0.0),   // Vector "up" para la cámara
    );

    // Variables para rastrear el movimiento del mouse
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    // Definir el material del cubo de césped utilizando la textura
    let grass_material = Material::new(
        Color::new(139, 69, 19),  // Color base café (brown)
        0.6,                      // Albedo - reduced for a less reflective, more matte appearance
        0.2,                      // Specular - lower value for non-shiny dirt
        0.05,                     // Reflectividad - very low, as dirt isn't reflective
        0.0,                      // Transparencia - unchanged, as dirt is not transparent
        true,                     // Indicar que tiene textura
        Some("textures/dirt.webp"), // Ruta de la textura
    );



    // Crear el cubo de césped en la posición central
    let grass_cube = Cube::new(Vec3::new(0.0, 0.0, 0.0), 1.0, grass_material);

    // Escena de objetos
    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(grass_cube)];

    // Render loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::RedrawRequested(_) => {
                render_scene(&camera, &objects, &mut framebuffer);

                // Copiar framebuffer a la ventana
                for (i, pixel) in framebuffer.pixels.iter().enumerate() {
                    let frame = pixels.get_frame();
                    let offset = i * 4;
                    frame[offset] = pixel.r;
                    frame[offset + 1] = pixel.g;
                    frame[offset + 2] = pixel.b;
                    frame[offset + 3] = 255;
                }

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(new_size) => {
                    pixels.resize_surface(new_size.width, new_size.height);
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                    // Actualiza los valores de yaw y pitch según el movimiento del ratón
                    yaw += dx as f32 * 0.01;
                    pitch -= dy as f32 * 0.01;

                    // Limitar el rango de pitch para evitar gimbal lock
                    pitch = pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);

                    // Calcular la nueva posición de la cámara
                    let distance = 5.0; // Distancia fija desde el cubo
                    let eye_x = distance * yaw.cos() * pitch.cos();
                    let eye_y = distance * pitch.sin();
                    let eye_z = distance * yaw.sin() * pitch.cos();

                    camera.eye = Vec3::new(eye_x, eye_y, eye_z);
                }
                _ => (),
            },
            _ => (),
        }

        window.request_redraw(); // Solicita redibujar la ventana
    });
}

fn render_scene(camera: &Camera, objects: &[Box<dyn RayIntersect>], framebuffer: &mut Framebuffer) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let u = x as f32 / framebuffer.width as f32;
            let v = y as f32 / framebuffer.height as f32;

            // Obtener la dirección del rayo desde la cámara
            let ray_direction = camera.get_ray_direction(u, v, framebuffer.aspect_ratio());

            // Comprobar intersecciones con los objetos en la escena
            let color = cast_ray(camera.eye, ray_direction, &objects);

            // Dibujar el píxel en el framebuffer
            framebuffer.set_pixel(x, y, color);
        }
    }
}

fn cast_ray(origin: Vec3, direction: Vec3, objects: &[Box<dyn RayIntersect>]) -> Color {
    let mut closest_intersection: Option<Intersect> = None;

    for object in objects {
        let intersection = object.ray_intersect(&origin, &direction);

        if intersection.is_intersecting {
            if closest_intersection.is_none() || intersection.distance < closest_intersection.as_ref().unwrap().distance {
                closest_intersection = Some(intersection);
            }
        }
    }

    if let Some(intersect) = closest_intersection {
        if intersect.material.has_texture {
            return intersect.material.get_color_from_texture(intersect.u, intersect.v); // Usa la textura si está disponible
        }
        return intersect.material.diffuse; // Devuelve el color difuso del material
    }

    Color::new(135, 206, 235) // Color de fondo (cielo, por ejemplo)
}
