fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    recipe_ios::start_app();
}
