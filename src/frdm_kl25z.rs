use io::VolatileRW;
use system_integration_module::SystemIntegrationModule;

pub struct FrdmKl25zBoard{

}

pub trait FrdmKl25z{
    fn init() -> FrdmKl25zBoard;
    fn disable_watchdog_timer(&self);
}

impl FrdmKl25z for FrdmKl25zBoard{
    fn init() -> FrdmKl25zBoard{
        FrdmKl25zBoard{}
    }
    fn disable_watchdog_timer(&self){
//        let sim = SystemIntegrationModule::get();
        // Disable Watchdog
        SystemIntegrationModule::disable_watchdog_timer();
    }
}
