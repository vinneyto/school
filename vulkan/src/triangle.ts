import * as fs from "fs";
import * as path from "path";

import {
  VulkanWindow,
  VkApplicationInfo,
  VK_STRUCTURE_TYPE_APPLICATION_INFO,
  VK_MAKE_VERSION,
  VK_API_VERSION_1_0,
  VkInstanceCreateInfo,
  VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
  VkResult,
  vkCreateInstance,
  VkInstance,
  VkLayerProperties,
  vkEnumerateInstanceLayerProperties,
  vkEnumeratePhysicalDevices,
  VkPhysicalDevice,
  VkPhysicalDeviceProperties,
  VkPhysicalDeviceFeatures,
  vkGetPhysicalDeviceProperties,
  vkGetPhysicalDeviceFeatures,
  VkPhysicalDeviceType,
  vkGetPhysicalDeviceQueueFamilyProperties,
  VkQueueFamilyProperties,
  VkQueueFlagBits,
  VkDeviceQueueCreateInfo,
  VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
  VkDeviceCreateInfo,
  VkDevice,
  vkCreateDevice,
  VkQueue,
  vkGetDeviceQueue,
  VkSurfaceKHR,
  vkGetPhysicalDeviceSurfaceSupportKHR,
  vkEnumerateDeviceExtensionProperties,
  VkExtensionProperties,
  VK_KHR_SWAPCHAIN_EXTENSION_NAME,
  VkSurfaceCapabilitiesKHR,
  VkSurfaceFormatKHR,
  vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
  vkGetPhysicalDeviceSurfaceFormatsKHR,
  vkGetPhysicalDeviceSurfacePresentModesKHR,
  VK_FORMAT_B8G8R8A8_SRGB,
  VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
  VK_PRESENT_MODE_MAILBOX_KHR,
  VK_PRESENT_MODE_FIFO_KHR,
  VkExtent2D,
  VkSwapchainCreateInfoKHR,
  VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
  VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
  VK_SHARING_MODE_CONCURRENT,
  VK_SHARING_MODE_EXCLUSIVE,
  VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
  VkSwapchainKHR,
  vkCreateSwapchainKHR,
  VkImage,
  vkGetSwapchainImagesKHR,
  VkImageView,
  VkImageViewCreateInfo,
  VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
  VK_IMAGE_VIEW_TYPE_2D,
  VkFormat,
  VK_COMPONENT_SWIZZLE_IDENTITY,
  VK_IMAGE_ASPECT_COLOR_BIT,
  vkCreateImageView,
} from "nvk/generated/1.1.126/win32";
// @ts-ignore
import { GLSL } from "nvk-essentials";

const VALIDATION_LAYERS = ["VK_LAYER_KHRONOS_validation"];

const DEVICE_EXTENSIONS = ([
  VK_KHR_SWAPCHAIN_EXTENSION_NAME,
] as unknown[]) as string[];

class QueueFamilyIndices {
  graphicsFamily?: number;
  presentFamily?: number;

  isComplete() {
    return (
      this.graphicsFamily !== undefined && this.presentFamily !== undefined
    );
  }
}

export class SwapChainSupportDetails {
  public capabilities: VkSurfaceCapabilitiesKHR;
  public formats: VkSurfaceFormatKHR[];
  public presentModes: Int32Array;

  constructor(device: VkPhysicalDevice, surface: VkSurfaceKHR) {
    this.capabilities = new VkSurfaceCapabilitiesKHR();
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
      device,
      surface,
      this.capabilities
    );

    const formatCount = { $: 0 };
    vkGetPhysicalDeviceSurfaceFormatsKHR(device, surface, formatCount, null);

    this.formats = makeArray(VkSurfaceFormatKHR, formatCount.$);
    vkGetPhysicalDeviceSurfaceFormatsKHR(
      device,
      surface,
      formatCount,
      this.formats
    );

    const presentModeCount = { $: 0 };
    vkGetPhysicalDeviceSurfacePresentModesKHR(
      device,
      surface,
      presentModeCount,
      null
    );

    this.presentModes = new Int32Array(presentModeCount.$);
    vkGetPhysicalDeviceSurfacePresentModesKHR(
      device,
      surface,
      presentModeCount,
      this.presentModes
    );
  }

  isComplete() {
    return this.presentModes.length > 0 && this.formats.length > 0;
  }

  chooseSwapSurfaceFormat() {
    for (const availableFormat of this.formats) {
      if (
        availableFormat.format == VK_FORMAT_B8G8R8A8_SRGB &&
        availableFormat.colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
      ) {
        return availableFormat;
      }
    }
    return this.formats[0];
  }

  chooseSwapPresentMode() {
    for (const availableMode of this.presentModes) {
      if (availableMode === VK_PRESENT_MODE_MAILBOX_KHR) {
        return availableMode;
      }
    }
    return VK_PRESENT_MODE_FIFO_KHR;
  }
}

class Renderer {
  win!: VulkanWindow;
  instance!: VkInstance;
  physicalDevice!: VkPhysicalDevice;
  device!: VkDevice;
  validationLayers!: string[];
  graphicsQueue!: VkQueue;
  presentQueue!: VkQueue;
  surface!: VkSurfaceKHR;
  swapChain!: VkSwapchainKHR;
  swapChainImages!: VkImage[];
  swapChainImageFormat!: VkFormat;
  swapChainExtent!: VkExtent2D;
  swapChainImageViews!: VkImageView[];

  constructor() {
    this.win = new VulkanWindow({
      width: 800,
      height: 600,
      title: "nvk triangle",
    });

    this.win.height;

    this.initValidationLayers();
    this.initInstance();
    this.initSurface();
    this.initPhysicalDevice();
    this.initLogicalDevice();
    this.initSwapChain();
    this.initImageViews();
    this.initGraphicsPipeline();
  }

  initValidationLayers() {
    const layerCount = { $: 0 };
    vkEnumerateInstanceLayerProperties(layerCount, null);

    const availableLayers = makeArray(VkLayerProperties, layerCount.$);
    vkEnumerateInstanceLayerProperties(layerCount, availableLayers);

    this.validationLayers = VALIDATION_LAYERS.filter((l) =>
      availableLayers.some((al) => al.layerName === l)
    );

    console.log(
      "availableValidationLayers =",
      availableLayers.map((l) => l.layerName)
    );

    console.log("validationLayers =", this.validationLayers);
  }

  initInstance() {
    const appInfo = createAppInfo();
    const createInfo = new VkInstanceCreateInfo();
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = appInfo;

    const instanceExtensions = this.win.getRequiredInstanceExtensions();
    createInfo.enabledExtensionCount = instanceExtensions.length;
    createInfo.ppEnabledExtensionNames = instanceExtensions;

    if (this.validationLayers.length > 0) {
      createInfo.enabledLayerCount = this.validationLayers.length;
      createInfo.ppEnabledLayerNames = this.validationLayers;
    } else {
      createInfo.enabledLayerCount = 0;
    }

    console.log("extensions", createInfo.ppEnabledExtensionNames);

    const instance = new VkInstance();

    let result: VkResult;

    result = vkCreateInstance(createInfo, null, instance);
    ASSERT_VK_RESULT(result);

    this.instance = instance;
  }

  initSurface() {
    this.surface = new VkSurfaceKHR();

    this.win.createSurface(this.instance, null, this.surface);
  }

  initPhysicalDevice() {
    const deviceCount = { $: 0 };
    vkEnumeratePhysicalDevices(this.instance, deviceCount, null);

    if (deviceCount.$ === 0) {
      throw new Error("no devices with Vulkan support");
    }

    const devices = makeArray(VkPhysicalDevice, deviceCount.$);
    vkEnumeratePhysicalDevices(this.instance, deviceCount, devices);

    let raitings = devices
      .map((d) => rateDeviceSuitability(d, this.surface))
      .map((raiting, index) => ({ raiting, index }))
      .sort((a, b) => b.raiting - a.raiting);

    console.log(raitings);

    if (raitings.length === 0 || raitings[0].raiting === 0) {
      throw new Error("failed to find a suitable GPU!");
    }

    this.physicalDevice = devices[raitings[0].index];
  }

  initLogicalDevice() {
    const indices = findQueueFamilies(this.physicalDevice, this.surface);

    const queueCreateInfos: VkDeviceQueueCreateInfo[] = [];
    const uniqueQueueFamilies = [
      indices.graphicsFamily!,
      indices.presentFamily!,
    ];

    for (const queueFamily of uniqueQueueFamilies) {
      const queueCreateInfo = new VkDeviceQueueCreateInfo();
      queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
      queueCreateInfo.queueFamilyIndex = queueFamily;
      queueCreateInfo.queueCount = 1;
      queueCreateInfo.pQueuePriorities = new Float32Array([1]);
      queueCreateInfos.push(queueCreateInfo);
    }

    const deviceFeatures = new VkPhysicalDeviceFeatures();
    const deviceCreateInfo = new VkDeviceCreateInfo();

    deviceCreateInfo.pQueueCreateInfos = queueCreateInfos;
    deviceCreateInfo.queueCreateInfoCount = queueCreateInfos.length;

    deviceCreateInfo.pEnabledFeatures = deviceFeatures;
    deviceCreateInfo.enabledExtensionCount = DEVICE_EXTENSIONS.length;
    deviceCreateInfo.ppEnabledExtensionNames = DEVICE_EXTENSIONS;

    if (this.validationLayers.length > 0) {
      deviceCreateInfo.enabledLayerCount = this.validationLayers.length;
      deviceCreateInfo.ppEnabledLayerNames = this.validationLayers;
    } else {
      deviceCreateInfo.enabledLayerCount = 0;
    }

    const device = new VkDevice();

    const result = vkCreateDevice(
      this.physicalDevice,
      deviceCreateInfo,
      null,
      device
    );
    ASSERT_VK_RESULT(result);

    this.device = device;

    //
    this.graphicsQueue = new VkQueue();
    vkGetDeviceQueue(device, indices.graphicsFamily!, 0, this.graphicsQueue);

    //
    this.presentQueue = new VkQueue();
    vkGetDeviceQueue(device, indices.presentFamily!, 0, this.presentQueue);
  }

  initSwapChain() {
    const swapChainSupport = new SwapChainSupportDetails(
      this.physicalDevice,
      this.surface
    );

    const surfaceFormat = swapChainSupport.chooseSwapSurfaceFormat();
    const presentMode = swapChainSupport.chooseSwapPresentMode();
    const extent = new VkExtent2D();

    extent.width = this.win.width;
    extent.height = this.win.height;

    let imageCount = swapChainSupport.capabilities.minImageCount + 1;

    if (
      swapChainSupport.capabilities.maxImageCount > 0 &&
      imageCount > swapChainSupport.capabilities.maxImageCount
    ) {
      imageCount = swapChainSupport.capabilities.maxImageCount;
    }

    const createInfo = new VkSwapchainCreateInfoKHR();
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.surface = this.surface;
    createInfo.minImageCount = imageCount;
    createInfo.imageFormat = surfaceFormat.format;
    createInfo.imageColorSpace = surfaceFormat.colorSpace;
    createInfo.imageExtent = extent;
    createInfo.imageArrayLayers = 1;
    createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

    const indices = findQueueFamilies(this.physicalDevice, this.surface);
    const queueFamilyIndices = [
      indices.graphicsFamily!,
      indices.presentFamily!,
    ];

    if (indices.graphicsFamily !== indices.presentFamily) {
      createInfo.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
      createInfo.queueFamilyIndexCount = 2;
      createInfo.pQueueFamilyIndices = new Uint32Array(queueFamilyIndices);
    } else {
      createInfo.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
      createInfo.queueFamilyIndexCount = 0; // Optional
      createInfo.pQueueFamilyIndices = null; // Optional
    }

    createInfo.preTransform = swapChainSupport.capabilities.currentTransform;
    createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    createInfo.presentMode = presentMode;
    createInfo.clipped = true;

    this.swapChain = new VkSwapchainKHR();

    ASSERT_VK_RESULT(
      vkCreateSwapchainKHR(this.device, createInfo, null, this.swapChain)
    );

    this.swapChainExtent = extent;
    this.swapChainImageFormat = surfaceFormat.format;
  }

  initImageViews() {
    const imageCount = { $: 0 };
    vkGetSwapchainImagesKHR(this.device, this.swapChain, imageCount, null);

    this.swapChainImages = makeArray(VkImage, imageCount.$);
    vkGetSwapchainImagesKHR(
      this.device,
      this.swapChain,
      imageCount,
      this.swapChainImages
    );

    this.swapChainImageViews = [];

    for (const image of this.swapChainImages) {
      const createInfo = new VkImageViewCreateInfo();
      createInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
      createInfo.image = image;
      createInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
      createInfo.format = this.swapChainImageFormat;
      createInfo.components!.r = VK_COMPONENT_SWIZZLE_IDENTITY;
      createInfo.components!.g = VK_COMPONENT_SWIZZLE_IDENTITY;
      createInfo.components!.b = VK_COMPONENT_SWIZZLE_IDENTITY;
      createInfo.components!.a = VK_COMPONENT_SWIZZLE_IDENTITY;
      createInfo.subresourceRange!.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
      createInfo.subresourceRange!.baseMipLevel = 0;
      createInfo.subresourceRange!.levelCount = 1;
      createInfo.subresourceRange!.baseArrayLayer = 0;
      createInfo.subresourceRange!.layerCount = 1;

      const swapChainImageView = new VkImageView();
      ASSERT_VK_RESULT(
        vkCreateImageView(this.device, createInfo, null, swapChainImageView)
      );

      this.swapChainImageViews.push(swapChainImageView);
    }
  }

  initGraphicsPipeline() {
    const vertSrc = GLSL.toSPIRVSync({
      source: fs.readFileSync(path.join(__dirname, "./shaders/triangle.vert")),
      extension: `vert`,
    }).output;

    const fragSrc = GLSL.toSPIRVSync({
      source: fs.readFileSync(path.join(__dirname, "./shaders/triangle.frag")),
      extension: `frag`,
    }).output;

    console.log(vertSrc);
    console.log(fragSrc);
  }
}

function createAppInfo() {
  const appInfo = new VkApplicationInfo();
  appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
  appInfo.pApplicationName = "Hello Triangle";
  appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
  appInfo.pEngineName = "No Engine";
  appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
  appInfo.apiVersion = VK_API_VERSION_1_0;
  return appInfo;
}

function makeArray<T>(Ctor: new () => T, count: number) {
  return [...Array(count)].map(() => new Ctor());
}

function findQueueFamilies(device: VkPhysicalDevice, surface: VkSurfaceKHR) {
  const queueFamilyCount = { $: 0 };
  vkGetPhysicalDeviceQueueFamilyProperties(device, queueFamilyCount, null);

  const queueFamilies = makeArray(VkQueueFamilyProperties, queueFamilyCount.$);
  vkGetPhysicalDeviceQueueFamilyProperties(
    device,
    queueFamilyCount,
    queueFamilies
  );

  const indices = new QueueFamilyIndices();

  let i = 0;
  for (const queueFamily of queueFamilies) {
    if (
      indices.graphicsFamily === undefined &&
      queueFamily.queueFlags & VkQueueFlagBits.VK_QUEUE_GRAPHICS_BIT
    ) {
      indices.graphicsFamily = i;
    }

    if (
      indices.presentFamily === undefined ||
      indices.presentFamily === indices.graphicsFamily
    ) {
      const presentSupport = { $: false };
      vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface, presentSupport);

      if (presentSupport.$) {
        indices.presentFamily = i;
      }
    }

    i++;
  }

  return indices;
}

function rateDeviceSuitability(
  device: VkPhysicalDevice,
  surface: VkSurfaceKHR
) {
  const properties = new VkPhysicalDeviceProperties();
  const features = new VkPhysicalDeviceFeatures();

  vkGetPhysicalDeviceProperties(device, properties);
  vkGetPhysicalDeviceFeatures(device, features);

  const queueIndices = findQueueFamilies(device, surface);
  const swapChainDetails = new SwapChainSupportDetails(device, surface);

  let score = 0;

  if (!queueIndices.isComplete()) {
    return 0;
  }

  if (!checkDeviceExtensionSupport(device)) {
    return 0;
  }

  if (!swapChainDetails.isComplete()) {
    return 0;
  }

  if (
    properties.deviceType ==
    VkPhysicalDeviceType.VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
  ) {
    score += 1000;
  }

  score += properties.limits?.maxImageDimension2D || 0;

  if (!features.geometryShader) {
    return 0;
  }

  console.log({
    deviceName: properties.deviceName,
    isDiscrete:
      properties.deviceType ===
      VkPhysicalDeviceType.VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
    geometryShader: features.geometryShader,
    maxImageDimension2D: properties.limits?.maxImageDimension2D,
    queueIndices,
  });

  return score;
}

function checkDeviceExtensionSupport(device: VkPhysicalDevice) {
  const extensionCount = { $: 0 };
  vkEnumerateDeviceExtensionProperties(device, null, extensionCount, null);

  const availableExtensions = makeArray(
    VkExtensionProperties,
    extensionCount.$
  );
  vkEnumerateDeviceExtensionProperties(
    device,
    null,
    extensionCount,
    availableExtensions
  );

  const availableExtensionNames = availableExtensions.map(
    (ext) => ext.extensionName
  );

  for (const name of DEVICE_EXTENSIONS) {
    if (!availableExtensionNames.includes(name)) {
      return false;
    }
  }

  return true;
}

function ASSERT_VK_RESULT(result: VkResult) {
  if (result !== VkResult.VK_SUCCESS)
    throw new Error(`Vulkan assertion failed!`);
}

(function () {
  const renderer = new Renderer();

  console.log("Renderer consist of =", Object.keys(renderer));

  console.log("drawing..");
  (function drawLoop() {
    if (!renderer.win.shouldClose()) setTimeout(drawLoop, 0);
    // drawFrame();
    renderer.win.pollEvents();
  })();
})();
