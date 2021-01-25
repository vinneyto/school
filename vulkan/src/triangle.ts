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
  VkPresentModeKHR,
  VkSurfaceFormatKHR,
  vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
  vkGetPhysicalDeviceSurfaceFormatsKHR,
  vkGetPhysicalDeviceSurfacePresentModesKHR,
} from "nvk/generated/1.1.126/win32";

const VALIDATION_LAYERS = ["VK_LAYER_LUNARG_standard_validation"];

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

  constructor() {
    this.win = new VulkanWindow({
      width: 800,
      height: 600,
      title: "nvk triangle",
    });

    this.initValidationLayers();
    this.initInstance();
    this.initSurface();
    this.initPhysicalDevice();
    this.initLogicalDevice();
  }

  initValidationLayers() {
    const layerCount = { $: 0 };
    vkEnumerateInstanceLayerProperties(layerCount, null);

    const availableLayers = makeArray(VkLayerProperties, layerCount.$);
    vkEnumerateInstanceLayerProperties(layerCount, availableLayers);

    this.validationLayers = VALIDATION_LAYERS.filter((l) =>
      availableLayers.some((al) => al.layerName === l)
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

      console.log(createInfo.ppEnabledLayerNames);
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

    const presentSupport = { $: false };
    vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface, presentSupport);

    if (
      presentSupport.$ &&
      indices.presentFamily === undefined &&
      indices.graphicsFamily !== i
    ) {
      indices.presentFamily = i;
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
    swapChainDetails,
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
