use sha3::{Digest, Sha3_256};
use hound::{WavReader, WavSpec};
use rustfft::{FftPlanner, num_complex::Complex};
use thiserror::Error; // For custom error handling

/// Custom error type for audio analysis
#[derive(Error, Debug)]
pub enum AudioAnalysisError {
    #[error("Failed to decode audio data")]
    DecodeError(#[from] hound::Error),
    #[error("Invalid audio format")]
    InvalidFormat,
    #[error("Feature extraction failed")]
    FeatureExtractionError,
}

/// Solves an audio analysis puzzle by calculating features and hashing the result.
pub fn solve(audio_data: &[u8], difficulty: u8) -> Result<Vec<u8>, AudioAnalysisError> {
    // 1. Decode audio data (assuming WAV format)
    let cursor = std::io::Cursor::new(audio_data);
    let reader = WavReader::new(cursor)?;
    let spec = reader.spec();

    // Ensure the audio format is supported (16-bit PCM, mono)
    if spec.bits_per_sample != 16 || spec.channels != 1 {
        return Err(AudioAnalysisError::InvalidFormat);
    }

    // Convert samples to f32
    let samples: Vec<f32> = reader.into_samples::<i16>()
        .map(|s| s.map(|x| x as f32 / 32768.0)) // Normalize to [-1.0, 1.0]
        .collect::<Result<_, _>>()?;

    // 2. Feature extraction
    let features = extract_features(&samples, spec.sample_rate as usize, difficulty)?;

    // 3. Hash the features
    let mut hasher = Sha3_256::new();
    for feature in features {
        hasher.update(feature.to_be_bytes());
    }
    let hash = hasher.finalize().to_vec();

    Ok(hash)
}

/// Extracts audio features based on difficulty level.
fn extract_features(
    samples: &[f32],
    sample_rate: usize,
    difficulty: u8,
) -> Result<Vec<f32>, AudioAnalysisError> {
    let mut features = Vec::new();

    // Calculate spectral centroid (always included)
    features.push(calculate_spectral_centroid(samples, sample_rate)?);

    // Add more features based on difficulty
    if difficulty > 1 {
        features.push(calculate_zero_crossing_rate(samples)?);
    }
    if difficulty > 2 {
        // Add MFCCs or other advanced features
        features.extend(calculate_mfccs(samples, sample_rate)?);
    }

    Ok(features)
}

/// Calculates the spectral centroid of an audio signal.
fn calculate_spectral_centroid(samples: &[f32], sample_rate: usize) -> Result<f32, AudioAnalysisError> {
    const WINDOW_SIZE: usize = 1024; // Use a fixed window size for FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);

    let mut weighted_sum = 0.0;
    let mut magnitude_sum = 0.0;
    let mut window_count = 0;

    // Process the audio in overlapping windows
    for window in samples.chunks(WINDOW_SIZE) {
        if window.len() < WINDOW_SIZE {
            break; // Skip incomplete windows
        }

        // Convert window to complex numbers
        let mut fft_input: Vec<Complex<f32>> = window.iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect();

        // Perform FFT
        let mut fft_output = vec![Complex::new(0.0, 0.0); WINDOW_SIZE];
        fft.process(&mut fft_input, &mut fft_output);

        // Calculate spectral centroid for this window
        for (i, &bin) in fft_output.iter().enumerate().take(WINDOW_SIZE / 2) {
            let frequency = i as f32 * sample_rate as f32 / WINDOW_SIZE as f32;
            let magnitude = bin.norm();
            weighted_sum += frequency * magnitude;
            magnitude_sum += magnitude;
        }

        window_count += 1;
    }

    if magnitude_sum > 0.0 && window_count > 0 {
        // Average the spectral centroid over all windows
        Ok(weighted_sum / magnitude_sum / window_count as f32)
    } else {
        Ok(0.0)
    }
}

/// Calculates the zero-crossing rate of an audio signal.
fn calculate_zero_crossing_rate(samples: &[f32]) -> Result<f32, AudioAnalysisError> {
    let mut zero_crossings = 0;
    for i in 1..samples.len() {
        if (samples[i - 1] > 0.0 && samples[i] <= 0.0)
            || (samples[i - 1] <= 0.0 && samples[i] > 0.0)
        {
            zero_crossings += 1;
        }
    }
    Ok(zero_crossings as f32 / samples.len() as f32)
}

/// Calculates MFCCs (Mel-frequency cepstral coefficients) of an audio signal.
fn calculate_mfccs(samples: &[f32], sample_rate: usize) -> Result<Vec<f32>, AudioAnalysisError> {
    // Placeholder: Implement MFCC calculation
    // This requires additional dependencies (e.g., `rust-mfcc`) and is more complex.
    Ok(vec![0.0; 13]) // Return 13 MFCCs as a placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_valid_audio() {
        // Load a test WAV file (replace with actual test data)
        let audio_data = include_bytes!("test_audio.wav");
        let result = solve(audio_data, 1); // Test with difficulty 1
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // SHA3-256 output is 32 bytes
    }

    #[test]
    fn test_solve_invalid_format() {
        // Test with invalid audio format (e.g., stereo or non-16-bit)
        // Replace with actual invalid audio data
        let audio_data = vec![0; 1024];
        let result = solve(&audio_data, 1); // Test with difficulty 1
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_higher_difficulty() {
        // Test with higher difficulty levels
        let audio_data = include_bytes!("test_audio.wav");
        let result = solve(audio_data, 3); // Test with difficulty 3
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // SHA3-256 output is 32 bytes
    }
}