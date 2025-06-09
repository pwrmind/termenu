use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    error::Error,
    process::Command,
};

enum Clipboard {
    Empty,
    Copy(PathBuf),
    Cut(PathBuf),
}

struct AppState {
    current_dir: PathBuf,
    clipboard: Clipboard,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = AppState {
        current_dir: env::current_dir()?,
        clipboard: Clipboard::Empty,
    };

    loop {
        if let Err(e) = show_menu(&mut state) {
            eprintln!("Ошибка: {}", e);
            wait_for_enter();
        }
    }
}

fn show_menu(state: &mut AppState) -> Result<(), Box<dyn Error>> {
    let mut entries = vec![];

    // Добавляем родительскую директорию
    if let Some(parent) = state.current_dir.parent() {
        entries.push(("..".to_string(), true, parent.to_path_buf()));
    }

    // Читаем содержимое текущей директории
    for entry in fs::read_dir(&state.current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let is_dir = path.is_dir();
        let name = entry.file_name().to_string_lossy().to_string();
        entries.push((name, is_dir, path));
    }

    // Сортируем: сначала папки, потом файлы
    entries.sort_by(|a, b| {
        b.1.cmp(&a.1) // Сначала директории
            .then_with(|| a.0.cmp(&b.0)) // Затем по алфавиту
    });

    // Очищаем экран
    clear_screen();

    // Показываем хлебные крошки
    show_breadcrumbs(&state.current_dir);
    
    // Показываем статус буфера только если не пуст
    if !matches!(&state.clipboard, Clipboard::Empty) {
        show_clipboard_status(&state.clipboard);
    }

    // Показываем элементы меню
    for (i, (name, is_dir, _)) in entries.iter().enumerate() {
        let emoji = if *is_dir { "📁" } else { "📄" };
        println!("[{}] {} {}", i, emoji, name);
    }

    // Показываем системные команды
    println!("\nСистемные команды:");
    
    // Показываем пункт вставки только если буфер не пуст
    if !matches!(&state.clipboard, Clipboard::Empty) {
        println!("[p] Вставить из буфера");
        println!("[c] Очистить буфер");
    }
    
    println!("[n] Создать папку");
    println!("[f] Создать файл");
    println!("[q] Выход");

    // Обработка пользовательского ввода
    print!("\nВыберите пункт или команду: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    match input {
        "q" => std::process::exit(0),
        "p" => {
            if !matches!(&state.clipboard, Clipboard::Empty) {
                paste_from_clipboard(state)?;
                wait_for_enter();
            } else {
                println!("Буфер пуст!");
                wait_for_enter();
            }
        }
        "c" => {
            if !matches!(&state.clipboard, Clipboard::Empty) {
                state.clipboard = Clipboard::Empty;
                println!("Буфер очищен");
                wait_for_enter();
            } else {
                println!("Буфер уже пуст!");
                wait_for_enter();
            }
        }
        "n" => {
            create_directory(state)?;
            wait_for_enter();
        }
        "f" => {
            create_file(state)?;
            wait_for_enter();
        }
        _ => {
            if let Ok(index) = input.parse::<usize>() {
                if let Some((_, is_dir, path)) = entries.get(index) {
                    if *is_dir {
                        state.current_dir = path.clone();
                    } else {
                        file_menu(state, path)?;
                    }
                } else {
                    println!("Неверный индекс!");
                    wait_for_enter();
                }
            } else {
                println!("Неизвестная команда: {}", input);
                wait_for_enter();
            }
        }
    }

    Ok(())
}

fn file_menu(state: &mut AppState, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file_name = file_path.file_name().unwrap().to_string_lossy();

    loop {
        clear_screen();
        println!("Файл: {}", file_name);
        println!("[1] Копировать");
        println!("[2] Вырезать");
        println!("[3] Удалить");
        println!("[4] Переименовать");
        println!("[5] Открыть");
        
        print!("\nВыберите действие (b - назад): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "1" => {
                state.clipboard = Clipboard::Copy(file_path.to_path_buf());
                println!("Файл скопирован в буфер");
                wait_for_enter();
                return Ok(());
            }
            "2" => {
                state.clipboard = Clipboard::Cut(file_path.to_path_buf());
                println!("Файл вырезан");
                wait_for_enter();
                return Ok(());
            }
            "3" => {
                fs::remove_file(file_path)?;
                println!("Файл удален");
                wait_for_enter();
                return Ok(());
            }
            "4" => {
                print!("Введите новое имя файла: ");
                io::stdout().flush()?;
                
                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name)?;
                let new_name = new_name.trim();
                
                let new_path = file_path.with_file_name(new_name);
                fs::rename(file_path, &new_path)?;
                
                println!("Файл переименован");
                wait_for_enter();
                return Ok(());
            }
            "5" => {
                open_file(file_path)?;
                wait_for_enter();
            }
            "b" => return Ok(()),
            _ => {
                println!("Неверный ввод!");
                wait_for_enter();
            }
        }
    }
}

fn paste_from_clipboard(state: &mut AppState) -> Result<(), Box<dyn Error>> {
    match &state.clipboard {
        Clipboard::Copy(src) => {
            let file_name = src.file_name().unwrap();
            let dest = state.current_dir.join(file_name);
            
            // Проверяем, существует ли файл
            if dest.exists() {
                println!("Файл с таким именем уже существует!");
                return Ok(());
            }
            
            fs::copy(src, &dest)?;
            println!("Файл скопирован: {}", dest.display());
        }
        Clipboard::Cut(src) => {
            let file_name = src.file_name().unwrap();
            let dest = state.current_dir.join(file_name);
            
            // Проверяем, существует ли файл
            if dest.exists() {
                println!("Файл с таким именем уже существует!");
                return Ok(());
            }
            
            fs::rename(src, &dest)?;
            state.clipboard = Clipboard::Empty;
            println!("Файл перемещен: {}", dest.display());
        }
        Clipboard::Empty => {
            println!("Буфер обмена пуст!");
        }
    }
    Ok(())
}

fn show_clipboard_status(clipboard: &Clipboard) {
    match clipboard {
        Clipboard::Copy(path) => {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            println!("📋 Буфер: Копирование '{}'", name);
        }
        Clipboard::Cut(path) => {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            println!("📋 Буфер: Вырезание '{}'", name);
        }
        Clipboard::Empty => {}
    }
    println!();
}

fn create_directory(state: &mut AppState) -> Result<(), Box<dyn Error>> {
    clear_screen();
    println!("Создание новой папки в: {}", state.current_dir.display());
    print!("Введите имя папки: ");
    io::stdout().flush()?;
    
    let mut dir_name = String::new();
    io::stdin().read_line(&mut dir_name)?;
    let dir_name = dir_name.trim();
    
    if dir_name.is_empty() {
        println!("Имя папки не может быть пустым!");
        return Ok(());
    }
    
    let new_dir = state.current_dir.join(dir_name);
    if new_dir.exists() {
        println!("Папка уже существует!");
        return Ok(());
    }
    
    fs::create_dir(&new_dir)?;
    println!("Папка создана: {}", new_dir.display());
    Ok(())
}

fn create_file(state: &mut AppState) -> Result<(), Box<dyn Error>> {
    clear_screen();
    println!("Создание нового файла в: {}", state.current_dir.display());
    print!("Введите имя файла: ");
    io::stdout().flush()?;
    
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();
    
    if file_name.is_empty() {
        println!("Имя файла не может быть пустым!");
        return Ok(());
    }
    
    let new_file = state.current_dir.join(file_name);
    if new_file.exists() {
        println!("Файл уже существует!");
        return Ok(());
    }
    
    File::create(&new_file)?;
    println!("Файл создан: {}", new_file.display());
    Ok(())
}

fn open_file(path: &Path) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "", path.to_str().unwrap()])
        .spawn()?;

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()?;

    Ok(())
}

fn show_breadcrumbs(current_path: &Path) {
    let mut parts = vec![];
    let mut path = current_path.to_path_buf();
    
    while let Some(name) = path.file_name() {
        parts.push(name.to_string_lossy().to_string());
        path.pop();
    }
    
    parts.reverse();
    let breadcrumbs = parts.join("/");
    println!("Текущий путь: {}\n", breadcrumbs);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn wait_for_enter() {
    print!("\nНажмите Enter для продолжения...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}