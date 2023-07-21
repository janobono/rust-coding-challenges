pub enum Scale {
    Celsius,
    Fahrenheit,
}

pub struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    pub fn new(degrees: f32, scale: Scale) -> Temperature {
        Temperature {
            degrees,
            scale,
        }
    }

    pub fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32.0) * 5.0 / 9.0,
        }
    }

    pub fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * 9.0 / 5.0 + 32.0,
            Scale::Fahrenheit => self.degrees,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::convert_between_celsius_and_fahrenheit::*;

    #[test]
    fn to_celsius_works() {
        let temperature = Temperature::new(86.0, Scale::Fahrenheit);
        assert_eq!(temperature.to_celsius(), 30.0);
        assert_eq!(temperature.to_fahrenheit(), 86.0);
    }

    #[test]
    fn to_fahrenheit_works() {
        let temperature = Temperature::new(30.0, Scale::Celsius);
        assert_eq!(temperature.to_fahrenheit(), 86.0);
        assert_eq!(temperature.to_celsius(), 30.0);
    }
}
