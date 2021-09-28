import vertShaderCode from './shaders/triangle.vert.wgsl';
import fragShaderCode from './shaders/triangle.frag.wgsl';

export async function startWebGPUDemo() {
  const demo = new WebGPUDemo();

  await demo.initDevice();
  await demo.setupRendering();

  console.log(demo);
}

class WebGPUDemo {
  private device!: GPUDevice;
  private canvas!: HTMLCanvasElement;
  private ctx!: GPUCanvasContext;

  async initDevice() {
    const entry = navigator.gpu;
    if (!entry) {
      throw new Error('WebGPU is not supported on this browser.');
    }

    const adapter = await entry.requestAdapter();
    if (adapter === null) {
      throw new Error('unable to get adapter');
    }
    const device = await adapter.requestDevice();

    const canvas = document.createElement('canvas');
    document.body.appendChild(canvas);

    const ctx = canvas.getContext('webgpu');
    if (ctx === null) {
      throw new Error('unable to get gpu context');
    }

    this.canvas = canvas;
    this.ctx = ctx;

    this.resizeCanvas();

    const canvasConfig: GPUCanvasConfiguration = {
      device,
      format: 'bgra8unorm',
      usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
      size: [canvas.width, canvas.height],
    };
    ctx.configure(canvasConfig);

    this.device = device;

    window.addEventListener('resize', this.resizeCanvas);
  }

  async setupRendering() {
    const { device, ctx } = this;
    // const depthTexture = device.createTexture({
    //   size: [canvas.width, canvas.height, 1],
    //   dimension: '2d',
    //   format: 'depth24plus-stencil8',
    //   usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
    // });
    // const depthTextureView = depthTexture.createView();

    const colorTexture = ctx.getCurrentTexture();
    const colorTextureView = colorTexture.createView();

    // 📈 Position Vertex Buffer Data
    const positions = new Float32Array([
      1.0,
      -1.0,
      0.0,
      -1.0,
      -1.0,
      0.0,
      0.0,
      1.0,
      0.0,
    ]);

    for (let i = 0; i < positions.length; i++) {
      positions[i] = positions[i] * 0.5;
    }

    // 🎨 Color Vertex Buffer Data
    const colors = new Float32Array([
      1.0,
      0.0,
      0.0, // 🔴
      0.0,
      1.0,
      0.0, // 🟢
      0.0,
      0.0,
      1.0, // 🔵
    ]);

    // 📇 Index Buffer Data
    const indices = new Uint16Array([0, 1, 2]);

    const positionBuffer = this.createBuffer(positions, GPUBufferUsage.VERTEX);
    const colorBuffer = this.createBuffer(colors, GPUBufferUsage.VERTEX);
    const indexBuffer = this.createBuffer(indices, GPUBufferUsage.INDEX);

    const vertModule = device.createShaderModule({ code: vertShaderCode });
    const fragModule = device.createShaderModule({ code: fragShaderCode });

    const uniformData = new Float32Array([
      // ♟️ ModelViewProjection Matrix (Identity)
      1.0,
      0.0,
      0.0,
      0.0,
      0.0,
      1.0,
      0.0,
      0.0,
      0.0,
      0.0,
      1.0,
      0.0,
      0.0,
      0.0,
      0.0,
      1.0,

      // 🔴 Primary Color
      0.9,
      0.1,
      0.3,
      1.0,

      // 🟣 Accent Color
      0.8,
      0.2,
      0.8,
      1.0,
    ]);

    const uniformBuffer = this.createBuffer(
      uniformData,
      GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
    );

    const uniformBindGroupLayout = device.createBindGroupLayout({
      entries: [
        {
          binding: 0,
          visibility: GPUShaderStage.VERTEX,
          buffer: { type: 'uniform' },
        },
      ],
    });

    const uniformBindGroup = device.createBindGroup({
      layout: uniformBindGroupLayout,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: uniformBuffer,
          },
        },
      ],
    });

    const pipelineLayoutDesc = { bindGroupLayouts: [uniformBindGroupLayout] };
    const layout = this.device.createPipelineLayout(pipelineLayoutDesc);

    const pipelineDesc: GPURenderPipelineDescriptor = {
      layout,
      vertex: {
        module: vertModule,
        entryPoint: 'main',
        buffers: [
          {
            attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }],
            arrayStride: 4 * 3,
            stepMode: 'vertex',
          },
          {
            attributes: [{ shaderLocation: 1, offset: 0, format: 'float32x3' }],
            arrayStride: 4 * 3,
            stepMode: 'vertex',
          },
        ],
      },
      fragment: {
        module: fragModule,
        entryPoint: 'main',
        targets: [
          {
            format: 'bgra8unorm',
          },
        ],
      },
      primitive: {
        frontFace: 'cw',
        cullMode: 'none',
        topology: 'triangle-list',
      },
      // depthStencil: {
      //   depthWriteEnabled: true,
      //   depthCompare: 'less',
      //   format: 'depth24plus-stencil8',
      // },
    };

    const pipeline = device.createRenderPipeline(pipelineDesc);

    const encodeCommands = () => {
      const colorAttachment: GPURenderPassColorAttachment = {
        view: colorTextureView,
        loadValue: { r: 0, g: 0, b: 0, a: 1 },
        storeOp: 'store',
      };

      // const depthAttachment: GPURenderPassDepthStencilAttachment = {
      //   view: depthTextureView,
      //   depthLoadValue: 1,
      //   depthStoreOp: 'store',
      //   stencilLoadValue: 'load',
      //   stencilStoreOp: 'store',
      // };

      const renderPassDesc: GPURenderPassDescriptor = {
        colorAttachments: [colorAttachment],
        // depthStencilAttachment: depthAttachment,
      };

      const commandEncoder = device.createCommandEncoder();

      // 🖌️ Encode drawing commands
      const passEncoder = commandEncoder.beginRenderPass(renderPassDesc);
      passEncoder.setPipeline(pipeline);
      // passEncoder.setViewport(0, 0, canvas.width, canvas.height, 0, 1);
      // passEncoder.setScissorRect(0, 0, canvas.width, canvas.height);
      passEncoder.setVertexBuffer(0, positionBuffer);
      passEncoder.setVertexBuffer(1, colorBuffer);
      passEncoder.setBindGroup(0, uniformBindGroup);
      passEncoder.setIndexBuffer(indexBuffer, 'uint16');
      passEncoder.drawIndexed(3);
      passEncoder.endPass();

      device.queue.submit([commandEncoder.finish()]);
    };

    encodeCommands();
  }

  private createBuffer(arr: Float32Array | Uint16Array, usage: number) {
    const { device } = this;
    const buffer = device.createBuffer({
      size: (arr.byteLength + 3) & ~3,
      usage,
      mappedAtCreation: true,
    });

    const writeArray =
      arr instanceof Uint16Array
        ? new Uint16Array(buffer.getMappedRange())
        : new Float32Array(buffer.getMappedRange());

    writeArray.set(arr);
    buffer.unmap();
    return buffer;
  }

  private resizeCanvas = () => {
    const { canvas } = this;
    canvas.width = window.innerWidth * devicePixelRatio;
    canvas.height = window.innerHeight * devicePixelRatio;

    canvas.style.width = `${window.innerWidth}px`;
    canvas.style.height = `${window.innerHeight}px`;
  };
}
