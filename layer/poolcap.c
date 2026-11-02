// Fault-injection Vulkan layer: refuses update-after-bind descriptor pools
// once the live total of their descriptors would exceed POOLCAP_LIMIT
// (default 1048576), returning VK_ERROR_OUT_OF_DEVICE_MEMORY like a driver
// that enforces maxUpdateAfterBindDescriptorsInAllPools at pool creation.
// Optionally reports that limit too (POOLCAP_REPORT=1).
#include <vulkan/vulkan.h>
#include <vulkan/vk_layer.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define EXPORT __attribute__((visibility("default")))

static PFN_vkGetInstanceProcAddr next_gipa;
static PFN_vkGetDeviceProcAddr next_gdpa;
static PFN_vkCreateDescriptorPool next_create_pool;
static PFN_vkDestroyDescriptorPool next_destroy_pool;
static PFN_vkGetPhysicalDeviceProperties2 next_props2;
static PFN_vkGetPhysicalDeviceProperties2 next_props2_khr;

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static unsigned long long live_total;
#define MAX_POOLS 4096
static struct { VkDescriptorPool pool; unsigned long long n; } pools[MAX_POOLS];

static unsigned long long limit(void) {
    const char *s = getenv("POOLCAP_LIMIT");
    return s ? strtoull(s, NULL, 0) : 1048576ULL;
}

static VkResult VKAPI_CALL CreateDescriptorPool(VkDevice dev, const VkDescriptorPoolCreateInfo *ci,
                                                const VkAllocationCallbacks *a, VkDescriptorPool *out) {
    unsigned long long n = 0;
    int uab = (ci->flags & VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT) != 0;
    for (uint32_t i = 0; i < ci->poolSizeCount; i++) n += ci->pPoolSizes[i].descriptorCount;
    pthread_mutex_lock(&lock);
    if (uab && live_total + n > limit()) {
        fprintf(stderr, "[poolcap] REFUSED update-after-bind pool: maxSets=%u descriptors=%llu live=%llu limit=%llu\n",
                ci->maxSets, n, live_total, limit());
        pthread_mutex_unlock(&lock);
        return VK_ERROR_OUT_OF_DEVICE_MEMORY;
    }
    pthread_mutex_unlock(&lock);
    VkResult r = next_create_pool(dev, ci, a, out);
    if (r == VK_SUCCESS && uab) {
        pthread_mutex_lock(&lock);
        live_total += n;
        for (int i = 0; i < MAX_POOLS; i++)
            if (!pools[i].pool) { pools[i].pool = *out; pools[i].n = n; break; }
        fprintf(stderr, "[poolcap] update-after-bind pool: maxSets=%u descriptors=%llu live=%llu\n",
                ci->maxSets, n, live_total);
        pthread_mutex_unlock(&lock);
    }
    return r;
}

static void VKAPI_CALL DestroyDescriptorPool(VkDevice dev, VkDescriptorPool p, const VkAllocationCallbacks *a) {
    pthread_mutex_lock(&lock);
    for (int i = 0; i < MAX_POOLS; i++)
        if (p && pools[i].pool == p) { live_total -= pools[i].n; pools[i].pool = 0; break; }
    pthread_mutex_unlock(&lock);
    next_destroy_pool(dev, p, a);
}

static void patch_props(VkPhysicalDeviceProperties2 *p) {
    if (!getenv("POOLCAP_REPORT")) return;
    for (VkBaseOutStructure *s = (VkBaseOutStructure *)p->pNext; s; s = s->pNext) {
        if (s->sType == VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES)
            ((VkPhysicalDeviceDescriptorIndexingProperties *)s)->maxUpdateAfterBindDescriptorsInAllPools = (uint32_t)limit();
        if (s->sType == VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES)
            ((VkPhysicalDeviceVulkan12Properties *)s)->maxUpdateAfterBindDescriptorsInAllPools = (uint32_t)limit();
    }
}
static void VKAPI_CALL GetProps2(VkPhysicalDevice pd, VkPhysicalDeviceProperties2 *p) { next_props2(pd, p); patch_props(p); }
static void VKAPI_CALL GetProps2KHR(VkPhysicalDevice pd, VkPhysicalDeviceProperties2 *p) { next_props2_khr(pd, p); patch_props(p); }

static VkResult VKAPI_CALL CreateInstance(const VkInstanceCreateInfo *ci, const VkAllocationCallbacks *a, VkInstance *inst) {
    VkLayerInstanceCreateInfo *li = (VkLayerInstanceCreateInfo *)ci->pNext;
    while (li && !(li->sType == VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO && li->function == VK_LAYER_LINK_INFO))
        li = (VkLayerInstanceCreateInfo *)li->pNext;
    if (!li) return VK_ERROR_INITIALIZATION_FAILED;
    next_gipa = li->u.pLayerInfo->pfnNextGetInstanceProcAddr;
    li->u.pLayerInfo = li->u.pLayerInfo->pNext;
    PFN_vkCreateInstance create = (PFN_vkCreateInstance)next_gipa(NULL, "vkCreateInstance");
    VkResult r = create(ci, a, inst);
    if (r == VK_SUCCESS) {
        next_props2 = (PFN_vkGetPhysicalDeviceProperties2)next_gipa(*inst, "vkGetPhysicalDeviceProperties2");
        next_props2_khr = (PFN_vkGetPhysicalDeviceProperties2)next_gipa(*inst, "vkGetPhysicalDeviceProperties2KHR");
        fprintf(stderr, "[poolcap] active, limit=%llu report=%s\n", limit(), getenv("POOLCAP_REPORT") ? "yes" : "no");
    }
    return r;
}

static VkResult VKAPI_CALL CreateDevice(VkPhysicalDevice pd, const VkDeviceCreateInfo *ci, const VkAllocationCallbacks *a, VkDevice *dev) {
    VkLayerDeviceCreateInfo *li = (VkLayerDeviceCreateInfo *)ci->pNext;
    while (li && !(li->sType == VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO && li->function == VK_LAYER_LINK_INFO))
        li = (VkLayerDeviceCreateInfo *)li->pNext;
    if (!li) return VK_ERROR_INITIALIZATION_FAILED;
    PFN_vkGetInstanceProcAddr gipa = li->u.pLayerInfo->pfnNextGetInstanceProcAddr;
    next_gdpa = li->u.pLayerInfo->pfnNextGetDeviceProcAddr;
    li->u.pLayerInfo = li->u.pLayerInfo->pNext;
    PFN_vkCreateDevice create = (PFN_vkCreateDevice)gipa(NULL, "vkCreateDevice");
    VkResult r = create(pd, ci, a, dev);
    if (r == VK_SUCCESS) {
        next_create_pool = (PFN_vkCreateDescriptorPool)next_gdpa(*dev, "vkCreateDescriptorPool");
        next_destroy_pool = (PFN_vkDestroyDescriptorPool)next_gdpa(*dev, "vkDestroyDescriptorPool");
    }
    return r;
}

EXPORT PFN_vkVoidFunction VKAPI_CALL poolcap_GetDeviceProcAddr(VkDevice dev, const char *name) {
    if (!strcmp(name, "vkCreateDescriptorPool")) return (PFN_vkVoidFunction)CreateDescriptorPool;
    if (!strcmp(name, "vkDestroyDescriptorPool")) return (PFN_vkVoidFunction)DestroyDescriptorPool;
    if (!strcmp(name, "vkGetDeviceProcAddr")) return (PFN_vkVoidFunction)poolcap_GetDeviceProcAddr;
    return next_gdpa(dev, name);
}

EXPORT PFN_vkVoidFunction VKAPI_CALL poolcap_GetInstanceProcAddr(VkInstance inst, const char *name) {
    if (!strcmp(name, "vkCreateInstance")) return (PFN_vkVoidFunction)CreateInstance;
    if (!strcmp(name, "vkCreateDevice")) return (PFN_vkVoidFunction)CreateDevice;
    if (!strcmp(name, "vkGetInstanceProcAddr")) return (PFN_vkVoidFunction)poolcap_GetInstanceProcAddr;
    if (!strcmp(name, "vkGetDeviceProcAddr")) return (PFN_vkVoidFunction)poolcap_GetDeviceProcAddr;
    if (!strcmp(name, "vkCreateDescriptorPool")) return (PFN_vkVoidFunction)CreateDescriptorPool;
    if (!strcmp(name, "vkDestroyDescriptorPool")) return (PFN_vkVoidFunction)DestroyDescriptorPool;
    if (!strcmp(name, "vkGetPhysicalDeviceProperties2")) return (PFN_vkVoidFunction)GetProps2;
    if (!strcmp(name, "vkGetPhysicalDeviceProperties2KHR")) return (PFN_vkVoidFunction)GetProps2KHR;
    return next_gipa(inst, name);
}
