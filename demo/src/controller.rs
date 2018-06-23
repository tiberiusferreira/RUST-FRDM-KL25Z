#[derive(Clone, Debug)]
pub struct Controller{
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub accumulated_error: i32,
    pub last_error: i32,
    pub last_output: u8,
}

impl Controller{
    pub fn new() -> Controller{
        Controller{
            kp: 0.0,
            ki: 0.0,
            kd: 0.0,
            accumulated_error: 0,
            last_error: 0,
            last_output: 0,
        }
    }
    pub fn tick(&mut self, target_value: u32, current_value: u32) -> u8{
        let error = target_value as i32 - current_value as i32;


        // Kp
        let kp_contrib = self.kp*(error as f32);

        // Ki
        if self.ki > 0.0 {
            self.accumulated_error = self.accumulated_error + error;
        }else{
            self.accumulated_error = 0;
        }
        let ki_contrib = self.ki* (self.accumulated_error as f32);

        // Kd
        let kd_contrib = self.kd* ((error - self.last_error) as f32);

        self.last_error = error;

        let output;
        if (kp_contrib+ki_contrib+kd_contrib) > 100.0{
            output = 100;
        }else if (kp_contrib+ki_contrib+kd_contrib) < 0.0{
            output = 0
        }else {
            output = (kp_contrib+ki_contrib+kd_contrib) as u8;
        }
        self.last_output = output;
        return output
    }
}