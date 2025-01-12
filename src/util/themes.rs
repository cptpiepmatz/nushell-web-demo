#[derive(Debug, Clone, Default)]
pub struct Themed<T> {
    pub light: T,
    pub dark: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl<T> Themed<T> {
    pub fn pair(initial: T) -> Self
    where
        T: Clone,
    {
        Self { light: initial.clone(), dark: initial }
    }

    pub fn update(&mut self, f: impl Fn(&mut T, Theme)) {
        f(&mut self.light, Theme::Light);
        f(&mut self.dark, Theme::Dark);
    }

    pub fn map<U>(self, f: impl Fn(T, Theme) -> U) -> Themed<U> {
        Themed { light: f(self.light, Theme::Light), dark: f(self.dark, Theme::Dark) }
    }
}
