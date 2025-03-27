#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]
pub mod consts;

use std::{
    collections::HashSet,
    ffi::{CStr, CString},
};

use anyhow::Result;

use ash::{Entry, Instance, vk};
use consts::*;
use log::{info, warn};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::Window;

pub struct VulkanModule {
    entry: Entry,
    instance: Instance,
    data: VulkanData,
}

impl VulkanModule {
    /// Creates a new Vulkan module for the given window
    ///
    /// # Safety
    /// This function creates Vulkan resources that must be properly managed.
    /// Caller must ensure that the window is valid for the lifetime of the returned module.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        // Unsafe call wrapped in an unsafe block
        let entry = unsafe { Entry::load()? };
        let mut data = VulkanData::default();

        // Unsafe call wrapped in an unsafe block
        let instance = unsafe { create_instance(window, &entry, &mut data)? };

        Ok(Self {
            entry,
            instance,
            data,
        })
    }
}

unsafe fn create_instance(
    window: &Window,
    entry: &Entry,
    data: &mut VulkanData,
) -> Result<Instance> {
    // Using proper c-string literals
    let app_name = unsafe { CStr::from_ptr(c"Teste".as_ptr()) };
    let engine_name = unsafe { CStr::from_ptr(c"No Engine".as_ptr()) };

    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        p_engine_name: engine_name.as_ptr(),
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };

    // Get available Vulkan layers
    let layer_properties = unsafe { entry.enumerate_instance_layer_properties()? };

    // Convert layer names to strings
    let available_layers = layer_properties
        .iter()
        .map(|l| {
            // Convert Vulkan's fixed-size array to a null-terminated string
            unsafe {
                CStr::from_ptr(l.layer_name.as_ptr())
                    .to_str()
                    .unwrap_or("invalid_layer_name")
            }
        })
        .collect::<HashSet<_>>();

    // Enable validation layers in debug mode
    let mut layers = Vec::new();
    let mut layer_names = Vec::new(); // Keep CStrings alive until instance creation

    if VALIDATION_ENABLED {
        if available_layers.contains(VALIDATION_LAYER_NAME) {
            info!("Validation layer enabled");
            let validation_layer_cstring = CString::new(VALIDATION_LAYER_NAME).unwrap();
            layer_names.push(validation_layer_cstring);
            layers.push(layer_names.last().unwrap().as_ptr());
        } else {
            warn!("Validation layer requested but not supported");
        }
    }

    // Get required window and debug extensions
    let extension_names = get_required_extensions(window);

    // Keep extension names alive for the duration of instance creation
    let extension_name_ptrs: Vec<*const i8> =
        extension_names.iter().map(|name| name.as_ptr()).collect();

    let mut info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_extension_count: extension_name_ptrs.len() as u32,
        pp_enabled_extension_names: extension_name_ptrs.as_ptr(),
        ..Default::default()
    };

    // Only set enabled layers if we have some
    if !layers.is_empty() {
        info.enabled_layer_count = layers.len() as u32;
        info.pp_enabled_layer_names = layers.as_ptr();
    }

    // Explicit unsafe block for the unsafe function call
    let instance = unsafe { entry.create_instance(&info, None)? };

    Ok(instance)
}

/// Get the required Vulkan extensions for creating a window surface
///
/// Returns a list of extensions needed to create a window surface for the given window.
/// Also adds debug extensions if validation is enabled.
fn get_required_extensions(window: &Window) -> Vec<CString> {
    let mut extensions = Vec::new();

    // Get platform-specific surface extensions from winit
    // This provides extensions to create a surface compatible with the current window system
    let window_handle = window
        .window_handle()
        .expect("Failed to get window handle")
        .as_raw();

    // Add the required extensions based on the platform
    match window_handle {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(_) => {
            extensions.push(CString::new("VK_KHR_win32_surface").unwrap());
        }
        #[cfg(target_os = "linux")]
        RawWindowHandle::Wayland(_) => {
            extensions.push(CString::new("VK_KHR_wayland_surface").unwrap());
        }
        #[cfg(target_os = "linux")]
        RawWindowHandle::Xlib(_) => {
            extensions.push(CString::new("VK_KHR_xlib_surface").unwrap());
        }
        #[cfg(target_os = "android")]
        RawWindowHandle::Android(_) => {
            extensions.push(CString::new("VK_KHR_android_surface").unwrap());
        }
        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(_) => {
            extensions.push(CString::new("VK_MVK_macos_surface").unwrap());
            // MoltenVK requires this extension
            extensions.push(CString::new("VK_KHR_portability_enumeration").unwrap());
        }
        _ => {
            warn!("Unsupported window system: {:?}", window_handle);
        }
    }

    // Surface extension is always required for displaying to a window
    extensions.push(CString::new("VK_KHR_surface").unwrap());

    // Add validation layer extensions if requested
    if VALIDATION_ENABLED {
        extensions.push(CString::new("VK_EXT_debug_utils").unwrap());
    }

    // Log the extensions we're requesting
    for extension in &extensions {
        info!("Requesting extension: {:?}", extension);
    }

    extensions
}

#[derive(Clone, Copy, Default)]
pub struct VulkanData {}
