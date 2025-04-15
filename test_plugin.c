#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>

typedef int (*obs_module_load_t)(void);
typedef int (*obs_module_version_t)(void);
typedef const char* (*obs_module_name_t)(void);
typedef const char* (*obs_module_description_t)(void);

// Mock OBS structures and functions
typedef void* obs_source_t;
typedef void* obs_data_t;

// Mock plugin data structure
struct tarkov_price_overlay_data {
    int dummy; // Placeholder for testing
};

// Mock OBS functions
obs_source_t* obs_source_create(const char* id, const char* name, obs_data_t* settings, void* data) {
    printf("Mock: Creating source '%s' with id '%s'\n", name, id);
    return malloc(sizeof(obs_source_t));
}

void obs_source_release(obs_source_t* source) {
    printf("Mock: Releasing source\n");
    free(source);
}

obs_data_t* obs_data_create() {
    printf("Mock: Creating data object\n");
    return malloc(sizeof(obs_data_t));
}

void obs_data_release(obs_data_t* data) {
    printf("Mock: Releasing data object\n");
    free(data);
}

// Mock plugin functions
void* tarkov_price_overlay_create(obs_data_t* settings, obs_source_t* source) {
    printf("Mock: Creating tarkov-price-overlay plugin\n");
    struct tarkov_price_overlay_data* data = malloc(sizeof(struct tarkov_price_overlay_data));
    data->dummy = 42;
    return data;
}

void tarkov_price_overlay_destroy(void* data) {
    printf("Mock: Destroying tarkov-price-overlay plugin\n");
    free(data);
}

void tarkov_price_overlay_update(void* data, obs_data_t* settings) {
    printf("Mock: Updating tarkov-price-overlay plugin\n");
}

void tarkov_price_overlay_video_render(void* data) {
    printf("Mock: Rendering tarkov-price-overlay plugin\n");
}

// Test function
void test_plugin() {
    printf("Testing tarkov-price-overlay plugin...\n");
    
    // Create settings
    obs_data_t* settings = obs_data_create();
    
    // Create source
    obs_source_t* source = obs_source_create("tarkov-price-overlay", "Tarkov Price Overlay", settings, NULL);
    
    // Create plugin data
    void* plugin_data = tarkov_price_overlay_create(settings, source);
    
    // Test update
    tarkov_price_overlay_update(plugin_data, settings);
    
    // Test render
    tarkov_price_overlay_video_render(plugin_data);
    
    // Cleanup
    tarkov_price_overlay_destroy(plugin_data);
    obs_source_release(source);
    obs_data_release(settings);
    
    printf("Test completed successfully!\n");
}

int main(int argc, char *argv[]) {
    const char *plugin_path = "target/release/libtarkov-price-overlay.dylib";
    if (argc > 1) {
        plugin_path = argv[1];
    }

    printf("Loading plugin: %s\n", plugin_path);
    
    void *handle = dlopen(plugin_path, RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "Error loading plugin: %s\n", dlerror());
        return 1;
    }
    
    // Clear any existing errors
    dlerror();

    // Load module functions
    obs_module_load_t obs_module_load = (obs_module_load_t)dlsym(handle, "obs_module_load");
    const char *dlsym_error = dlerror();
    if (dlsym_error) {
        fprintf(stderr, "Error loading obs_module_load: %s\n", dlsym_error);
        dlclose(handle);
        return 1;
    }

    obs_module_version_t obs_module_ver = (obs_module_version_t)dlsym(handle, "obs_module_ver");
    dlsym_error = dlerror();
    if (dlsym_error) {
        fprintf(stderr, "Error loading obs_module_ver: %s\n", dlsym_error);
    }
    
    obs_module_name_t obs_module_name = (obs_module_name_t)dlsym(handle, "obs_module_name");
    dlsym_error = dlerror();
    if (dlsym_error) {
        fprintf(stderr, "Error loading obs_module_name: %s\n", dlsym_error);
    }
    
    obs_module_description_t obs_module_description = (obs_module_description_t)dlsym(handle, "obs_module_description");
    dlsym_error = dlerror();
    if (dlsym_error) {
        fprintf(stderr, "Error loading obs_module_description: %s\n", dlsym_error);
    }
    
    // Call module functions
    printf("Calling obs_module_load()...\n");
    int result = obs_module_load();
    printf("obs_module_load() returned: %d\n", result);
    
    if (obs_module_ver) {
        int version = obs_module_ver();
        printf("Plugin version: %d.%d.%d\n", 
               version / 10000, 
               (version % 10000) / 100, 
               version % 100);
    }
    
    if (obs_module_name) {
        printf("Plugin name: %s\n", obs_module_name());
    }
    
    if (obs_module_description) {
        printf("Plugin description: %s\n", obs_module_description());
    }
    
    // Close the library
    dlclose(handle);
    
    printf("Plugin test completed successfully!\n");
    return 0;
} 