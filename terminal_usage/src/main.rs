use std::io::{self};
use std::time::{Duration, Instant};
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    event::{poll, read, Event, KeyCode},
    style::Print,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Вход в альтернативный экран терминала
    execute!(stdout, EnterAlternateScreen, Hide)?;
    // Очистить экран
    execute!(stdout, Clear(ClearType::All))?;

    let mut x_position = 0;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(200); // Скорость обновления - 100 мс

    loop {
        // Обработка события нажатия клавиши для выхода
        if poll(Duration::from_millis(0))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Обновление и рендеринг на каждый тик
        let now = Instant::now();
        if now.duration_since(last_tick) >= tick_rate {
            last_tick = now;
            x_position += 1; // Смещаем блок вправо

            // Очистить экран и переместить блок
            execute!(stdout, Clear(ClearType::All), MoveTo(x_position, 10), Print("Hello, crossterm!"))?;
        }
    }

    // Возвращение к стандартному режиму экрана и показ курсора
    execute!(stdout, LeaveAlternateScreen, Show)?;
    Ok(())
}
