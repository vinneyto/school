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
} from "nvk/generated/1.1.126/win32";

const VALIDATION_LAYERS = ["VK_LAYER_KHRONOS_validation"];

(function () {
  const win = new VulkanWindow({
    width: 800,
    height: 600,
    title: "nvk triangle",
  });

  const validationLayers = getCurrentValidationLayers();
  const instance = createInstance(validationLayers, win);
  const physicaDevice = pickPhysicalDevice(instance);
  const device = createLogicalDevice(physicaDevice, validationLayers);

  console.log("drawing..");
  (function drawLoop() {
    if (!win.shouldClose()) setTimeout(drawLoop, 0);
    // drawFrame();
    win.pollEvents();
  })();
})();

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

function createInstance(validationLayers: string[], win: VulkanWindow) {
  const appInfo = createAppInfo();
  const createInfo = new VkInstanceCreateInfo();
  createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
  createInfo.pApplicationInfo = appInfo;

  const instanceExtensions = win.getRequiredInstanceExtensions();
  createInfo.enabledExtensionCount = instanceExtensions.length;
  createInfo.ppEnabledExtensionNames = instanceExtensions;

  if (validationLayers.length > 0) {
    createInfo.enabledLayerCount = validationLayers.length;
    createInfo.ppEnabledLayerNames = validationLayers;
  } else {
    createInfo.enabledLayerCount = 0;
  }

  console.log("extensions", createInfo.ppEnabledExtensionNames);
  console.log("layers", createInfo.ppEnabledLayerNames);

  const instance = new VkInstance();

  let result: VkResult;

  result = vkCreateInstance(createInfo, null, instance);
  ASSERT_VK_RESULT(result);

  return instance;
}

function makeArray<T>(Ctor: new () => T, count: number) {
  return [...Array(count)].map(() => new Ctor());
}

function pickPhysicalDevice(instance: VkInstance) {
  const deviceCount = { $: 0 };
  vkEnumeratePhysicalDevices(instance, deviceCount, null);

  if (deviceCount.$ === 0) {
    throw new Error("no devices with Vulkan support");
  }

  const devices = makeArray(VkPhysicalDevice, deviceCount.$);
  vkEnumeratePhysicalDevices(instance, deviceCount, devices);

  let raitings = devices
    .map(rateDeviceSuitability)
    .map((raiting, index) => ({ raiting, index }))
    .sort((a, b) => b.raiting - a.raiting);

  if (raitings.length === 0 || raitings[0].raiting === 0) {
    throw new Error("failed to find a suitable GPU!");
  }

  return devices[raitings[0].index];
}

interface QueueFamilyIndices {
  graphicsFamily?: number;
}

function findQueueFamilies(device: VkPhysicalDevice) {
  const queueFamilyCount = { $: 0 };
  vkGetPhysicalDeviceQueueFamilyProperties(device, queueFamilyCount, null);

  const queueFamilies = makeArray(VkQueueFamilyProperties, queueFamilyCount.$);
  vkGetPhysicalDeviceQueueFamilyProperties(
    device,
    queueFamilyCount,
    queueFamilies
  );

  const indices: QueueFamilyIndices = {};

  let i = 0;
  for (const queueFamily of queueFamilies) {
    if (queueFamily.queueFlags & VkQueueFlagBits.VK_QUEUE_GRAPHICS_BIT) {
      indices.graphicsFamily = i;
    }

    i++;
  }

  return indices;
}

function rateDeviceSuitability(device: VkPhysicalDevice) {
  const properties = new VkPhysicalDeviceProperties();
  const features = new VkPhysicalDeviceFeatures();

  vkGetPhysicalDeviceProperties(device, properties);
  vkGetPhysicalDeviceFeatures(device, features);

  const queueIndices = findQueueFamilies(device);

  console.log({
    deviceName: properties.deviceName,
    isDiscrete:
      properties.deviceType ===
      VkPhysicalDeviceType.VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
    geometryShader: features.geometryShader,
    maxImageDimension2D: properties.limits?.maxImageDimension2D,
    queueIndices,
  });

  let score = 0;

  if (queueIndices.graphicsFamily === undefined) {
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

  return score;
}

function createLogicalDevice(
  physicalDevice: VkPhysicalDevice,
  layers: string[]
) {
  const indices = findQueueFamilies(physicalDevice);

  const queueCreateInfo = new VkDeviceQueueCreateInfo();
  queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
  queueCreateInfo.queueFamilyIndex = indices.graphicsFamily as number;
  queueCreateInfo.queueCount = 1;
  queueCreateInfo.pQueuePriorities = new Float32Array([1]);

  const deviceFeatures = new VkPhysicalDeviceFeatures();
  const deviceCreateInfo = new VkDeviceCreateInfo();

  deviceCreateInfo.pQueueCreateInfos = [queueCreateInfo];
  deviceCreateInfo.queueCreateInfoCount = 1;

  deviceCreateInfo.pEnabledFeatures = deviceFeatures;
  deviceCreateInfo.enabledExtensionCount = 0;

  if (layers.length > 0) {
    deviceCreateInfo.enabledLayerCount = layers.length;
    deviceCreateInfo.ppEnabledLayerNames = layers;
  } else {
    deviceCreateInfo.enabledLayerCount = 0;
  }

  const device = new VkDevice();

  const result = vkCreateDevice(physicalDevice, deviceCreateInfo, null, device);
  ASSERT_VK_RESULT(result);

  return device;
}

function ASSERT_VK_RESULT(result: VkResult) {
  if (result !== VkResult.VK_SUCCESS)
    throw new Error(`Vulkan assertion failed!`);
}

function getCurrentValidationLayers() {
  const availableValidationLayers = getAvailableValidationLayers();
  return VALIDATION_LAYERS.filter((l) =>
    availableValidationLayers.some((al) => al.layerName === l)
  );
}

function getAvailableValidationLayers() {
  const layerCount = { $: 0 };
  vkEnumerateInstanceLayerProperties(layerCount, null);

  const availableLayers = makeArray(VkLayerProperties, layerCount.$);
  vkEnumerateInstanceLayerProperties(layerCount, availableLayers);

  return availableLayers;
}
