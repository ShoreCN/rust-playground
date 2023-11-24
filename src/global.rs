use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

// 从Cargo.toml中读取配置内容
lazy_static! {
    static ref DEPENDENCYIES_CONFIG: HashMap<String, String> = {
        let mut config = HashMap::new();
        // 打开文件, 读取Cargo.toml配置中的依赖库和版本号
        // 所需内容包含在[dependencies]中
        let content = fs::read_to_string("Cargo.toml").expect("read Cargo.toml failed");
        // 查找是否包含[dependencies]字段
        let mut dependencies_index = content.find("[dependencies]").expect("not found [dependencies] in Cargo.toml");
        // 从[dependencies]字段开始查找, 查找下一个[字符, 该字符之前的内容就是依赖库和版本号, 如果不存在则end_index就是文件末尾
        dependencies_index += "[dependencies]".len();
        let end_index = content[dependencies_index..].find("[").unwrap_or(content.len());
        // 截取[dependencies]字段中的内容
        let dependencies = &content[dependencies_index..end_index];
        println!("dependencies_index = {dependencies_index} end_index = {end_index}"); 
        println!("original dependencies content = {}", dependencies);
        // 依赖库和版本号的格式为: 依赖库 = 版本号
        // 通过换行符分割字符串, 得到每一行的内容
        // 遍历每一行内容, 通过=分割字符串, 得到依赖库和版本号
        for line in dependencies.split("\n") {
            let line = line.trim();
            // 跳过空行
            if line.is_empty() {
                continue;
            }
            // 跳过注释行
            if line.starts_with("#") {
                continue;
            }
            // 通过=分割字符串, 得到依赖库和版本号
            let kv: Vec<&str> = line.split("=").collect();
            // 跳过格式错误的行
            if kv.len() != 2 {
                continue;
            }
            // 依赖库和版本号都是字符串, 所以去掉两边的空格
            let key = kv[0].trim();
            let value = kv[1].trim();
            // 依赖库和版本号都是字符串, 所以去掉两边的双引号
            let key = key.trim_matches('"');
            let value = value.trim_matches('"');
            // 保存到HashMap中
            config.insert(key.to_string(), value.to_string());
        }
        println!("config = {:?}", config);

        config
    };
}

pub fn global() {
    println!("read global config result is HASHMAP: {:?}", *DEPENDENCYIES_CONFIG);
}
