use crate::prelude::*;

#[derive(Debug)]
pub struct PowerConsumption {
    pub gamma_rate:   u32,
    pub epsilon_rate: u32,
}

impl PowerConsumption {
    pub fn generate_from_measurements(measurements: &Vec<Vec<u32>>) -> Self {
        let mut positions = vec![
            vec![0u32; measurements.len()];
            measurements[0].len()
        ];
        for (n, measurement) in measurements.iter().enumerate() {
            for (m, bit) in measurement.iter().enumerate() {
                positions[m][n] = *bit;
            }
        }

        let bit_counts: Vec<BitCount> = positions.iter().map(|position_bits|
            BitCount::new(&position_bits)
        ).collect();

        let gamma_bits: Vec<u32> = bit_counts.iter().map(|bit_count|
            bit_count.max_counted_bit()
        ).collect();
        let gamma_value: u32 = number_from_bit_array(&gamma_bits);

        let epsilon_bits: Vec<u32> = bit_counts.iter().map(|bit_count|
            bit_count.min_counted_bit()
        ).collect();
        let epsilon_value: u32 = number_from_bit_array(&epsilon_bits);

        Self {
            gamma_rate: gamma_value,
            epsilon_rate: epsilon_value,
        }
    }

    pub fn value(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}
