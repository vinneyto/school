extern crate nalgebra as na;

use image::{ImageBuffer, Rgb};
use na::{Point3, Point4};
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::{Device, DeviceExtensions};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::pipeline::ComputePipeline;
use vulkano::sync;
use vulkano::sync::GpuFuture;

use ray_tracing::clamp;

fn main() {
    let image_width = 800;
    let image_height = 600;

    let now = Instant::now();
    println!("begin rendering...");

    let color_data = compute(
        image_width,
        image_height,
        &[Primitive::sphere(Point3::new(1.0, 2.0, 3.0), 0.5)],
    );

    println!(
        "rendered for {} s",
        now.elapsed().as_millis() as f32 / 1000.0
    );

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(image_width, image_height, color_data).unwrap();
    let path = "one_weekend_vk.bmp";
    img.save(path).unwrap();
}

const DEVICE_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_storage_buffer_storage_class: true,
    ..DeviceExtensions::none()
};

const UNIFORM_BUFFER_USAGE: BufferUsage = BufferUsage {
    uniform_buffer: true,
    ..BufferUsage::none()
};

const BUFFER_USAGE: BufferUsage = BufferUsage {
    storage_buffer: true,
    ..BufferUsage::none()
};

const GROUP_SIZE: u32 = 32;

struct Primitive {
    pub a: Point4<f32>,
    pub b: Point4<f32>,
    pub c: Point4<f32>,
    pub d: Point4<f32>,
}

impl Primitive {
    pub fn sphere(center: Point3<f32>, radius: f32) -> Self {
        Self {
            a: Point4::new(center.x, center.y, center.z, radius),
            b: Point4::new(0.0, 0.0, 0.0, 0.0),
            c: Point4::new(0.0, 0.0, 0.0, 0.0),
            d: Point4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

fn compute(image_width: u32, image_height: u32, primitives: &[Primitive]) -> Vec<u8> {
    // prepare data

    let pixels_count = image_width * image_height;
    let uniforms = [image_width as f32, image_height as f32];
    let primitives_data = primitives
        .iter()
        .map(|p| {
            vec![
                p.a.x, p.a.y, p.a.z, p.a.w, //
                p.b.x, p.b.y, p.b.z, p.b.w, //
                p.c.x, p.c.y, p.c.z, p.c.w, //
                p.d.x, p.d.y, p.d.z, p.d.w, //
            ]
        })
        .flatten()
        .collect::<Vec<f32>>();

    // configure Vulkan

    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

    println!("Device {}", physical.name());

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_compute())
        .unwrap();

    let (device, mut queues) = Device::new(
        physical,
        physical.supported_features(),
        &DEVICE_EXTENSIONS,
        [(queue_family, 0.5)].iter().cloned(),
    )
    .unwrap();

    let queue = queues.next().unwrap();

    println!("Device initialized");

    let pipeline = Arc::new({
        mod cs {
            vulkano_shaders::shader! {
                ty: "compute",
                path: "./src/one_weekend_vk.glsl"
            }
        }
        let shader = cs::Shader::load(device.clone()).unwrap();
        let spec_const = cs::SpecializationConstants {
            primitive_count: primitives.len() as u32,
        };
        ComputePipeline::new(
            device.clone(),
            &shader.main_entry_point(),
            &spec_const,
            None,
        )
        .unwrap()
    });

    // buffers

    let uniform_buffer = {
        let data_iter = uniforms.iter().map(|x| *x);
        CpuAccessibleBuffer::from_iter(device.clone(), UNIFORM_BUFFER_USAGE, false, data_iter)
            .unwrap()
    };

    let color_buffer = {
        let data_iter = (0..pixels_count * 4).map(|_| 0.0 as f32);
        CpuAccessibleBuffer::from_iter(device.clone(), BUFFER_USAGE, false, data_iter).unwrap()
    };

    let primitives_buffer = {
        let data_iter = primitives_data.iter().map(|x| *x);
        CpuAccessibleBuffer::from_iter(device.clone(), BUFFER_USAGE, false, data_iter).unwrap()
    };

    // layout

    let layout = pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_buffer(uniform_buffer.clone())
            .unwrap()
            .add_buffer(color_buffer.clone())
            .unwrap()
            .add_buffer(primitives_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .dispatch(
            [pixels_count / GROUP_SIZE, 1, 1],
            pipeline.clone(),
            set.clone(),
            (),
            vec![],
        )
        .unwrap();
    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let color_buffer_content = color_buffer.read().unwrap();
    let mut color_data: Vec<u8> = vec![];

    for i in 0..pixels_count {
        let r = color_buffer_content[(i * 4) as usize];
        let g = color_buffer_content[(i * 4) as usize + 1];
        let b = color_buffer_content[(i * 4) as usize + 2];

        color_data.push((256.0 * clamp(r, 0.0, 0.999)) as u8);
        color_data.push((256.0 * clamp(g, 0.0, 0.999)) as u8);
        color_data.push((256.0 * clamp(b, 0.0, 0.999)) as u8);
    }

    color_data
}
