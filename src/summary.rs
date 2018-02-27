use std::fmt;
use sample::Sample;

#[derive(Default)]
struct Average {
    count: u64,
    value: u64,
}

impl Average {
    fn append(&mut self, value: u64) {
        self.value += value;
        self.count += 1;
    }

    fn result(&self) -> u64 {
        self.value / self.count
    }
}

#[derive(Default)]
pub struct Summary {
    total_ram: Option<u64>,
    ram_average: Average,
    ram_max: u64,
    swap_average: Average,
    swap_max: u64,
}

impl Summary {
    pub fn apply(&mut self, sample: &Sample) {
        use std::cmp;

        let ram = sample.total - sample.free;
        let swap = sample.swap_total - sample.swap_free;

        self.ram_max = cmp::max(self.ram_max, ram);
        self.swap_max = cmp::max(self.swap_max, swap);
        self.ram_average.append(ram);
        self.swap_average.append(swap);

        if self.total_ram.is_none() {
            self.total_ram = Some(sample.total);
        }
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_total_ram = self.total_ram.map(|n| n.to_string());
        let formatted_total_ram = formatted_total_ram
            .as_ref()
            .map(|s| s.as_ref())
            .unwrap_or("N/A");

        write!(
            f,
            "Average RAM:\t{}\nAverage Swap:\t{}\nMax RAM:\t{}\nMax Swap:\t{}\nTotal RAM:\t{}",
            self.ram_average.result(),
            self.swap_average.result(),
            self.ram_max,
            self.swap_max,
            formatted_total_ram,
        )
    }
}
