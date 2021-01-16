import {
  VulkanWindow,
  VkApplicationInfo,
  VK_STRUCTURE_TYPE_APPLICATION_INFO,
  VK_MAKE_VERSION,
  VK_API_VERSION_1_0,
} from "nvk/generated/1.1.126/darwin";

const win = new VulkanWindow({
  width: 800,
  height: 600,
  title: "nvk triangle",
});

const appInfo = new VkApplicationInfo();
appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
appInfo.pApplicationName = "Hello Triangle";
appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
appInfo.pEngineName = "No Engine";
appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
appInfo.apiVersion = VK_API_VERSION_1_0;

console.log("drawing..");
(function drawLoop() {
  if (!win.shouldClose()) setTimeout(drawLoop, 0);
  // drawFrame();
  win.pollEvents();
})();
