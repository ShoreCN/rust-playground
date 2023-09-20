fn main() {
    let chinese = "你好，世界！";
    let japanese = "こんにちは世界";
    let korean = "안녕하세요 세계";
    let regions = [chinese, japanese, korean];
    let len = regions.len();

    println!("Iterate the regions array, len = {}", len);
    for region in regions {
        println!("{}", region);
    }

    // 变量遮蔽, 改变变量的内容
    let len = 0;
    println!("Iterate over, clear regions, len = {}", len);
}
