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
} from "nvk/generated/1.1.126/darwin";

const VALIDATION_LAYERS = ["VK_LAYER_KHRONOS_validation"];

(function () {
  const win = new VulkanWindow({
    width: 800,
    height: 600,
    title: "nvk triangle",
  });

  const validationLayers = getCurrentValidationLayers();
  const instance = createInstance(validationLayers, win);

  pickPhysicalDevice(instance);

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

function pickPhysicalDevice(instance: VkInstance) {
  const deviceCount = { $: 0 };
  vkEnumeratePhysicalDevices(instance, deviceCount, null);

  if (deviceCount.$ === 0) {
    throw new Error("no devices with Vulkan support");
  }

  const devices = [...Array(deviceCount.$)].map(() => new VkPhysicalDevice());
  vkEnumeratePhysicalDevices(instance, deviceCount, devices);

  let device = devices.map(recognizeDevice).find((d) => isDeviceSuitable(d));

  if (device === undefined) {
    throw new Error("failed to find a suitable GPU!");
  }
}

function recognizeDevice(device: VkPhysicalDevice) {
  const properties = new VkPhysicalDeviceProperties();
  const features = new VkPhysicalDeviceFeatures();

  vkGetPhysicalDeviceProperties(device, properties);
  vkGetPhysicalDeviceFeatures(device, features);

  console.log(
    properties.deviceName,
    properties.deviceType ===
      VkPhysicalDeviceType.VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
    features.geometryShader
  );

  return { properties, features };
}

function isDeviceSuitable({
  properties,
  features,
}: {
  properties: VkPhysicalDeviceProperties;
  features: VkPhysicalDeviceFeatures;
}) {
  return (
    properties.deviceType ==
      VkPhysicalDeviceType.VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU &&
    features.geometryShader
  );
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

  const availableLayers = [...Array(layerCount.$)].map(
    () => new VkLayerProperties()
  );
  vkEnumerateInstanceLayerProperties(layerCount, availableLayers);

  return availableLayers;
}
