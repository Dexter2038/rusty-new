/// Embedded MCU Platform
/// Single choice
/// HALs are not portable across chips
#[allow(non_camel_case_types)]
pub enum EmbeddedMCU {
    STM32,
    ESP32,
    nRF52,
    RP2040,
    None,
}

/// Runtime / Concurrency model
/// Single choice
/// Each define different startup,
/// memory layout, etc
#[allow(clippy::upper_case_acronyms)]
pub enum EmbeddedRuntime {
    RTIC,
    Embassy,
    TockOS,
    DroneOS,
    None,
}

/// Optional features / Ecosystem crates
/// Multi select
/// These are independent and just add crates + features
#[allow(non_camel_case_types)]
pub enum EmbeddedFeatures {
    defmt,
    heapless,
    embedded_graphics,
    smoltcp,
    slint,
    panic_halt,
    embedded_hal_mock,
}

/// Tools / Workflow preferences
/// Multi select
#[allow(non_camel_case_types)]
pub enum EmbeddedTools {
    cargo_embed,
    probe_rs,
    cargo_generate,
}

/// Panic handler
/// Single choice
/// Only one can be linked
#[allow(non_camel_case_types)]
pub enum EmbeddedPanic {
    panic_halt,
    panic_probe,
    panic_abort,
    custom,
}

/// Logging
/// Multi select
/// Only if they picked 'defmt'
#[allow(clippy::upper_case_acronyms)]
pub enum EmbeddedLogging {
    RTT,
    Serial,
    ITM,
}
