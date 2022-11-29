#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <vulkan/vulkan.h>

const uint32_t WIDTH = 800;
const uint32_t HEIGHT = 600;
const char* APPNAME = "Apps";

const uint32_t numValidationLayers = 1;
const char* validationLayers[] = {"VK_LAYER_KHRONOS_validation"};

#ifdef NDEBUG
const bool enableValidationLayers = false;
#else
const bool enableValidationLayers = true;
#endif

bool checkValidationLayerSupport();

int main()
{
  GLFWwindow* window;
  VkInstance instance;
  
  /* init vulkan */
  glfwInit();
  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);
  
  window = glfwCreateWindow(WIDTH, HEIGHT, APPNAME , NULL, NULL);

  /* create instance */
  VkApplicationInfo appInfo;
  appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
  appInfo.pApplicationName = APPNAME;
  appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
  appInfo.pEngineName = "No Engine";
  appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
  appInfo.apiVersion = VK_API_VERSION_1_0;


  uint32_t glfwExtensionCount = 0;
  const char** glfwExtensions;
  glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

  VkInstanceCreateInfo createInfo;
  createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
  createInfo.pApplicationInfo = &appInfo;
  createInfo.enabledExtensionCount = glfwExtensionCount;
  createInfo.ppEnabledExtensionNames = glfwExtensions;
  createInfo.enabledLayerCount = 0;
  createInfo.pNext = NULL;

  bool addValidationLayers = enableValidationLayers && checkValidationLayerSupport();
  createInfo.enabledLayerCount = addValidationLayers? numValidationLayers : 0;
  createInfo.ppEnabledLayerNames = addValidationLayers? validationLayers : NULL;
  
  VkResult result = vkCreateInstance(&createInfo, NULL, &instance);
  if (result != VK_SUCCESS) {
    return 1;
  }

  uint32_t extensionCount = 0;
  vkEnumerateInstanceExtensionProperties(NULL, &extensionCount, NULL);
  VkExtensionProperties* extensions = calloc(extensionCount, sizeof(VkExtensionProperties));
  vkEnumerateInstanceExtensionProperties(NULL, &extensionCount, extensions);
  printf("Available Extensions: \n");
  for (int i = 0;  i < extensionCount; i++) {
    printf("\t%s\n", extensions[i].extensionName);
  }
  
  /* Drawing Loop */
  while (!glfwWindowShouldClose(window)) {
    glfwPollEvents();
  }

  /* Clean Up Code */
  free(extensions);
  vkDestroyInstance(instance, NULL);
  glfwDestroyWindow(window);
  glfwTerminate();
  return 0;
}
bool checkValidationLayerSupport()
{
  uint32_t layerCount;
  vkEnumerateInstanceLayerProperties(&layerCount, NULL);
  VkLayerProperties* availableLayers = calloc(layerCount, sizeof(VkLayerProperties));
  vkEnumerateInstanceLayerProperties(&layerCount, availableLayers);

  bool foundAll = true;
  for (int i = 0; i < numValidationLayers; i++) {
    bool layerFound = false;
    for (int j = 0; j < layerCount && !layerFound; j++) {
      layerFound = strcmp(validationLayers[i], availableLayers[j].layerName) == 0;
    }
    if (!layerFound) {
      printf("Requested Validation Layer Missing (%s)\n", validationLayers[i]);
      foundAll = false;
    }
  }
  return foundAll;
}
