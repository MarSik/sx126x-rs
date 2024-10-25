#[cfg(feature = "async")]
use embedded_hal_async::digital::Wait;
#[cfg(not(feature = "async"))]
use embedded_hal::digital::InputPin;

pub trait AnyWait {
    async fn wait_for_high(&mut self);
    async fn wait_for_low(&mut self);
}

#[cfg(not(feature = "async"))]
impl <T:InputPin> AnyWait for T {
    async fn wait_for_high(&mut self) {
        while let Ok(false) = self.is_high() {
            // Busy loop
        }
    }

    async fn wait_for_low(&mut self) {
        while let Ok(false) = self.is_low() {
            // Busy loop
        }
    }
}

#[cfg(feature = "async")]
impl <T:Wait> AnyWait for T {
    async fn wait_for_high(&mut self) {
        self.wait_for_high().await;
    }

    async fn wait_for_low(&mut self) {
        self.wait_for_low().await;
    }
}
