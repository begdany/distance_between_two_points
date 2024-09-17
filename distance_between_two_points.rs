// Координаты точки
struct Point {
    x: f64,
    y: f64,
}

// Функции структуры
impl Point {
    // Конструктор создания новой точки
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Метод для вычисления расстояния до другой точки
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    // Задание координат точек
    let point1 = Point::new(5.0, 0.0);
    let point2 = Point::new(0.0, 0.0);
    
    // Вычисление расстояния между точками
    let distance = point1.distance(&point2);

    // Вывод результата
    println!("Координаты 1-й точки: {}, {}", point1.x, point1.y);
    println!("Координаты 2-й точки: {}, {}", point2.x, point2.y);
    println!("Расстояние между точками: {}", distance);
}
