use {
    std::fs,
    std::fs::File,
    std::path::Path,
};

pub trait FileMetaData {
    fn exists(&self) -> bool;
    fn is_readable(&self) -> bool;
    fn is_writeable(&self) -> bool;
}

impl FileMetaData for Path {
    fn exists(&self) -> bool {
        self.exists()
    }

    fn is_readable(&self) -> bool {
        File::open(self).is_ok()
    }

    fn is_writeable(&self) -> bool {
        fs::metadata(self)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use {
        crate::check_if_a_file_exists::FileMetaData,
        std::path::Path,
    };

    #[test]
    fn exists() {
        let path = Path::new("./test.txt");
        assert!(path.exists());
    }

    #[test]
    fn is_readable() {
        let path = Path::new("./test.txt");
        assert!(path.is_readable());
    }

    #[test]
    fn is_writeable() {
        let path = Path::new("./test.txt");
        assert!(path.is_writeable());
    }
}
