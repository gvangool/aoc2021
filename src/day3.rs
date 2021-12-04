use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let records: Vec<&str> = input.lines().collect();
    let record_count = records.len();
    let record_size = records[0].len();
    let mut bit_counts = vec![0; record_size];
    for record in records {
        for (i, c) in record.chars().enumerate() {
            if c == '1' {
                bit_counts[i] += 1;
            }
        }
    }

    let mut gamma_rate: usize = 0;
    let mut epsilon_rate: usize = 0;
    for count in bit_counts {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if count > (record_count / 2) {
            gamma_rate |= 1;
        } else {
            epsilon_rate |= 1;
        }
    }
    gamma_rate * epsilon_rate
}

enum Device {
    OxygenGenerator,
    Co2Scrubber,
}
fn get_device_rating(device: Device, records: &Vec<&str>, index: usize) -> i32 {
    let record_count = records.len();
    if record_count == 1 {
        return i32::from_str_radix(&records[0], 2).unwrap();
    }

    let number_of_ones = records
        .iter()
        .filter(|l| l.chars().nth(index).unwrap() == '1')
        .count();
    let number_of_zeros = record_count - number_of_ones;

    let filter_char = if number_of_ones >= number_of_zeros {
        match device {
            Device::OxygenGenerator => '1',
            Device::Co2Scrubber => '0',
        }
    } else {
        match device {
            Device::OxygenGenerator => '0',
            Device::Co2Scrubber => '1',
        }
    };

    get_device_rating(
        device,
        &records
            .iter()
            .filter(|l| l.chars().nth(index).unwrap() == filter_char)
            .cloned()
            .collect(),
        index + 1,
    )
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let records: Vec<&str> = input.lines().collect();
    let oxygen_rating = get_device_rating(Device::OxygenGenerator, &records, 0);
    let co2_rating = get_device_rating(Device::Co2Scrubber, &records, 0);
    oxygen_rating * co2_rating
}
