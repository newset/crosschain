mod ton {
    pub fn recover() {}
}

#[cfg(test)]
mod tests {
    use super::ton;

    #[test]
    fn recover() {
        eth::recover()
    }
}
