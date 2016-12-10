extern crate zip;

pub trait WriteZipInfo {
    fn write_zip_info(&mut self) -> String;
}

#[cfg(test)]
mod test {
    // Not sure how to test this yet...
    // Mostly integration-test-oriented?
}
