use image::{ImageBuffer, Rgb};
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

use crate::common::*;

pub struct GPURenderingParams {
    pub acc: GPUAcceleratedStructure,
    pub camera: Camera,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub aspect_ratio: f32,
    pub path: String,
    pub background: Color,
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

pub fn render_world_gpu(params: GPURenderingParams) {
    let GPURenderingParams {
        acc,
        camera,
        image_width,
        samples_per_pixel,
        max_depth,
        aspect_ratio,
        path,
        background,
    } = params;

    let now = Instant::now();
    println!("begin rendering...");

    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let position = vec![Attribute {
        a: Vec3::new(0.0, 0.0, 0.0),
        b: Vec3::new(1.0, 0.0, 0.0),
        c: Vec3::new(0.0, 1.0, 0.0),
    }];

    // prepare data

    let pixels_count = image_width * image_height;
    let uniforms = [
        // image size
        image_width as f32,
        image_height as f32,
        samples_per_pixel as f32,
        max_depth as f32,
        // camera
        camera.origin.x,
        camera.origin.y,
        camera.origin.z,
        0.0,
        //
        camera.lower_left_corner.x,
        camera.lower_left_corner.y,
        camera.lower_left_corner.z,
        0.0,
        //
        camera.horizontal.x,
        camera.horizontal.y,
        camera.horizontal.z,
        0.0,
        //
        camera.vertical.x,
        camera.vertical.y,
        camera.vertical.z,
        0.0,
        //
        camera.u.x,
        camera.u.y,
        camera.u.z,
        0.0,
        //
        camera.v.x,
        camera.v.y,
        camera.v.z,
        camera.lens_radius,
    ];

    // data
    let position_data = position
        .iter()
        .map(|p| {
            vec![
                p.a.x, p.a.y, p.a.z, 0.0, //
                p.b.x, p.b.y, p.b.z, 0.0, //
                p.c.x, p.c.y, p.c.z, 0.0,
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
                path: "./src/shaders/one_weekend.glsl"
            }
        }
        let shader = cs::Shader::load(device.clone()).unwrap();
        let spec_const = cs::SpecializationConstants {
            primitive_count: position.len() as u32,
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
        let data_iter = position_data.iter().map(|x| *x);
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

    println!(
        "rendered for {} s",
        now.elapsed().as_millis() as f32 / 1000.0
    );

    println!("saving -> {}", path);

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(image_width, image_height, color_data).unwrap();
    img.save(path).unwrap();
}
