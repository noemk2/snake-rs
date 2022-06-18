use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use crate::{game::game::Game, snake::direction::Direction};

impl Game {
    pub fn draw_borders(&mut self) {
        self.stdout
            .execute(SetForegroundColor(Color::DarkGrey))
            .unwrap();

        for y in 0..self.height + 2 {
            self.stdout
                .execute(MoveTo(0, y))
                .unwrap()
                .execute(Print("#"))
                .unwrap()
                .execute(MoveTo(self.width + 1, y))
                .unwrap()
                .execute(Print("#"))
                .unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, 0))
                .unwrap()
                .execute(Print("#"))
                .unwrap()
                .execute(MoveTo(x, self.height + 1))
                .unwrap()
                .execute(Print("#"))
                .unwrap();
        }

        self.stdout
            .execute(MoveTo(0, 0))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(self.width + 1, self.height + 1))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(self.width + 1, 0))
            .unwrap()
            .execute(Print("#"))
            .unwrap()
            .execute(MoveTo(0, self.height + 1))
            .unwrap()
            .execute(Print("#"))
            .unwrap();
    }

    pub fn draw_background(&mut self) {
        self.stdout.execute(ResetColor).unwrap();

        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y))
                    .unwrap()
                    .execute(Print(" "))
                    .unwrap();
            }
        }
    }

    pub fn draw_food(&mut self) {
        self.stdout
            .execute(SetForegroundColor(Color::White))
            .unwrap();

        for food in self.food.iter() {
            self.stdout
                .execute(MoveTo(food.x + 1, food.y + 1))
                .unwrap()
                .execute(Print("•"))
                .unwrap();
        }
    }

    pub fn draw_snake(&mut self) {
        let fg = SetForegroundColor(match self.speed % 3 {
            0 => Color::Green,
            1 => Color::Cyan,
            _ => Color::Yellow,
        });
        self.stdout.execute(fg).unwrap();

        let body_points = self.snake.get_body_points();
        for (i, body) in body_points.iter().enumerate() {
            let previous = if i == 0 { None } else { body_points.get(i - 1) };
            let next = body_points.get(i + 1);
            let symbol = if let Some(&next) = next {
                if let Some(&previous) = previous {
                    if previous.x == next.x {
                        '║'
                    } else if previous.y == next.y {
                        '═'
                    } else {
                        let d = body.transform(Direction::Down, 1);
                        let r = body.transform(Direction::Right, 1);
                        let u = if body.y == 0 {
                            body.clone()
                        } else {
                            body.transform(Direction::Up, 1)
                        };
                        let l = if body.x == 0 {
                            body.clone()
                        } else {
                            body.transform(Direction::Left, 1)
                        };
                        if (next == d && previous == r) || (previous == d && next == r) {
                            '╔'
                        } else if (next == d && previous == l) || (previous == d && next == l) {
                            '╗'
                        } else if (next == u && previous == r) || (previous == u && next == r) {
                            '╚'
                        } else {
                            '╝'
                        }
                    }
                } else {
                    'O'
                }
            } else if let Some(&previous) = previous {
                if body.y == previous.y {
                    '═'
                } else {
                    '║'
                }
            } else {
                panic!("Invalid snake body point.");
            };

            self.stdout
                .execute(MoveTo(body.x + 1, body.y + 1))
                .unwrap()
                .execute(Print(symbol))
                .unwrap();
        }
    }
}
