#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate yaxpeax_x86;

fuzz_target!(|data: &[u8]| {
    let x86_64_decoder = yaxpeax_x86::long_mode::InstDecoder::default();
    let x86_32_decoder = yaxpeax_x86::protected_mode::InstDecoder::default();
    let x86_16_decoder = yaxpeax_x86::real_mode::InstDecoder::default();

    if let Ok(inst) = x86_64_decoder.decode_slice(data) {
        let _ = inst.display_with(yaxpeax_x86::long_mode::DisplayStyle::C).to_string();
    };

    if let Ok(inst) = x86_32_decoder.decode_slice(data) {
        let _ = inst.display_with(yaxpeax_x86::protected_mode::DisplayStyle::C).to_string();
    };

    if let Ok(inst) = x86_16_decoder.decode_slice(data) {
        let _ = inst.display_with(yaxpeax_x86::real_mode::DisplayStyle::C).to_string();
    };
});
