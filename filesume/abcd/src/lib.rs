#[derive(Debug)]
struct Abc {
    height: u32,
    width: u32,
}

impl Abc {
    fn islargerthan(&self, other: &Abc) -> bool {
        println!("{:?}{:?}", self, other);
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn islarger() {
        let smaller = Abc {
            height: 2,
            width: 4,
        };
        let larger = Abc {
            height: 4,
            width: 6,
        };
        assert!(
            larger.islargerthan(&smaller),
            "It is not larger it is smaller "
        )
    }
    #[test]
    fn fuck() {
        assert!(true)
    }
}
