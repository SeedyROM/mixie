use std::ops::Deref;

use num::Float;

pub struct Frequency<T: ?Sized>(T);

impl<T> Frequency<T>
where
    T: Float,
{
    pub fn period_in_seconds(&self, sample_rate: T) -> T {
        sample_rate / self.0 / T::from(1000.0).unwrap()
    }

    pub fn period_in_milliseconds(&self, sample_rate: T) -> T {
        sample_rate / self.0
    }
}

impl<T> Deref for Frequency<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ToFrequency {
    type Sample;

    fn hz(&self) -> Frequency<Self::Sample>;
    fn khz(&self) -> Frequency<Self::Sample>;
}

impl ToFrequency for f32 {
    type Sample = f32;

    fn hz(&self) -> Frequency<Self::Sample> {
        Frequency(*self)
    }

    fn khz(&self) -> Frequency<Self::Sample> {
        Frequency(*self * 1000.0)
    }
}

impl ToFrequency for i32 {
    type Sample = f32;

    fn hz(&self) -> Frequency<Self::Sample> {
        Frequency(*self as Self::Sample)
    }

    fn khz(&self) -> Frequency<Self::Sample> {
        Frequency((*self * 1000) as Self::Sample)
    }
}

pub struct Duration<T: ?Sized>(T);

impl<T> Duration<T>
where
    T: Float,
{
    pub fn into_samples(&self, sample_rate: T) -> T {
        sample_rate * self.0
    }
}

impl<T> Deref for Duration<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ToDuration {
    type Duration;

    fn milliseconds(&self) -> Duration<Self::Duration>;
    fn seconds(&self) -> Duration<Self::Duration>;
}

impl ToDuration for f32 {
    type Duration = f32;

    fn milliseconds(&self) -> Duration<Self::Duration> {
        Duration(*self / 1000.0)
    }

    fn seconds(&self) -> Duration<Self::Duration> {
        Duration(*self)
    }
}

impl ToDuration for i32 {
    type Duration = f32;

    fn milliseconds(&self) -> Duration<Self::Duration> {
        Duration(*self as Self::Duration / 1000.0)
    }

    fn seconds(&self) -> Duration<Self::Duration> {
        Duration(*self as Self::Duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_frequency() {
        let freq = 440.hz();
        assert_eq!(*freq, 440.0);

        let freq = 3.3.khz();
        assert_eq!(*freq, 3_300.0);
    }

    #[test]
    fn test_frequency_to_samples() {
        let sample_rate = 44_100.0;
        let freq = 2.khz();

        let period = freq.period_in_milliseconds(sample_rate);

        assert_eq!(period, 22.05);
    }

    #[test]
    fn test_duration() {
        let duration = 10.seconds();
        assert_eq!(*duration, 10.0);

        let duration = 10.milliseconds();
        assert_eq!(*duration, 0.01);
    }

    #[test]
    fn test_duration_in_samples() {
        let duration = 10.seconds();
        let samples = duration.into_samples(44_100.0);

        assert_eq!(samples, 44_100.0 * 10.0);
    }
}
