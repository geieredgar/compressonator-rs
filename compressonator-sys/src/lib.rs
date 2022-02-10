use libc::{c_char, c_int, c_uint};

pub type Status = c_uint;
pub const OK: Status = 0;
pub const ABORTED: Status = 1;
pub const ERR_INVALID_SOURCE_TEXTURE: Status = 2;
pub const ERR_INVALID_DEST_TEXTURE: Status = 3;
pub const ERR_UNSUPPORTED_SOURCE_FORMAT: Status = 4;
pub const ERR_UNSUPPORTED_DEST_FORMAT: Status = 5;
pub const ERR_UNSUPPORTED_GPU_ASTC_DECODE: Status = 6;
pub const ERR_UNSUPPORTED_GPU_BASIS_DECODE: Status = 7;
pub const ERR_SIZE_MISMATCH: Status = 8;
pub const ERR_UNABLE_TO_INIT_CODEC: Status = 9;
pub const ERR_UNABLE_TO_INIT_DECOMPRESSLIB: Status = 10;
pub const ERR_UNABLE_TO_INIT_COMPUTELIB: Status = 11;
pub const ERR_DESTINATION: Status = 12;
pub const ERR_MEM_ALLOC_FOR_MIPSET: Status = 13;
pub const ERR_UNKNOWN_DESTINATION_FORMAT: Status = 14;
pub const ERR_FAILED_HOST_SETUP: Status = 15;
pub const ERR_PLUGIN_FILE_NOT_FOUND: Status = 16;
pub const ERR_UNABLE_TO_LOAD_FILE: Status = 17;
pub const ERR_UNABLE_TO_CREATE_ENCODER: Status = 18;
pub const ERR_UNABLE_TO_LOAD_ENCODER: Status = 19;
pub const ERR_NOSHADER_CODE_DEFINED: Status = 20;
pub const ERR_GPU_DOESNOT_SUPPORT_COMPUTE: Status = 21;
pub const ERR_NOPERFSTATS: Status = 22;
pub const ERR_GPU_DOESNOT_SUPPORT_EXT: Status = 23;
pub const ERR_GAMMA_OUTOFRANGE: Status = 24;
pub const ERR_PLUGIN_SHAREDIO_NOT_SET: Status = 25;
pub const ERR_UNABLE_TO_INIT_D3DX: Status = 26;
pub const FRAMEWORK_NOT_INITIALIZED: Status = 27;
pub const ERR_GENERIC: Status = 28;

pub type Format = c_uint;
pub const FORMAT_UNKNOWN: Format = 0;
pub const FORMAT_RGBA_8888_S: Format = 1;
pub const FORMAT_ARGB_8888_S: Format = 2;
pub const FORMAT_ARGB_8888: Format = 3;
pub const FORMAT_ABGR_8888: Format = 4;
pub const FORMAT_RGBA_8888: Format = 5;
pub const FORMAT_BGRA_8888: Format = 6;
pub const FORMAT_RGB_888: Format = 7;
pub const FORMAT_RGB_888_S: Format = 8;
pub const FORMAT_BGR_888: Format = 9;
pub const FORMAT_RG_8_S: Format = 10;
pub const FORMAT_RG_8: Format = 11;
pub const FORMAT_R_8_S: Format = 12;
pub const FORMAT_R_8: Format = 13;
pub const FORMAT_ARGB_2101010: Format = 14;
pub const FORMAT_ARGB_16: Format = 15;
pub const FORMAT_ABGR_16: Format = 16;
pub const FORMAT_RGBA_16: Format = 17;
pub const FORMAT_BGRA_16: Format = 18;
pub const FORMAT_RG_16: Format = 19;
pub const FORMAT_R_16: Format = 20;
pub const FORMAT_RGBE_32F: Format = 21;
pub const FORMAT_ARGB_16F: Format = 22;
pub const FORMAT_ABGR_16F: Format = 23;
pub const FORMAT_RGBA_16F: Format = 24;
pub const FORMAT_BGRA_16F: Format = 25;
pub const FORMAT_RG_16F: Format = 26;
pub const FORMAT_R_16F: Format = 27;
pub const FORMAT_ARGB_32F: Format = 28;
pub const FORMAT_ABGR_32F: Format = 29;
pub const FORMAT_RGBA_32F: Format = 30;
pub const FORMAT_BGRA_32F: Format = 31;
pub const FORMAT_RGB_32F: Format = 32;
pub const FORMAT_BGR_32F: Format = 33;
pub const FORMAT_RG_32F: Format = 34;
pub const FORMAT_R_32F: Format = 35;
pub const FORMAT_ASTC: Format = 36;
pub const FORMAT_ATI1N: Format = 37;
pub const FORMAT_ATI2N: Format = 38;
pub const FORMAT_ATI2N_XY: Format = 39;
pub const FORMAT_ATI2N_DXT5: Format = 40;
pub const FORMAT_ATC_RGB: Format = 41;
pub const FORMAT_ATC_RGBA_EXPLICIT: Format = 42;
pub const FORMAT_ATC_RGBA_INTERPOLATED: Format = 43;
pub const FORMAT_BC1: Format = 44;
pub const FORMAT_BC2: Format = 45;
pub const FORMAT_BC3: Format = 46;
pub const FORMAT_BC4: Format = 47;
pub const FORMAT_BC4_S: Format = 48;
pub const FORMAT_BC5: Format = 49;
pub const FORMAT_BC5_S: Format = 50;
pub const FORMAT_BC6H: Format = 51;
pub const FORMAT_BC6H_SF: Format = 52;
pub const FORMAT_BC7: Format = 53;
pub const FORMAT_DXT1: Format = 54;
pub const FORMAT_DXT3: Format = 55;
pub const FORMAT_DXT5: Format = 56;
pub const FORMAT_DXT5_XGBR: Format = 57;
pub const FORMAT_DXT5_RXBG: Format = 58;
pub const FORMAT_DXT5_RBXG: Format = 59;
pub const FORMAT_DXT5_XRBG: Format = 60;
pub const FORMAT_DXT5_RGXB: Format = 61;
pub const FORMAT_DXT5_XGXR: Format = 62;
pub const FORMAT_ETC_RGB: Format = 63;
pub const FORMAT_ETC2_RGB: Format = 64;
pub const FORMAT_ETC2_SRGB: Format = 65;
pub const FORMAT_ETC2_RGBA: Format = 66;
pub const FORMAT_ETC2_RGBA1: Format = 67;
pub const FORMAT_ETC2_SRGBA: Format = 68;
pub const FORMAT_ETC2_SRGBA1: Format = 69;
pub const FORMAT_PVRTC: Format = 70;
pub const FORMAT_GTC: Format = 71;
pub const FORMAT_BASIS: Format = 72;
pub const FORMAT_MAX: Format = 72;

pub type Speed = c_uint;
pub const SPEED_NORMAL: Speed = 0;
pub const SPEED_FAST: Speed = 1;
pub const SPEED_SUPER_FAST: Speed = 2;

pub type GpuDecode = c_uint;
pub const GPU_DECODE_OPENGL: GpuDecode = 0;
pub const GPU_DECODE_DIRECTX: GpuDecode = 1;
pub const GPU_DECODE_VULKAN: GpuDecode = 2;
pub const GPU_DECODE_INVALID: GpuDecode = 3;

pub type ComputeType = c_uint;
pub const COMPUTE_TYPE_UNKNOWN: ComputeType = 0;
pub const COMPUTE_TYPE_CPU: ComputeType = 1;
pub const COMPUTE_TYPE_HPC: ComputeType = 2;
pub const COMPUTE_TYPE_GPU_OCL: ComputeType = 3;
pub const COMPUTE_TYPE_GPU_DXC: ComputeType = 4;
pub const COMPUTE_TYPE_GPU_VLK: ComputeType = 5;
pub const COMPUTE_TYPE_GPU_HW: ComputeType = 6;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: Format,
    pub transcode_format: Format,
    pub block_height: u8,
    pub block_width: u8,
    pub block_depth: u8,
    pub data_size: u32,
    pub data: *mut u8,
    pub mip_set: *mut libc::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompressOptions {
    pub size: u32,
    pub use_refinement_steps: bool,
    pub refinement_steps: c_int,
    pub use_channel_weighting: bool,
    pub weighting_red: f32,
    pub weighting_green: f32,
    pub weighting_blue: f32,
    pub use_adaptive_weighting: bool,
    pub dxt1_use_alpha: bool,
    pub use_gpudecompress: bool,
    pub use_cgcompress: bool,
    pub alpha_threshold: u8,
    pub disable_multi_threading: bool,
    pub compression_speed: Speed,
    pub gpu_decode: GpuDecode,
    pub encode_with: ComputeType,
    pub num_threads: u32,
    pub quality: f32,
    pub restrict_colour: bool,
    pub restrict_alpha: bool,
    pub mode_mask: u32,
    pub num_cmds: ::std::os::raw::c_int,
    pub cmd_set: [AmdCmdSet; 20usize],
    pub input_defog: f32,
    pub input_exposure: f32,
    pub input_knee_low: f32,
    pub input_knee_high: f32,
    pub input_gamma: f32,
    pub cmp_level: c_int,
    pub pos_bits: c_int,
    pub tex_cbits: c_int,
    pub normal_bits: c_int,
    pub generic_bits: c_int,
    pub source_format: Format,
    pub dest_format: Format,
    pub format_support_host_encoder: bool,
    pub print_info_str: PrintInfoStr,
    pub get_perf_stats: bool,
    pub perf_stats: KernelPerformanceStats,
    pub get_device_info: bool,
    pub device_info: KernelDeviceInfo,
    pub gen_gpumip_maps: bool,
    pub use_srgbframes: bool,
    pub miplevels: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AmdCmdSet {
    pub str_command: [c_char; 32usize],
    pub str_parameter: [c_char; 16usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernelPerformanceStats {
    pub compute_shader_elapsed_ms: f32,
    pub num_blocks: c_int,
    pub cmp_mtx_per_sec: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KernelDeviceInfo {
    pub device_name: [c_char; 256usize],
    pub version: [c_char; 128usize],
    pub max_ucores: c_int,
}

pub type PrintInfoStr = ::std::option::Option<unsafe extern "C" fn(InfoStr: *const c_char)>;

pub type FeedbackProc = ::std::option::Option<
    unsafe extern "C" fn(fProgress: f32, pUser1: *mut i32, pUser2: *mut i32) -> bool,
>;

extern "C" {
    #[link_name = "CMP_ConvertTexture"]
    pub fn convert_texture(
        source_texture: *mut Texture,
        dest_texture: *mut Texture,
        options: *const CompressOptions,
        feedback_proc: FeedbackProc,
    ) -> Status;

    #[link_name = "CMP_CalculateBufferSize"]
    pub fn calculate_buffer_size(texture: *const Texture) -> u32;
}
