use std::{marker::PhantomData, time::Instant};

pub struct Vehicle<S: VehicleState> {
  state: InnerVehicleState,
  _marker: PhantomData<S>,
}

pub trait VehicleState {}

#[derive(Default)]
struct InnerVehicleState {
  speed: u32,
  aesthetics: CarAesthetics,
  running_time: Option<Instant>,
  distance: u64,
}

pub enum Start {}
pub enum Stop {}
pub enum Running {}

impl VehicleState for Start {}
impl VehicleState for Stop {}
impl VehicleState for Running {}

#[derive(Copy, Clone, Default)]
pub enum CarAesthetics {
  #[default]
  Clean,
  Dirty,
}

impl Vehicle<Stop> {
  pub fn start(mut self) -> Vehicle<Running> {
    self.state.running_time = Some(Instant::now());
    Vehicle {
      state: self.state,
      _marker: PhantomData,
    }
  }

  pub fn wash(&mut self) {
    self.state.aesthetics = CarAesthetics::Clean;
  }
}

impl Vehicle<Running> {
  pub fn stop(mut self) -> Vehicle<Stop> {
    self.state.speed = 0;
    Vehicle {
      state: self.state,
      _marker: PhantomData,
    }
  }

  fn update_distance(&mut self) {
    if let Some(time_at_speed) = self.state.running_time.replace(Instant::now())
    {
      self.state.distance +=
        time_at_speed.elapsed().as_secs() * self.state.speed as u64;
    }
  }

  pub fn accelerate(&mut self, speed: u32) {
    self.update_distance();
    self.state.speed += speed;
    if self.state.speed > 20 {
      self.state.aesthetics = CarAesthetics::Dirty
    }
  }

  pub fn decelerate(&mut self, speed: u32) {
    self.update_distance();
    self.state.speed -= speed
  }
}

impl<S: VehicleState> Vehicle<S> {
  pub fn speed(&self) -> u32 {
    self.state.speed
  }

  pub fn aesthetics(&self) -> CarAesthetics {
    self.state.aesthetics
  }

  pub fn distance(&self) -> u64 {
    self.state.distance / 60
  }
}

pub fn new_vehicle() -> Vehicle<Stop> {
  Vehicle {
    state: InnerVehicleState::default(),
    _marker: PhantomData,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_vehicle_state() {
    let mut car = new_vehicle();

    assert_eq!(car.speed(), 0);

    car.wash();

    let mut car = car.start();

    for _ in 1..5 {
      car.accelerate(10);
    }

    assert_eq!(car.speed(), 40);
    car.decelerate(20);
    assert_eq!(car.speed(), 20);
    assert!(matches!(car.aesthetics(), CarAesthetics::Dirty));

    let car = car.stop();

    assert_eq!(car.speed(), 0);
    println!("distance traveled: {}", car.distance());
  }
}
