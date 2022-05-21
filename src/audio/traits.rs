use std::{iter::Zip, slice::IterMut};

// TODO: Come up with a clearer name for this
pub trait FramedSampleTrait<'a> {
    fn samples(&mut self) -> IterMut<'_, f32>;
}

// TODO: Come up with a clearer name for this
impl<'a> FramedSampleTrait<'a> for &'a mut [f32] {
    fn samples(&mut self) -> IterMut<'_, f32> {
        self.iter_mut()
    }
}

pub trait FramedSamplesTrait<'a> {
    fn samples(&mut self) -> Zip<IterMut<'_, f32>, IterMut<'_, f32>>;
}

impl<'a> FramedSamplesTrait<'a> for (&'a mut [f32], &'a mut [f32]) {
    fn samples(&mut self) -> Zip<IterMut<'_, f32>, IterMut<'_, f32>> {
        self.0.iter_mut().zip(self.1.iter_mut())
    }
}
