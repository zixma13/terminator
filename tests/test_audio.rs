/// Test that AudioCapture can be constructed (device enumeration).
#[test]
fn test_audio_capture_init() {
    // This just tests that cpal can enumerate devices without panicking.
    // Actual recording requires a real mic and is tested manually.
    let host = cpal::traits::HostTrait::default_host(&cpal::default_host());
    // Should not panic
    let _ = cpal::traits::HostTrait::default_input_device(&host);
}

/// Test base64 encoding of PCM data.
#[test]
fn test_pcm_base64_roundtrip() {
    let samples: Vec<f32> = vec![0.0, 0.5, -0.5, 1.0, -1.0];
    let bytes: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let encoded = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &bytes,
    );
    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &encoded,
    )
    .unwrap();

    let roundtrip: Vec<f32> = decoded
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();

    assert_eq!(samples, roundtrip);
}
