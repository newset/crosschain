mod eth {
    pub fn recover() {
        println!("123")
    }
}

#[cfg(test)]
mod tests {
    use super::eth;

    #[test]
    fn recover() {
        eth::recover()
    }
}
