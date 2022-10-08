#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;
    #[test]
    fn pathfile_test() {
        let path = Path::new("./foo/bar.txt");

        // 返回上级路径，若无上级路径则返回 `None`
        let parent = path.parent().unwrap();

        // 返回文件名（不包含文件扩展名）
        let file_stem = path.file_stem().unwrap();

        println!(
            "path: {:?}, parent: {:?}, file_stem: {:?}",
            path, parent, file_stem
        );

        // 创建一个空的 `PathBuf`
        let mut empty_path = PathBuf::new();
        println!("empty_path: {:?}", empty_path);

        // 根据字符串切片创建 `PathBuf`
        let path = PathBuf::from(r"C:\windows\system32.dll");

        // 添加路径
        empty_path.push(r"C:\");

        println!("empty_path: {:?}, path: {:?}", empty_path, path);
    }

    use std::fs;
    #[test]
    // 由于字符串切片实现了 `AsRef<Path>` Trait，因此函数中的参数可以直接使用字符串字面量
    fn create_test() -> std::io::Result<()> {
        // 创建一个空目录
        fs::create_dir("./empty")?;

        // 创建一个目录，若其上级目录不存在，则一同创建
        fs::create_dir_all("./some/dir")?;

        Ok(())
    }

    #[test]
    fn remove_test() -> std::io::Result<()> {
        // 删除一个空目录
        fs::remove_dir("./empty")?;
     
        // 删除指定目录及其目录下的内容，但不会删除其上级目录
        fs::remove_dir_all("./some/dir")?;
     
        Ok(())
    }
    use std::fs::File;
    #[test]
    fn create_file_test() -> std::io::Result<()> {
        // 以只写模式打开指定文件，若文件存在则清空文件内容，若文件不存在则新建一个
        let mut f = File::create("foo.txt")?;

        // 删除文件
        fs::remove_file("foo.txt")?;

        Ok(())
    }

    #[test]
    fn open_test() -> std::io::Result<()> {
        // 以只读模式打开指定文件，若文件不存在则返回错误
        let _file = File::open("foo.txt")?;
     
        Ok(())
    }

    use std::fs::OpenOptions;
    #[test]
    fn open_options_test() -> std::io::Result<()> {
        let _file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // 新建，若文件存在则打开这个文件
            .open("foo.txt")?;
     
        let _file = OpenOptions::new()
            .append(true) // 追加内容
            .open("foo.txt")?;
     
        let _file = OpenOptions::new()
            .write(true)
            .truncate(true) // 清空文件
            .open("foo.txt");
     
        Ok(())
    }

    use std::io;
    use std::io::prelude::*;
    #[test]
    fn read_test() -> io::Result<()> {
        let mut f = File::open("foo.txt")?;
        let mut buffer = [0; 10];
        // 读取文件中的前10个字节
        let n = f.read(&mut buffer[..])?;
        println!("The bytes: {:?}", &buffer[..n]);
    
        // 接着读取10个字节
        let n = f.read(&mut buffer[..])?;
        println!("The bytes: {:?}", &buffer[..n]);
    
        let mut f = File::open("foo.txt")?;
        let mut buffer = String::new();
        // 读取文件所有内容并转为字符字符串，若文件非 UTF-8 格式，则会报错
        f.read_to_string(&mut buffer)?;
        println!("The string: {}", buffer);
    
        Ok(())
    }

    #[test]
    fn write_test() -> std::io::Result<()> {
        let mut buffer = File::create("foo.txt")?;
     
        buffer.write(b"some bytes")?;
     
        buffer.write_all(b"more bytes")?;
     
        buffer.flush()?;
     
        Ok(())
    }
}
