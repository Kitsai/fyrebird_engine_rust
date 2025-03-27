// Vulkan constants
pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

// Validation layer name as a static C string
pub const VALIDATION_LAYER_NAME: &str = "VK_LAYER_KHRONOS_validation";
