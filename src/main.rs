fn main() {
    let chinese = "你好，世界！";
    let japanese = "こんにちは世界";
    let korean = "안녕하세요 세계";
    for region in [chinese, japanese, korean] {
        println!("{}", region);
    }
}
