use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terrabytes,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Unit::Bytes => "Byte",
            Unit::Kilobytes => "KB",
            Unit::Megabytes => "MB",
            Unit::Gigabytes => "GB",
            Unit::Terrabytes => "TB",
        };
        write!(f, "{}", text)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Progress {
    loaded: f64,
    total: f64,
    loaded_string: String,
    total_string: String,
    loaded_unit: Unit,
    total_unit: Unit,
}

impl Progress {
    pub fn new(loaded: f64, total: f64) -> Self {
        let (loaded_unit, loaded_string) = get_unit(loaded);
        let (total_unit, total_string) = get_unit(total);
        Self {
            loaded,
            total,
            loaded_string,
            total_string,
            loaded_unit,
            total_unit
        }
    }

    pub fn loaded(&self) -> f64 {
        self.loaded
    }

    pub fn loaded_string(&self) -> &String {
        &self.loaded_string
    }

    pub fn loaded_unit(&self) -> Unit {
        self.loaded_unit
    }

    pub fn set_loaded(&mut self, loaded: f64) {
        self.loaded = loaded;
        let (unit, loaded_string) = get_unit(loaded);
        self.loaded_unit = unit;
        self.loaded_string = loaded_string;
    }

    pub fn total(&self) -> f64 {
        self.total
    }
    
    pub fn total_string(&self) -> &String {
        &self.total_string
    }

    pub fn total_unit(&self) -> Unit {
        self.total_unit
    }

    pub fn set_total(&mut self, total: f64) {
        self.total = total;
        let (unit, total_string) = get_unit(total);
        self.total_unit = unit;
        self.total_string = total_string;
    }
}

const KILOBYTE: f64 = 1000.0;
const KILOBYTE_BORDER: f64 = 10000.0;
const MEGABYTE: f64 = 1000000.0;
const MEGABYTE_BORDER: f64 = 10000000.0;
const GIGABYTE: f64 = 1000000000.0;
const GIGABYTE_BORDER: f64 = 10000000000.0;
const TERRABYTE: f64 = 100000000000.0;
const TERRABYTE_BORDER: f64 = 1000000000000.0;

fn get_unit(value: f64) -> (Unit, String) {
    let (unit, value) = if value >= TERRABYTE_BORDER {
        (Unit::Terrabytes, (value / TERRABYTE))
    } else if value >= GIGABYTE_BORDER {
        (Unit::Gigabytes, (value / GIGABYTE))
    } else if value >= MEGABYTE_BORDER {
        (Unit::Megabytes, (value / MEGABYTE))
    } else if value >= KILOBYTE_BORDER {
        (Unit::Kilobytes, (value / KILOBYTE))
    } else {
        (Unit::Bytes, value)
    };

    let value = value.round();
    (unit, value.floor().to_string())
}


#[cfg(test)]
mod progress_test {
    use super::{get_unit, Progress, Unit};

    #[test]
    fn get_0() {
        let (unit, string) = get_unit(0.0);
        assert_eq!(unit, Unit::Bytes);
        assert_eq!(string, "0".to_string());
    }

    #[test]
    fn get_999() {
        let (unit, string) = get_unit(999.0);
        assert_eq!(unit, Unit::Bytes);
        assert_eq!(string, "999".to_string());
    }

    #[test]
    fn get_2001() {
        let (unit, string) = get_unit(20001.0);
        assert_eq!(unit, Unit::Kilobytes);
        assert_eq!(string, "20".to_string());
    }

    #[test]
    fn new_0() {
        let progress = Progress::new(0.0, 0.0);
        let reference = Progress {
            loaded: 0.0,
            total: 0.0,
            loaded_string: "0".into(),
            total_string: "0".into(),
            loaded_unit: Unit::Bytes,
            total_unit: Unit::Bytes
        };
        assert_eq!(progress, reference);
    }

    #[test]
    fn set_values() {
        let mut progress = Progress::new(0.0, 0.0);
        progress.set_loaded(50000.0);
        progress.set_total(3000000000.0);
        let reference = Progress {
            loaded: 50000.0,
            total: 3000000000.0,
            loaded_string: "50".into(),
            total_string: "3000".into(),
            loaded_unit: Unit::Kilobytes,
            total_unit: Unit::Megabytes
        };
        assert_eq!(progress, reference);
    }
}
