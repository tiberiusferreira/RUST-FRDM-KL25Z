pub use switch::*;
pub use led::*;
pub use display::*;
pub use frdm_kl25z::*;
pub use Es670Board;
pub use frdm_kl25z::Value::{High, Low};


const CMD_INIT_LCD :u8 = 0x0F;
const CMD_CLEAR :u8 = 0x01;
const CMD_NO_CURSOR :u8 = 0x0C;
const CMD_CURSOR2R :u8 = 0x06; /* cursor to right */
const CMD_NO_CUR_NO_BLINK :u8= 0x38; /* no cursor, no blink */


/* line and columns */
const L0C0_BASE :u8 = 0x80 ;/* line 0, column 0 */
const L1C0_BASE :u8 = 0xC0 ;/* line 0, column 0 */
const MAX_COLUMN :u8 = 15 ;/* line 0, column 0 */

pub struct LcdPins {
    rs: Gpio,
    enable: Gpio,
    db0: Gpio,
    db1: Gpio,
    db2: Gpio,
    db3: Gpio,
    db4: Gpio,
    db5: Gpio,
    db6: Gpio,
    db7: Gpio,
}

pub enum BitPositionsU8 {
    Bit0 = 0,
    Bit1 = 1,
    Bit2 = 2,
    Bit3 = 3,
    Bit4 = 4,
    Bit5 = 5,
    Bit6 = 6,
    Bit7 = 7
}

impl Es670Board{

    pub fn lcd_clear(&self) -> LcdPins {
        let lcd = self.lcd_init_pins();

        // turn-on LCD, with no cursor and no blink
        self.write_to_lcd(CMD_NO_CUR_NO_BLINK, true);

        // init LCD
        self.write_to_lcd(CMD_INIT_LCD, true);
        self.write_to_lcd(CMD_CLEAR, true);
        self.write_to_lcd(CMD_NO_CURSOR, true);
        self.write_to_lcd(CMD_CURSOR2R, true);
        lcd
    }
    fn lcd_init_pins(&self) -> LcdPins {

        let rs = self.get_gpio(PortLetter::PortC, Pin::Pin8);
        let enable = self.get_gpio(PortLetter::PortC, Pin::Pin9);
        enable.set_direction(Direction::Out);
        let db0 = self.get_gpio(PortLetter::PortC, Pin::Pin0);
        db0.set_direction(Direction::Out);
        let db1 = self.get_gpio(PortLetter::PortC, Pin::Pin1);
        db1.set_direction(Direction::Out);
        let db2 = self.get_gpio(PortLetter::PortC, Pin::Pin2);
        db2.set_direction(Direction::Out);
        let db3 = self.get_gpio(PortLetter::PortC, Pin::Pin3);
        db3.set_direction(Direction::Out);
        let db4 = self.get_gpio(PortLetter::PortC, Pin::Pin4);
        db4.set_direction(Direction::Out);
        let db5 = self.get_gpio(PortLetter::PortC, Pin::Pin5);
        db5.set_direction(Direction::Out);
        let db6 = self.get_gpio(PortLetter::PortC, Pin::Pin6);
        db6.set_direction(Direction::Out);
        let db7 = self.get_gpio(PortLetter::PortC, Pin::Pin7);
        db7.set_direction(Direction::Out);
        LcdPins {
            rs,
            enable,
            db0,
            db1,
            db2,
            db3,
            db4,
            db5,
            db6,
            db7,
        }
    }

    pub fn lcd_set_cursor(&self, line: u8, col_left_to_right: u8) {
        let mut c_command;

        if line == 0 {
            c_command = L0C0_BASE;
        } else{
            c_command = L1C0_BASE;
        }
        /* maximum MAX_COLUMN columns */
        c_command += col_left_to_right & MAX_COLUMN;

        // send the command to set the cursor
        self.write_to_lcd(c_command, true);
    }


    pub fn write_char(&self, c: char){
        self.write_to_lcd(c as u8, false);
    }

    pub fn write_string(&self, string: &str){
        for c in string.chars(){
            self.write_to_lcd(c as u8, false);
        }
    }

    fn get_bit_at(data: u8, at: BitPositionsU8) -> bool{
        return (data & (1 << at as u8)) != 0;
    }

    pub fn write_to_lcd(&self, data: u8, is_cmd: bool){
        let lcd = self.lcd_init_pins();
        if is_cmd{
            lcd.rs.set_value(Low);
        }else{
            lcd.rs.set_value(High);
        }

        lcd.db0.set_value(Self::get_bit_at(data, BitPositionsU8::Bit0).into());
        lcd.db1.set_value(Self::get_bit_at(data, BitPositionsU8::Bit1).into());
        lcd.db2.set_value(Self::get_bit_at(data, BitPositionsU8::Bit2).into());
        lcd.db3.set_value(Self::get_bit_at(data, BitPositionsU8::Bit3).into());
        lcd.db4.set_value(Self::get_bit_at(data, BitPositionsU8::Bit4).into());
        lcd.db5.set_value(Self::get_bit_at(data, BitPositionsU8::Bit5).into());
        lcd.db6.set_value(Self::get_bit_at(data, BitPositionsU8::Bit6).into());
        lcd.db7.set_value(Self::get_bit_at(data, BitPositionsU8::Bit7).into());

        lcd.enable.set_value(High);
        self.delay(1);
        lcd.enable.set_value(Low);
        self.delay(2);

    }

}
