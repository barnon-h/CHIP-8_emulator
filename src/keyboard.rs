use winit::keyboard::KeyCode;

// Map 16 keys
pub fn map_key( key : KeyCode ) -> Option< usize >
{
    match key {
        KeyCode::Digit1 => Some( 0x1 ),
        KeyCode::Digit2 => Some( 0x2 ),
        KeyCode::Digit3 => Some( 0x3 ),
        KeyCode::Digit4 => Some( 0xC ),

        KeyCode::KeyQ   => Some( 0x4 ),
        KeyCode::KeyW   => Some( 0x5 ),
        KeyCode::KeyE   => Some( 0x6 ),
        KeyCode::KeyR   => Some( 0xD ),

        KeyCode::KeyA   => Some( 0x7 ),
        KeyCode::KeyS   => Some( 0x8 ),
        KeyCode::KeyD   => Some( 0x9 ),
        KeyCode::KeyF   => Some( 0xE ),

        KeyCode::KeyZ   => Some( 0xA ),
        KeyCode::KeyX   => Some( 0x0 ),
        KeyCode::KeyC   => Some( 0xB ),
        KeyCode::KeyV   => Some( 0xF ),

        _ => None,
    }
}