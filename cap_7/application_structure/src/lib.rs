pub mod frontend {
    use backend::fetch_data;
    
    pub fn get_users() -> Vec<String> {
        fetch_data::get_users()
    }
}