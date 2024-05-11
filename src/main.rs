/**
 * 1. 从命令行参数或者交互式地获取 folder_name 和 project_name
 * 2. 创建一个名为 folder_name 的文件夹, 将 template 文件夹中的所有文件复制到 folder_name 文件夹中
 * 3. 将 folder_name 文件夹中的 package.json 文件中的 name 字段替换为 project_name
 */

use std::env;
use std::fs;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "template"]
struct Asset;

fn main() {
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("使用方法: create-react folder_name project_name");
        return;
    }
    let folder_name: &String = &args[1];
    let project_name: &String = &args[2];

    // 当前目录
    let current_dir = env::current_dir().unwrap();
    let folder_path = format!("{}/{}", current_dir.display(), folder_name);

    // 创建文件夹
    fs::create_dir(&folder_path).unwrap();

    // 复制文件
    for file in Asset::iter() {
        let file_path = format!("{}/{}", folder_path, file.as_ref());
        let file_content = Asset::get(file.as_ref()).unwrap().data;
        // 如果是文件夹, 则创建文件夹, 如 src/components/App.tsx, 需要创建 src 和 src/components 两个文件夹
        if file.as_ref().contains("/") {
            let mut path = folder_path.clone();
            for folder in file.as_ref().split("/").collect::<Vec<&str>>().iter().take(file.as_ref().split("/").collect::<Vec<&str>>().len() - 1) {
                path = format!("{}/{}", path, folder);
                // 如果文件夹不存在, 则创建文件夹
                if !fs::metadata(&path).is_ok() {
                    fs::create_dir(&path).unwrap();
                }                
            }
        }
        // 创建文件
        fs::write(file_path, file_content).unwrap();
    }    

    // 替换 package.json 中的 name 字段
    let package_json_path = format!("{}/package.json", folder_path);
    let package_json_content = fs::read_to_string(&package_json_path).unwrap();
    let new_package_json_content = package_json_content.replace("{{project_name}}", project_name);
    fs::write(&package_json_path, new_package_json_content).unwrap();

    // 打印成功信息
    println!("\n项目创建成功, 打开项目: code ./{}\n", folder_name);
    
}